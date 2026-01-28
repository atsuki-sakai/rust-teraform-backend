mod common;

#[tokio::test]
async fn test_health_check_returns_ok() {
    let (server, pool) = common::create_test_server().await;

    let response = server.get("/health").await;

    response.assert_status_ok();
    response.assert_text("OK");

    // Cleanup
    common::cleanup_test_data(&pool).await;
}
