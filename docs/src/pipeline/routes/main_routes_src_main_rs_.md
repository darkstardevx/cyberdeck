## Snippet: Main_Routes (src/main.rs)

```rust
let app = Router::new()
    .route("/api/themes", get(routes::get_themes_list))
    .route("/", get(routes::get_index_page))
    // Updated paths to /cyberdeck/api/
    .route("/cyberdeck/api/state", get(routes::get_cyberdeck_state))
    .route("/cyberdeck/api/command", post(routes::post_cyberdeck_command))
    .route("/cyberdeck/api/action", post(routes::post_cyberdeck_action)) // Add this for the new popup
    .route("/cyberdeck/api/list", get(routes::list_diagnostics))
    .fallback_service(ServeDir::new("static"))
    .with_state(state);
```