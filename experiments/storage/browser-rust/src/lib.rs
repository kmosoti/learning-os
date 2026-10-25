//! The experiment uses Rust, not a JavaScript IndexedDB application implementation.
use js_sys::{Function, Promise};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, prelude::*};
use wasm_bindgen_futures::JsFuture;
use web_sys::*;

fn error(s: impl ToString) -> JsValue {
    JsValue::from_str(&s.to_string())
}
async fn request(r: &IdbRequest) -> Result<JsValue, JsValue> {
    let mut handlers = None;
    let p = Promise::new(&mut |resolve: Function, reject: Function| {
        let rr = r.clone();
        let result_reject = reject.clone();
        let ok = Closure::<dyn FnMut(Event)>::new(move |_| match rr.result() {
            Ok(v) => {
                let _ = resolve.call1(&JsValue::NULL, &v);
            }
            Err(e) => {
                let _ = result_reject.call1(&JsValue::NULL, &e);
            }
        });
        let rr = r.clone();
        let fail = Closure::<dyn FnMut(Event)>::new(move |_| {
            let e = rr
                .error()
                .ok()
                .flatten()
                .map(JsValue::from)
                .unwrap_or_else(|| error("request failed"));
            let _ = reject.call1(&JsValue::NULL, &e);
        });
        r.set_onsuccess(Some(ok.as_ref().unchecked_ref()));
        r.set_onerror(Some(fail.as_ref().unchecked_ref()));
        handlers = Some((ok, fail));
    });
    let result = JsFuture::from(p).await;
    r.set_onsuccess(None);
    r.set_onerror(None);
    drop(handlers);
    result
}
struct Completion {
    promise: Promise,
    tx: IdbTransaction,
    _ok: Closure<dyn FnMut(Event)>,
    _fail: Closure<dyn FnMut(Event)>,
}
impl Completion {
    fn new(tx: &IdbTransaction) -> Self {
        let mut handlers = None;
        let promise = Promise::new(&mut |resolve: Function, reject: Function| {
            let ok = Closure::<dyn FnMut(Event)>::new(move |_| {
                let _ = resolve.call0(&JsValue::NULL);
            });
            let t = tx.clone();
            let fail = Closure::<dyn FnMut(Event)>::new(move |_| {
                let e = t
                    .error()
                    .map(JsValue::from)
                    .unwrap_or_else(|| error("transaction aborted; no write acknowledged"));
                let _ = reject.call1(&JsValue::NULL, &e);
            });
            tx.set_oncomplete(Some(ok.as_ref().unchecked_ref()));
            tx.set_onabort(Some(fail.as_ref().unchecked_ref()));
            handlers = Some((ok, fail));
        });
        let (ok, fail) = handlers.expect("Promise executor is synchronous");
        Self {
            promise,
            tx: tx.clone(),
            _ok: ok,
            _fail: fail,
        }
    }
    async fn wait(self) -> Result<(), JsValue> {
        let result = JsFuture::from(self.promise.clone()).await;
        self.tx.set_oncomplete(None);
        self.tx.set_onabort(None);
        result.map(|_| ())
    }
}
#[derive(Serialize, Deserialize, Clone)]
struct Operation {
    key: String,
    expected: u64,
    events: Vec<String>,
    projection: String,
}
#[derive(Serialize, Deserialize)]
struct State {
    revision: u64,
    projection: String,
}
#[derive(Serialize, Deserialize)]
struct Receipt {
    request: String,
    revision: u64,
}
#[wasm_bindgen]
pub struct BrowserJournal {
    db: IdbDatabase,
    version_change: Closure<dyn FnMut(Event)>,
}
#[wasm_bindgen]
impl BrowserJournal {
    pub async fn open(name: &str) -> Result<BrowserJournal, JsValue> {
        let factory = web_sys::window()
            .ok_or_else(|| error("window unavailable"))?
            .indexed_db()?
            .ok_or_else(|| error("IndexedDB unavailable"))?;
        let r = factory.open_with_u32(name, 1)?;
        let rr = r.clone();
        let upgrade = Closure::<dyn FnMut(Event)>::new(move |_| {
            if let Ok(v) = rr.result() {
                let db: IdbDatabase = v.unchecked_into();
                if !db.object_store_names().contains("journal")
                    && db.create_object_store("journal").is_err()
                    && let Some(tx) = rr.transaction()
                {
                    let _ = tx.abort();
                }
            }
        });
        r.set_onupgradeneeded(Some(upgrade.as_ref().unchecked_ref()));
        let v = request(&r).await;
        r.set_onupgradeneeded(None);
        drop(upgrade);
        let db: IdbDatabase = v?.dyn_into()?;
        let copy = db.clone();
        let version_change = Closure::<dyn FnMut(Event)>::new(move |_| copy.close());
        db.set_onversionchange(Some(version_change.as_ref().unchecked_ref()));
        Ok(Self { db, version_change })
    }
    pub async fn append(&self, json: &str, abort: bool) -> Result<u64, JsValue> {
        let op: Operation = serde_json::from_str(json).map_err(error)?;
        if op.key.is_empty()
            || op.key.len() > 160
            || op.events.is_empty()
            || op.events.len() > 100
            || op.events.iter().any(|e| e.len() > 65536)
        {
            return Err(error("invalid bounded journal operation"));
        }
        let options = IdbTransactionOptions::new();
        options.set_durability(IdbTransactionDurability::Strict);
        let tx = self.db.transaction_with_str_and_mode_and_options(
            "journal",
            IdbTransactionMode::Readwrite,
            &options,
        )?;
        let done = Completion::new(&tx);
        let result = Self::write(&tx, &op, abort).await;
        match result {
            Ok(v) => {
                done.wait().await?;
                Ok(v)
            }
            Err(e) => {
                let _ = tx.abort();
                let _ = done.wait().await;
                Err(e)
            }
        }
    }
    pub async fn state(&self) -> Result<String, JsValue> {
        let tx = self.db.transaction_with_str("journal")?;
        let store = tx.object_store("journal")?;
        let value = request(&store.get(&JsValue::from_str("state"))?).await?;
        Ok(value
            .as_string()
            .unwrap_or_else(|| "{\"revision\":0,\"projection\":\"{}\"}".into()))
    }
    pub async fn events(&self) -> Result<String, JsValue> {
        let tx = self.db.transaction_with_str("journal")?;
        let store = tx.object_store("journal")?;
        let range = IdbKeyRange::bound(&JsValue::from_str("e/"), &JsValue::from_str("e0"))?;
        let values = request(&store.get_all_with_key(&range)?).await?;
        let events: Vec<String> = js_sys::Array::from(&values)
            .iter()
            .map(|v| v.as_string().ok_or_else(|| error("corrupt event")))
            .collect::<Result<_, _>>()?;
        serde_json::to_string(&events).map_err(error)
    }
    pub fn close(&self) {
        self.db.close();
    }
}
impl BrowserJournal {
    async fn write(tx: &IdbTransaction, op: &Operation, abort: bool) -> Result<u64, JsValue> {
        let store = tx.object_store("journal")?;
        let canonical = serde_json::to_string(op).map_err(error)?;
        let key = JsValue::from_str(&format!("op/{}", op.key));
        let duplicate = request(&store.get(&key)?).await?;
        if let Some(s) = duplicate.as_string() {
            let r: Receipt = serde_json::from_str(&s).map_err(error)?;
            return if r.request == canonical {
                Ok(r.revision)
            } else {
                Err(error("operation conflict"))
            };
        }
        let old = request(&store.get(&JsValue::from_str("state"))?).await?;
        let state: State = serde_json::from_str(
            &old.as_string()
                .unwrap_or_else(|| "{\"revision\":0,\"projection\":\"{}\"}".into()),
        )
        .map_err(error)?;
        if state.revision != op.expected {
            return Err(error("stale revision; draft preserved"));
        }
        let next = state
            .revision
            .checked_add(op.events.len() as u64)
            .ok_or_else(|| error("revision overflow"))?;
        for (n, e) in op.events.iter().enumerate() {
            store.add_with_key(
                &JsValue::from_str(e),
                &JsValue::from_str(&format!("e/{:020}", state.revision + n as u64 + 1)),
            )?;
        }
        store.put_with_key(
            &JsValue::from_str(
                &serde_json::to_string(&State {
                    revision: next,
                    projection: op.projection.clone(),
                })
                .map_err(error)?,
            ),
            &JsValue::from_str("state"),
        )?;
        if abort {
            return Err(error("injected transaction abort"));
        }
        store.add_with_key(
            &JsValue::from_str(
                &serde_json::to_string(&Receipt {
                    request: canonical,
                    revision: next,
                })
                .map_err(error)?,
            ),
            &key,
        )?;
        Ok(next)
    }
}
impl Drop for BrowserJournal {
    fn drop(&mut self) {
        self.db.set_onversionchange(None);
        self.db.close();
        let _ = &self.version_change;
    }
}
