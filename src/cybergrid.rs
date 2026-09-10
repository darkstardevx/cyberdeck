//! # CYBERGRID palette bridge (`core` branch)
//!
//! Exposes the named colour themes from the shared [`cybercore`] crate
//! (schema v2) over the HTTP API, so the CYBERDECK front-end can list and
//! apply them next to its own file-based theme chooser. Nothing here
//! replaces the existing `/api/themes` scan — it's an additional source.
//!
//! Endpoints:
//! - `GET /api/cybergrid/themes`      — every theme + its 11 colour roles
//! - `GET /api/cybergrid/css/:name`   — a ready `:root{ --bg:#… }` block
//!
//! The active theme can be pinned per process with `CYBERGRID_THEME`.

use axum::{extract::Path, http::header, response::IntoResponse, Json};
use serde_json::{json, Value};

fn role_map(name: &str, p: &cybercore::schema::Palette) -> Value {
    json!({
        "name": name,
        "bg": p.bg,
        "white": p.white,
        "acid_green": p.acid_green,
        "hot_pink": p.hot_pink,
        "purple": p.purple,
        "cyan": p.cyan,
        "orange": p.orange,
        "red": p.red,
        "panel": p.panel,
        "line": p.line,
        "muted": p.muted,
    })
}

/// `GET /api/cybergrid/themes` — the whole CYBERGRID set with the active slug.
pub async fn list_themes() -> impl IntoResponse {
    let s = cybercore::schema::load();
    let themes: Vec<Value> = s
        .theme_names()
        .filter_map(|n| s.theme(n).map(|p| role_map(n, p)))
        .collect();
    Json(json!({ "active": s.active, "themes": themes }))
}

/// `GET /api/cybergrid/css/:name` — `:root{}` custom properties for one theme
/// (falls back to the active theme when the name is unknown).
pub async fn theme_css(Path(name): Path<String>) -> impl IntoResponse {
    let s = cybercore::schema::load();
    let p = s.theme(&name).unwrap_or_else(|| s.active_theme());
    let css = format!(
        ":root{{\
--bg:#{bg};--fg:#{white};\
--acid:#{acid};--pink:#{pink};--purple:#{purple};--cyan:#{cyan};\
--orange:#{orange};--red:#{red};\
--panel:#{panel};--line:#{line};--muted:#{muted};\
}}\n",
        bg = p.bg,
        white = p.white,
        acid = p.acid_green,
        pink = p.hot_pink,
        purple = p.purple,
        cyan = p.cyan,
        orange = p.orange,
        red = p.red,
        panel = p.panel,
        line = p.line,
        muted = p.muted,
    );
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        css,
    )
}
