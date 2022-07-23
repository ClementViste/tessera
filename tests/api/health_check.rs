use crate::helpers::spawn_test_app;

// Must return a `200 OK` response with an empty body,
// when a `GET` request is received at `/health_check`.
#[tokio::test]
async fn health_check_returns_a_200_with_an_empty_body() {
    // Build and then run the test application.
    let app = spawn_test_app().await;

    // Send a request and then return the response.
    let response = app.get_health_check().await;

    // Check.
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.content_length(), Some(0));
}
