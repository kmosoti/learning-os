pub mod journal;
use axum::{
    Router,
    extract::{Path, State},
    http::{HeaderValue, StatusCode, header},
    response::{Html, IntoResponse},
    routing::get,
};
use learningos_protocol::lesson_bundle;
use leptos::prelude::*;
use rusqlite::{Connection, params};
use std::{
    path::Path as FsPath,
    sync::{Arc, Mutex},
};
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};
#[derive(Clone)]
pub struct Catalog(Arc<Mutex<Connection>>);
impl Catalog {
    pub fn open(path: impl AsRef<FsPath>) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        let schema: u32 = conn.pragma_query_value(None, "user_version", |r| r.get(0))?;
        if schema > 1 {
            return Err(rusqlite::Error::InvalidParameterName(
                "Catalog schema is newer than this application; no migration was attempted".into(),
            ));
        }
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "FULL")?;
        conn.execute_batch("CREATE TABLE IF NOT EXISTS lesson_bundle(fixture TEXT PRIMARY KEY NOT NULL, schema INTEGER NOT NULL CHECK(schema=1), payload TEXT NOT NULL); PRAGMA user_version=1;")?;
        let bundle = lesson_bundle();
        let payload = serde_json::to_string(&bundle).expect("typed bundle");
        conn.execute(
            "INSERT OR IGNORE INTO lesson_bundle(fixture,schema,payload) VALUES (?1,1,?2)",
            params![bundle.fixture, payload],
        )?;
        let stored: String = conn.query_row(
            "SELECT payload FROM lesson_bundle WHERE fixture=?1",
            [&bundle.fixture],
            |r| r.get(0),
        )?;
        if stored != payload {
            return Err(rusqlite::Error::InvalidParameterName("Immutable lesson changed under the same fixture version; use a reviewed new fixture namespace".into()));
        }
        Ok(Self(Arc::new(Mutex::new(conn))))
    }
    fn bundle(&self) -> rusqlite::Result<String> {
        self.0.lock().expect("catalog mutex").query_row(
            "SELECT payload FROM lesson_bundle WHERE fixture=?1",
            [learningos_domain::FIXTURE_VERSION],
            |r| r.get(0),
        )
    }
}
pub fn router(catalog: Catalog, assets: impl AsRef<FsPath>) -> Router {
    Router::new().route("/",get(shell)).route("/graph-benchmark",get(graph_shell)).route("/session/ownership",get(shell)).route("/api/v1/lesson",get(bundle)).route("/source/{id}",get(source)).fallback_service(ServeDir::new(assets).precompressed_gzip()).with_state(catalog)
    .layer(SetResponseHeaderLayer::overriding(header::CONTENT_SECURITY_POLICY,HeaderValue::from_static("default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; font-src 'self'; img-src 'self' data:; connect-src 'self'; worker-src 'self'; object-src 'none'; base-uri 'none'; frame-ancestors 'none'; form-action 'self'")))
    .layer(SetResponseHeaderLayer::overriding(header::X_CONTENT_TYPE_OPTIONS,HeaderValue::from_static("nosniff")))
    .layer(SetResponseHeaderLayer::overriding(header::REFERRER_POLICY,HeaderValue::from_static("no-referrer")))
}
async fn shell() -> Html<String> {
    let owner = Owner::new();
    let body = owner.with(|| view! {<learningos_web::App/>}.to_html());
    Html(format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1,interactive-widget=resizes-content\"><meta name=\"theme-color\" content=\"#141414\"><meta name=\"description\" content=\"A personal offline workspace for evidence-grounded learning\"><link rel=\"manifest\" href=\"/manifest.webmanifest\"><link rel=\"icon\" href=\"/icon.svg\" type=\"image/svg+xml\"><link rel=\"apple-touch-icon\" href=\"/icon-192.png\"><link rel=\"stylesheet\" href=\"/app.css\"><title>LearningOS · Personal learning workspace</title><script type=\"module\" src=\"/bootstrap.js\"></script></head><body>{body}<noscript><p>LearningOS needs its local Rust application to open your browser-owned journal. Enable JavaScript to load the WebAssembly application.</p></noscript></body></html>"
    ))
}
async fn bundle(State(catalog): State<Catalog>) -> impl IntoResponse {
    match catalog.bundle() {
        Ok(json) => (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, "application/json; charset=utf-8"),
                (header::CACHE_CONTROL, "no-cache"),
            ],
            json,
        )
            .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Catalog unavailable").into_response(),
    }
}
async fn source(Path(id): Path<String>) -> impl IntoResponse {
    match learningos_domain::sources()
        .into_iter()
        .find(|s| s.id == id)
    {
        Some(document) => (
            [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            document.content,
        )
            .into_response(),
        None => (StatusCode::NOT_FOUND, "Unknown source").into_response(),
    }
}
async fn graph_shell() -> Html<String> {
    let owner = Owner::new();
    let body = owner.with(|| view! {<learningos_web::graph::GraphBenchmark/>}.to_html());
    Html(format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><link rel=\"stylesheet\" href=\"/app.css\"><script type=\"module\" src=\"/bootstrap.js\"></script><title>Graph renderer experiment</title></head><body>{body}</body></html>"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_body_util::BodyExt;
    use tower::ServiceExt;
    #[tokio::test]
    async fn neutral_ssr_has_islands_and_no_solutions() {
        let app = router(Catalog::open(":memory:").unwrap(), "../web/public");
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/session/ownership")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            response.headers()[header::CONTENT_SECURITY_POLICY]
                .to_str()
                .unwrap()
                .contains("frame-ancestors 'none'")
        );
        let html = String::from_utf8(
            response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes()
                .to_vec(),
        )
        .unwrap();
        assert!(html.contains("leptos-island"));
        assert!(!html.contains("E0382"));
        assert!(!html.contains("Assigning this String"));
        assert!(html.contains("Opening your local journal"));
    }
    #[tokio::test]
    async fn catalog_roundtrips_immutable_bundle() {
        let catalog = Catalog::open(":memory:").unwrap();
        let value: learningos_protocol::LessonBundle =
            serde_json::from_str(&catalog.bundle().unwrap()).unwrap();
        assert_eq!(value.fixture, learningos_domain::FIXTURE_VERSION);
        for span in value.spans {
            assert!(
                span.text(
                    value
                        .sources
                        .iter()
                        .find(|s| s.id == span.document)
                        .unwrap()
                )
                .is_ok()
            );
        }
    }
}
