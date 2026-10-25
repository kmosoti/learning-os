use super::*;
use learningos_application::{AppError, execute};
use store::BrowserJournal;
use wasm_bindgen::{JsCast, prelude::*};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlElement, KeyboardEvent};
fn failure(ui: RwSignal<Ui>, e: impl ToString) {
    ui.update(|u| {
        u.error = e.to_string();
        u.busy = false;
        u.status = "Save not acknowledged — your draft and operation ID are retained".into();
    });
}
fn spawn(task: impl std::future::Future<Output = ()> + 'static) {
    leptos::task::spawn_local(task);
}
pub fn initialize(ui: RwSignal<Ui>) {
    spawn(async move {
        match load().await {
            Ok((session, draft)) => {
                let home = session.phase == Phase::Goal;
                ui.update(|u| {
                    u.session = session;
                    u.draft = draft;
                    u.ready = true;
                    u.home = home;
                    u.status = "Local journal ready · tab drafts are retained separately".into();
                });
            }
            Err(e) => failure(ui, e),
        }
        let nav = web_sys::window().expect("window").navigator();
        let workers = nav.service_worker();
        match JsFuture::from(workers.register("/sw.js")).await {
            Ok(_) => {
                if let Ok(ready) = workers.ready() {
                    match JsFuture::from(ready).await {
                        Ok(_) => ui.update(|u| {
                            u.status =
                                "Offline lesson cached · evidence stays in this browser".into()
                        }),
                        Err(_) => ui.update(|u| {
                            u.status =
                                "Offline cache incomplete · local evidence is retained".into()
                        }),
                    }
                }
            }
            Err(_) => ui.update(|u| {
                u.status = "Offline caching unavailable · local evidence is retained".into()
            }),
        }

        if let Ok(promise) = nav.storage().persist() {
            let _ = JsFuture::from(promise).await;
        }
    });
}
async fn load() -> Result<(Session, Draft), AppError> {
    store::prepare_tab_identity().await?;
    let db = BrowserJournal::open().await?;
    let (_, state) = db.verified_snapshot().await?;
    let mut draft = db.draft().await?;
    if let Some(pending) = draft.pending.clone()
        && learningos_application::Journal::receipt(&db, &pending.id)
            .await?
            .is_some()
    {
        let consuming = consumes(&pending.command);
        execute(&db, pending).await?;
        draft = if consuming {
            Draft {
                generation: draft.generation.saturating_add(1),
                updated_at_ms: js_sys::Date::now() as u64,
                reading: draft.reading,
                ..Draft::default()
            }
        } else {
            Draft {
                generation: draft.generation.saturating_add(1),
                updated_at_ms: js_sys::Date::now() as u64,
                pending: None,
                ..draft
            }
        };
        db.save_draft(&draft).await?;
    }
    Ok((state, draft))
}
pub fn reload(ui: RwSignal<Ui>) {
    spawn(async move {
        let result = async {
            let db = BrowserJournal::open().await?;
            let (_, session) = db.verified_snapshot().await?;
            let mut draft = ui.get_untracked().draft;
            draft.pending = None;
            draft.generation = draft.generation.saturating_add(1);
            draft.updated_at_ms = js_sys::Date::now() as u64;
            db.save_draft(&draft).await?;
            Ok::<_, AppError>((session, draft))
        }
        .await;
        match result {
            Ok((session, draft)) => ui.update(|u| {
                u.session = session;
                u.draft = draft;
                u.error.clear();
                u.ready = true;
                u.status = "Fresh local state loaded; your draft is retained in this tab".into();
            }),
            Err(e) => failure(ui, e),
        }
    });
}
pub fn save_draft(ui: RwSignal<Ui>) {
    let draft = ui.get_untracked().draft;
    spawn(async move {
        let result = async {
            let db = BrowserJournal::open().await?;
            db.save_draft(&draft).await
        }
        .await;
        if let Err(e) = result {
            failure(ui, e);
        }
    });
}
fn consumes(command: &Command) -> bool {
    matches!(
        command,
        Command::Answer { .. }
            | Command::FinishReading { .. }
            | Command::Continue { .. }
            | Command::ChooseGoal { .. }
    )
}
pub fn send(ui: RwSignal<Ui>, command: Command) {
    send_internal(ui, command, false);
}
pub fn consult_source(ui: RwSignal<Ui>) {
    remember_focus();
    send_internal(ui, Command::ConsultSource, true);
}
fn send_internal(ui: RwSignal<Ui>, command: Command, open_source: bool) {
    if ui.with_untracked(|u| u.busy) {
        return;
    }
    ui.update(|u| {
        u.busy = true;
        u.error.clear();
        u.status = "Committing to your local journal…".into();
    });
    spawn(async move {
        let mut committed = None;
        let result=async{let db=BrowserJournal::open().await?;let current=ui.get_untracked();let request=match current.draft.pending.clone(){Some(p)if p.command==command=>p,Some(_)=>return Err(AppError::Storage("An earlier request is pending. Reload local state before changing it; your draft is retained".into())),None=>Request{schema:1,id:web_sys::window().expect("window").crypto().map_err(|_|AppError::Storage("secure operation IDs unavailable".into()))?.random_uuid(),expected:current.session.revision,at_ms:js_sys::Date::now()as u64,command}};let mut pending=current.draft.clone();pending.pending=Some(request.clone());pending.generation=pending.generation.saturating_add(1);pending.updated_at_ms=js_sys::Date::now()as u64;ui.update(|u|u.draft=pending.clone());db.save_draft(&pending).await?;let consuming=matches!(&request.command,Command::Answer{..}|Command::FinishReading{..}|Command::Continue{..}|Command::ChooseGoal{..});let receipt=execute(&db,request).await?;committed=Some(receipt.revision);let state=learningos_application::Journal::session(&db).await?;if state.revision<receipt.revision{return Err(AppError::Storage("receipt ahead of journal".into()));}let clean=if consuming{Draft{generation:pending.generation.saturating_add(1),updated_at_ms:js_sys::Date::now()as u64,reading:pending.reading,..Draft::default()}}else{Draft{generation:pending.generation.saturating_add(1),updated_at_ms:js_sys::Date::now()as u64,pending:None,..pending}};db.save_draft(&clean).await?;Ok((state,clean))}.await;
        match result {
            Ok((state, draft)) => ui.update(|u| {
                u.session = state;
                u.draft = draft;
                u.busy = false;
                u.status = format!("Saved locally · revision {}", u.session.revision);
                if open_source {
                    u.sheet = "source".into();
                }
            }),
            Err(e) => {
                if let Some(revision) = committed {
                    ui.update(|u|{u.busy=false;u.error=format!("Evidence committed at revision {revision}, but draft cleanup or refresh failed: {e}. Reload local state to reconcile the retained operation receipt.");u.status="Evidence committed · local refresh needs recovery".into();});
                } else {
                    failure(ui, e)
                }
            }
        }
        if open_source && ui.with_untracked(|u| u.sheet == "source") {
            focus_sheet_target();
        }
    });
}
pub fn export(ui: RwSignal<Ui>) {
    spawn(async move {
        let result = async {
            let db = BrowserJournal::open().await?;
            let backup = db.backup().await?;
            let text =
                serde_json::to_string(&backup).map_err(|e| AppError::Backup(e.to_string()))?;
            let document = web_sys::window()
                .expect("window")
                .document()
                .expect("document");
            let a = document
                .create_element("a")
                .map_err(|_| AppError::Storage("download unavailable".into()))?
                .unchecked_into::<web_sys::HtmlAnchorElement>();
            let parts = js_sys::Array::new();
            parts.push(&JsValue::from_str(&text));
            let options = web_sys::BlobPropertyBag::new();
            options.set_type("application/json");
            let blob = web_sys::Blob::new_with_str_sequence_and_options(&parts, &options)
                .map_err(|_| AppError::Storage("export unavailable".into()))?;
            let url = web_sys::Url::create_object_url_with_blob(&blob)
                .map_err(|_| AppError::Storage("download URL unavailable".into()))?;
            a.set_href(&url);
            a.set_download("learningos-evidence-v1.json");
            a.click();
            let _ = web_sys::Url::revoke_object_url(&url);
            Ok::<_, AppError>(text)
        }
        .await;
        match result {
            Ok(text) => ui.update(|u| {
                u.backup_text = text;
                u.status = "Backup exported; event and receipt identities preserved".into();
            }),
            Err(e) => failure(ui, e),
        }
    });
}
pub fn import(ui: RwSignal<Ui>) {
    let text = ui.get_untracked().backup_text;
    spawn(async move {
        let result = async {
            let backup = Backup::parse(&text)?;
            let db = BrowserJournal::open().await?;
            if !db.backup().await?.events.is_empty() {
                return Err(AppError::Backup(
                    "export and explicitly clear existing evidence before import".into(),
                ));
            }
            db.replace(&backup).await?;
            backup.validate()
        }
        .await;
        match result {
            Ok(session) => ui.update(|u| {
                u.session = session;
                u.home = false;
                u.draft = Draft::default();
                u.error.clear();
                u.status = "Backup imported and replay verified".into();
            }),
            Err(e) => failure(ui, e),
        }
    });
}
pub fn clear(ui: RwSignal<Ui>) {
    if ui.get_untracked().clear_text != "CLEAR" {
        return;
    }
    spawn(async move {
        let result = async {
            let db = BrowserJournal::open().await?;
            db.clear().await
        }
        .await;
        match result {
            Ok(()) => ui.update(|u| {
                *u = Ui {
                    ready: true,
                    status: "Local journal and draft cleared".into(),
                    ..Ui::default()
                };
            }),
            Err(e) => failure(ui, e),
        }
    });
}
thread_local! {static LAST_FOCUS:std::cell::RefCell<Option<HtmlElement>>=const{std::cell::RefCell::new(None)};}
fn remember_focus() {
    let doc = web_sys::window()
        .expect("window")
        .document()
        .expect("document");
    LAST_FOCUS.with(|p| *p.borrow_mut() = doc.active_element().and_then(|e| e.dyn_into().ok()));
}
pub fn focus_sheet() {
    remember_focus();
    focus_sheet_target();
}
fn focus_sheet_target() {
    let doc = web_sys::window()
        .expect("window")
        .document()
        .expect("document");
    request_animation_frame(move || {
        if let Some(el) = doc
            .query_selector(".sheet, .sidebar.open")
            .ok()
            .flatten()
            .and_then(|e| e.dyn_into::<HtmlElement>().ok())
        {
            let _ = el.focus();
        }
    });
}
pub fn restore_focus() {
    LAST_FOCUS.with(|p| {
        if let Some(el) = p.borrow_mut().take() {
            request_animation_frame(move || {
                if el.get_bounding_client_rect().right() <= 0.0 {
                    if let Some(menu) = web_sys::window()
                        .and_then(|w| w.document())
                        .and_then(|d| {
                            d.query_selector("button[aria-label='Open navigation']")
                                .ok()
                                .flatten()
                        })
                        .and_then(|v| v.dyn_into::<HtmlElement>().ok())
                    {
                        let _ = menu.focus();
                    }
                } else {
                    let _ = el.focus();
                }
            });
        }
    });
}
pub fn dialog_key(ui: RwSignal<Ui>, ev: KeyboardEvent) {
    if ev.key() == "Escape" {
        ui.update(|u| {
            u.sheet.clear();
            u.nav = false;
        });
        restore_focus();
        ev.prevent_default();
    }
    if ev.key() == "Tab" {
        let doc = web_sys::window()
            .expect("window")
            .document()
            .expect("document");
        let Some(dialog) = doc.query_selector(".sheet, .sidebar.open").ok().flatten() else {
            return;
        };
        let Ok(nodes) = dialog.query_selector_all(
            "button:not([disabled]), a[href], input, textarea, summary, [tabindex='0']",
        ) else {
            return;
        };
        let len = nodes.length();
        if len == 0 {
            ev.prevent_default();
            return;
        }
        let first = nodes.item(0).and_then(|n| n.dyn_into::<HtmlElement>().ok());
        let last = nodes
            .item(len - 1)
            .and_then(|n| n.dyn_into::<HtmlElement>().ok());
        if let (Some(first), Some(last)) = (first, last) {
            let active = doc.active_element();
            if ev.shift_key()
                && (active.as_ref() == Some(first.as_ref())
                    || active.as_ref() == Some(dialog.as_ref()))
            {
                let _ = last.focus();
                ev.prevent_default();
            } else if !ev.shift_key() && active.as_ref() == Some(last.as_ref()) {
                let _ = first.focus();
                ev.prevent_default();
            }
        }
    }
}
pub(super) fn graph_pointer(
    graph: RwSignal<GraphState>,
    event: web_sys::PointerEvent,
    start: bool,
) {
    let x = event.client_x();
    let y = event.client_y();
    let element = event
        .current_target()
        .and_then(|v| v.dyn_into::<web_sys::Element>().ok());
    if start {
        if event
            .target()
            .and_then(|v| v.dyn_into::<web_sys::Element>().ok())
            .and_then(|e| {
                e.closest(".selectable-node, .graph-junction")
                    .ok()
                    .flatten()
            })
            .is_some()
        {
            return;
        }
        if let Some(element) = &element {
            let _ = element.set_pointer_capture(event.pointer_id());
        }
        graph.update(|g| g.drag = Some((x, y)));
    } else if let Some((last_x, last_y)) = graph.with_untracked(|g| g.drag) {
        let width = element
            .as_ref()
            .map(|e| e.get_bounding_client_rect().width())
            .unwrap_or(1000.0)
            .max(1.0);
        graph.update(|g| {
            g.x = ((last_x - x) * 1300.0 / width / g.zoom + g.x).clamp(-1300.0, 1800.0);
            g.y = ((last_y - y) * 1300.0 / width / g.zoom + g.y).clamp(-1000.0, 1600.0);
            g.drag = Some((x, y));
        });
    }
}
