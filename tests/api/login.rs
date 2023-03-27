use crate::helpers::create_and_run_test_app;

// Must redirect to `/dashboard/` after a successful login.
#[tokio::test]
async fn login_redirects_to_dashboard_when_successful() {
    let test_app = create_and_run_test_app().await;

    let login_body = serde_json::json!({
        "username": &test_app.test_user.username,
        "password": &test_app.test_user.password
    });

    let response = test_app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/dashboard/");

    let html_page = test_app.get_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", test_app.test_user.username)));
}

// Must return a flash error message,
// when a `POST` request is received at `/login` with invalid data.
#[tokio::test]
async fn login_returns_an_error_flash_message_when_unsuccessful() {
    let test_app = create_and_run_test_app().await;

    let login_body = serde_json::json!({
        "username": "fake-username",
        "password": "fake-password"
    });

    let response = test_app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");

    let html_page = test_app.get_login_html().await;
    assert!(html_page.contains("Authentication failed."));
}
