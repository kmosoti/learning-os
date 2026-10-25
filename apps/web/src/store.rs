//! Browser composition root. A bounded journal and its receipts commit in one strict transaction.
use crate::Draft;
use js_sys::{Function, Promise};
use learningos_application::*;
use learningos_domain::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, prelude::*};
use wasm_bindgen_futures::JsFuture;
use web_sys::*;

pub const DATABASE: &str = "learningos-v2-evidence-1";
fn error(e: impl ToString) -> AppError {
    AppError::Storage(e.to_string())
}
fn js_error(e: JsValue) -> AppError {
    if let Some(exception) = e.dyn_ref::<DomException>() {
        match exception.name().as_str() {
            "VersionError" => {
                return error(
                    "This journal uses a newer storage schema. Open the newer application to export or close other tabs and retry; no migration or alternate store was opened",
                );
            }
            "QuotaExceededError" => {
                return error(
                    "QuotaExceededError: browser storage quota exhausted. The save is not acknowledged; retain this tab's draft, export existing evidence, then free space and retry",
                );
            }
            _ => {}
        }
    }
    error(e.as_string().unwrap_or_else(|| format!("{e:?}")))
}
async fn request(r: &IdbRequest) -> Result<JsValue, AppError> {
    let mut handlers = None;
    let p = Promise::new(&mut |resolve: Function, reject: Function| {
        let rr = r.clone();
        let reject_result = reject.clone();
        let ok = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| match rr.result() {
            Ok(value) => {
                let _ = resolve.call1(&JsValue::NULL, &value);
            }
            Err(value) => {
                let _ = reject_result.call1(&JsValue::NULL, &value);
            }
        });
        let rr = r.clone();
        let fail_reject = reject.clone();
        let fail = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
            let value = rr
                .error()
                .ok()
                .flatten()
                .map(JsValue::from)
                .unwrap_or_else(|| JsValue::from_str("IndexedDB request failed"));
            let _ = fail_reject.call1(&JsValue::NULL, &value);
        });
        r.set_onsuccess(Some(ok.as_ref().unchecked_ref()));
        r.set_onerror(Some(fail.as_ref().unchecked_ref()));
        let blocked = r.dyn_ref::<IdbOpenDbRequest>().map(|open| { let reject=reject.clone(); let handler=Closure::<dyn FnMut(web_sys::Event)>::new(move |_| { let _=reject.call1(&JsValue::NULL,&JsValue::from_str("Storage upgrade is blocked by another tab. Close other LearningOS tabs, export there if needed, then retry. No alternate store was opened.")); }); open.set_onblocked(Some(handler.as_ref().unchecked_ref())); handler });
        handlers = Some((ok, fail, blocked));
    });
    let result = JsFuture::from(p).await.map_err(js_error);
    r.set_onsuccess(None);
    r.set_onerror(None);
    if let Some(open) = r.dyn_ref::<IdbOpenDbRequest>() {
        open.set_onblocked(None);
    }
    drop(handlers);
    result
}
struct Completion {
    promise: Promise,
    tx: IdbTransaction,
    _ok: Closure<dyn FnMut(web_sys::Event)>,
    _fail: Closure<dyn FnMut(web_sys::Event)>,
}
impl Completion {
    fn new(tx: &IdbTransaction) -> Self {
        let mut handlers = None;
        let promise = Promise::new(&mut |resolve: Function, reject: Function| {
            let ok = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
                let _ = resolve.call0(&JsValue::NULL);
            });
            let t = tx.clone();
            let fail = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
                let value = t
                    .error()
                    .map(JsValue::from)
                    .unwrap_or_else(|| JsValue::from_str("transaction aborted"));
                let _ = reject.call1(&JsValue::NULL, &value);
            });
            tx.set_oncomplete(Some(ok.as_ref().unchecked_ref()));
            tx.set_onabort(Some(fail.as_ref().unchecked_ref()));
            handlers = Some((ok, fail));
        });
        let (ok, fail) = handlers.expect("synchronous promise executor");
        Self {
            promise,
            tx: tx.clone(),
            _ok: ok,
            _fail: fail,
        }
    }
    async fn wait(self) -> Result<(), AppError> {
        let result = JsFuture::from(self.promise.clone()).await.map_err(js_error);
        self.tx.set_oncomplete(None);
        self.tx.set_onabort(None);
        result.map(|_| ())
    }
}
pub struct BrowserJournal {
    db: IdbDatabase,
    _version: Closure<dyn FnMut(web_sys::Event)>,
}
pub fn empty_backup() -> Backup {
    Backup {
        schema: 1,
        fixture: FIXTURE_VERSION.into(),
        fixture_digest: fixture_digest(),
        events: vec![],
        receipts: vec![],
        event_digest: digest("[]"),
    }
}
impl BrowserJournal {
    pub async fn open() -> Result<Self, AppError> {
        let factory = window()
            .ok_or_else(|| error("window unavailable"))?
            .indexed_db()
            .map_err(js_error)?
            .ok_or_else(|| error("IndexedDB unavailable; no substitute store was opened"))?;
        let r = factory.open_with_u32(DATABASE, 1).map_err(js_error)?;
        let rr = r.clone();
        let upgrade = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
            if let Ok(value) = rr.result() {
                let db: IdbDatabase = value.unchecked_into();
                if db.create_object_store("local").is_err()
                    && let Some(tx) = rr.transaction()
                {
                    let _ = tx.abort();
                }
            }
        });
        r.set_onupgradeneeded(Some(upgrade.as_ref().unchecked_ref()));
        let value = request(&r).await;
        r.set_onupgradeneeded(None);
        drop(upgrade);
        let db: IdbDatabase = value?.dyn_into().map_err(js_error)?;
        let copy = db.clone();
        let version = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| copy.close());
        db.set_onversionchange(Some(version.as_ref().unchecked_ref()));
        Ok(Self {
            db,
            _version: version,
        })
    }
    fn transaction(&self) -> Result<IdbTransaction, AppError> {
        let options = IdbTransactionOptions::new();
        options.set_durability(IdbTransactionDurability::Strict);
        self.db
            .transaction_with_str_and_mode_and_options(
                "local",
                IdbTransactionMode::Readwrite,
                &options,
            )
            .map_err(js_error)
    }
    async fn get<T: for<'a> Deserialize<'a>>(&self, key: &str) -> Result<Option<T>, AppError> {
        let tx = self.db.transaction_with_str("local").map_err(js_error)?;
        let store = tx.object_store("local").map_err(js_error)?;
        read(&store, key).await
    }
    pub async fn backup(&self) -> Result<Backup, AppError> {
        let backup: Backup = self.get("journal").await?.unwrap_or_else(empty_backup);
        backup.validate()?;
        Ok(backup)
    }
    async fn archive(&self) -> Result<Backup, AppError> {
        Ok(self.get("journal").await?.unwrap_or_else(empty_backup))
    }
    pub async fn verified_snapshot(&self) -> Result<(Backup, Session), AppError> {
        let tx = self.db.transaction_with_str("local").map_err(js_error)?;
        let store = tx.object_store("local").map_err(js_error)?;
        let backup: Backup = read(&store, "journal").await?.unwrap_or_else(empty_backup);
        let snapshot: Session = read(&store, "projection").await?.unwrap_or_default();
        let replayed = backup.validate()?;
        if replayed != snapshot {
            return Err(error(
                "Snapshot differs from the journal in the same read transaction. Export for explicit recovery; nothing was overwritten",
            ));
        }
        Ok((backup, replayed))
    }
    pub async fn draft(&self) -> Result<Draft, AppError> {
        let key = tab_key()?;
        if let Some(draft) = self.get(&key).await? {
            return Ok(draft);
        }
        let tx = self.db.transaction_with_str("local").map_err(js_error)?;
        let store = tx.object_store("local").map_err(js_error)?;
        let range = IdbKeyRange::bound(&JsValue::from_str("draft/"), &JsValue::from_str("draft0"))
            .map_err(js_error)?;
        let values = request(&store.get_all_with_key(&range).map_err(js_error)?).await?;
        let drafts: Vec<Draft> = js_sys::Array::from(&values)
            .iter()
            .map(|value| {
                serde_json::from_str(&value.as_string().ok_or_else(|| error("damaged draft"))?)
                    .map_err(error)
            })
            .collect::<Result<_, _>>()?;
        Ok(drafts
            .into_iter()
            .max_by_key(|d| d.updated_at_ms)
            .unwrap_or_default())
    }
    pub async fn save_draft(&self, draft: &Draft) -> Result<(), AppError> {
        draft.validate()?;
        let key = tab_key()?;
        let tx = self.transaction()?;
        let done = Completion::new(&tx);
        let result=async{let store=tx.object_store("local").map_err(js_error)?;let old:Option<Draft>=read(&store,&key).await?;if let Some(old)=old{if old.generation>draft.generation{return Ok(());}else if old.generation==draft.generation{return if old==*draft{Ok(())}else{Err(error("Draft generation conflict. Another duplicated tab may share this tab identity; your current draft is kept"))};}}else{let range=IdbKeyRange::bound(&JsValue::from_str("draft/"),&JsValue::from_str("draft0")).map_err(js_error)?;let keys=request(&store.get_all_keys_with_key(&range).map_err(js_error)?).await?;if js_sys::Array::from(&keys).length()>=64{return Err(error("64 retained tab drafts reached. Export evidence and explicitly clear local data before creating more tab drafts"));}}put(&store,&key,draft)?;Ok(())}.await;
        match result {
            Ok(()) => done.wait().await,
            Err(e) => {
                let _ = tx.abort();
                let _ = done.wait().await;
                Err(e)
            }
        }
    }
    pub async fn replace(&self, backup: &Backup) -> Result<(), AppError> {
        let projection = backup.validate()?;
        let tx = self.transaction()?;
        let done = Completion::new(&tx);
        let result = async {
            let store = tx.object_store("local").map_err(js_error)?;
            let current: Backup = read(&store, "journal").await?.unwrap_or_else(empty_backup);
            if !current.events.is_empty() {
                return Err(AppError::Backup("import requires an empty journal".into()));
            }
            put(&store, "journal", backup)?;
            put(&store, "projection", &projection)?;
            put(&store, &tab_key()?, &Draft::default())?;
            Ok(())
        }
        .await;
        match result {
            Ok(()) => done.wait().await,
            Err(e) => {
                let _ = tx.abort();
                let _ = done.wait().await;
                Err(e)
            }
        }
    }
    pub async fn clear(&self) -> Result<(), AppError> {
        let tx = self.transaction()?;
        let done = Completion::new(&tx);
        tx.object_store("local")
            .map_err(js_error)?
            .clear()
            .map_err(js_error)?;
        done.wait().await
    }
}
async fn read<T: for<'a> Deserialize<'a>>(
    store: &IdbObjectStore,
    key: &str,
) -> Result<Option<T>, AppError> {
    let value = request(&store.get(&JsValue::from_str(key)).map_err(js_error)?).await?;
    if value.is_undefined() {
        return Ok(None);
    }
    let text = value.as_string().ok_or_else(|| {
        error(format!(
            "Damaged local record {key}: expected a JSON string. No empty substitute was opened"
        ))
    })?;
    Ok(Some(serde_json::from_str(&text).map_err(error)?))
}
fn put<T: Serialize>(store: &IdbObjectStore, key: &str, value: &T) -> Result<(), AppError> {
    let encoded = serde_json::to_string(value).map_err(error)?;
    if encoded.len() > 8 * 1024 * 1024 {
        return Err(error("8 MiB local record limit"));
    }
    store
        .put_with_key(&JsValue::from_str(&encoded), &JsValue::from_str(key))
        .map_err(js_error)?;
    Ok(())
}
impl Journal for BrowserJournal {
    async fn events(&self) -> Result<Vec<learningos_domain::Event>, AppError> {
        Ok(self.backup().await?.events)
    }
    async fn session(&self) -> Result<Session, AppError> {
        Ok(self.get("projection").await?.unwrap_or_default())
    }
    async fn receipt(&self, id: &str) -> Result<Option<Receipt>, AppError> {
        Ok(self
            .archive()
            .await?
            .receipts
            .into_iter()
            .find(|r| r.request.id == id))
    }
    async fn commit(&self, commit: &Commit) -> Result<Receipt, AppError> {
        let tx = self.transaction()?;
        let done = Completion::new(&tx);
        let result = async {
            let store = tx.object_store("local").map_err(js_error)?;
            let mut backup: Backup = read(&store, "journal").await?.unwrap_or_else(empty_backup);
            if let Some(receipt) = backup
                .receipts
                .iter()
                .find(|r| r.request.id == commit.request.id)
            {
                return if receipt.request == commit.request {
                    Ok(receipt.clone())
                } else {
                    Err(AppError::Conflict)
                };
            }
            let old: Session = read(&store, "projection").await?.unwrap_or_default();
            if backup.events.len() as u64!=old.revision || backup.receipts.last().map_or(0,|r|r.revision)!=old.revision || backup.fixture_digest!=fixture_digest() || backup.event_digest!=digest(&serde_json::to_string(&backup.events).map_err(error)?){return Err(error("Journal/snapshot identity or digest mismatch; export for explicit recovery before another write"));}
            if old.revision != commit.request.expected {
                return Err(AppError::Stale);
            }
            let expected = decide(
                &old,
                &commit.request.command,
                &commit.request.id,
                commit.request.at_ms,
            )?;
            if expected != commit.events {
                return Err(error("adapter rejected unbound event batch"));
            }
            backup.events.extend(commit.events.clone());
            backup.receipts.push(commit.receipt.clone());
            backup.event_digest = digest(&serde_json::to_string(&backup.events).map_err(error)?);
            let mut next = old;
            for event in &expected {
                next = apply(&next, event)?;
            }
            if next != commit.projection {
                return Err(error("projection does not match journal"));
            }
            put(&store, "projection", &next)?;
            put(&store, "journal", &backup)?;
            Ok(commit.receipt.clone())
        }
        .await;
        match result {
            Ok(receipt) => {
                done.wait().await?;
                Ok(receipt)
            }
            Err(e) => {
                let _ = tx.abort();
                let _ = done.wait().await;
                Err(e)
            }
        }
    }
}
impl Drop for Completion {
    fn drop(&mut self) {
        self.tx.set_oncomplete(None);
        self.tx.set_onabort(None);
    }
}
impl Drop for BrowserJournal {
    fn drop(&mut self) {
        self.db.set_onversionchange(None);
        self.db.close();
    }
}

