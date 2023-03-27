use crate::helpers::create_and_run_test_app;

// Must redirect an unknown user trying to access the dashboard.
#[tokio::test]
async fn dashboard_redirects_when_not_logged_in() {
    let test_app = create_and_run_test_app().await;

    let response = test_app.get_dashboard().await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");
}
