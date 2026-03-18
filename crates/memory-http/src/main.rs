#[tokio::main]
async fn main() {
    let app = memory_http::app_for_test("memory.db").await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8787")
        .await
        .expect("bind failed");
    axum::serve(listener, app).await.expect("server error");
}