fn tab_key() -> Result<String, AppError> {
    let window = window().ok_or_else(|| error("window unavailable"))?;
    let storage = window.session_storage().map_err(js_error)?.ok_or_else(|| {
        error("Tab identity storage unavailable; no shared draft key was substituted")
    })?;
    let id = match storage.get_item("learningos-v2-tab-id").map_err(js_error)? {
        Some(id) => id,
        None => {
            let id = window.crypto().map_err(js_error)?.random_uuid();
            storage
                .set_item("learningos-v2-tab-id", &id)
                .map_err(js_error)?;
            id
        }
    };
    Ok(format!("draft/{id}"))
}

// sessionStorage survives reload but is cloned by "Duplicate tab". A live peer
// claiming the same ID makes this document choose a fresh draft namespace.
type TabClaim = (BroadcastChannel, Closure<dyn FnMut(MessageEvent)>);
thread_local! { static TAB_CHANNEL:std::cell::RefCell<Option<TabClaim>>=const{std::cell::RefCell::new(None)}; }
pub async fn prepare_tab_identity() -> Result<(), AppError> {
    if TAB_CHANNEL.with(|c| c.borrow().is_some()) {
        return Ok(());
    }
    let window = window().ok_or_else(|| error("window unavailable"))?;
    let storage = window
        .session_storage()
        .map_err(js_error)?
        .ok_or_else(|| error("tab identity storage unavailable"))?;
    let initial = tab_key()?.trim_start_matches("draft/").to_string();
    let identity = std::rc::Rc::new(std::cell::RefCell::new(initial.clone()));
    let token = window.crypto().map_err(js_error)?.random_uuid();
    let channel = BroadcastChannel::new("learningos-v2-tab-claims").map_err(js_error)?;
    let occupied = std::rc::Rc::new(std::cell::Cell::new(false));
    let peer = channel.clone();
    let own_id = identity.clone();
    let own_token = token.clone();
    let collision = occupied.clone();
    let handler = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
        let Some(json) = event.data().as_string() else {
            return;
        };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&json) else {
            return;
        };
        if value["id"].as_str() != Some(own_id.borrow().as_str()) {
            return;
        }
        if value["kind"] == "probe" && value["token"].as_str() != Some(own_token.as_str()) {
            let reply =
                serde_json::json!({"kind":"occupied","id":*own_id.borrow(),"token":value["token"]})
                    .to_string();
            let _ = peer.post_message(&JsValue::from_str(&reply));
        } else if value["kind"] == "occupied" && value["token"].as_str() == Some(own_token.as_str())
        {
            collision.set(true);
        }
    });
    channel.set_onmessage(Some(handler.as_ref().unchecked_ref()));
    channel
        .post_message(&JsValue::from_str(
            &serde_json::json!({"kind":"probe","id":initial,"token":token}).to_string(),
        ))
        .map_err(js_error)?;
    let mut timeout = None;
    let timer = Promise::new(&mut |resolve: Function, _reject: Function| {
        let done = Closure::<dyn FnMut()>::new(move || {
            let _ = resolve.call0(&JsValue::NULL);
        });
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            done.as_ref().unchecked_ref(),
            40,
        );
        timeout = Some(done);
    });
    JsFuture::from(timer).await.map_err(js_error)?;
    drop(timeout);
    if occupied.get() {
        let fresh = window.crypto().map_err(js_error)?.random_uuid();
        storage
            .set_item("learningos-v2-tab-id", &fresh)
            .map_err(js_error)?;
        *identity.borrow_mut() = fresh;
    }
    TAB_CHANNEL.with(|slot| *slot.borrow_mut() = Some((channel, handler)));
    Ok(())
}
