## Snippet: Port_IP [127.0.0.1:8080] (src/main.rs)

```rust
let port = std::env::var("CYBERDECK_PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

    println!("[Cyberdeck Core] API active on http://127.0.0.1:{}", port);
    axum::serve(listener, app).await.unwrap();
```