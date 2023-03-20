use crate::helpers::create_and_run_test_app;

// Must return a `200 OK` response with an empty body,
// when a `GET` request is received at `/health_check`.
#[tokio::test]
async fn health_check_returns_a_200_with_an_empty_body() {
    let test_app = create_and_run_test_app().await;

    let response = test_app.get_health_check().await;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.content_length(), Some(0));
}
