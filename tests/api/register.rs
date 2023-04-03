use crate::helpers::create_and_run_test_app;

// Must return a `303 See Other` response,
// when a `POST` request with valid form data is received at `/register`.
#[tokio::test]
async fn register_returns_a_303_when_valid_form_data() {
    let test_app = create_and_run_test_app().await;

    let body = &serde_json::json!({
        "username": "fake-username",
        "password": "fake-password"
    });

    let response = test_app.post_register(body).await;
    assert_eq!(response.status().as_u16(), 303);
}

// Must redirect to `/login` after a successful registration.
#[tokio::test]
async fn register_redirects_to_login_when_successful() {
    let test_app = create_and_run_test_app().await;

    let body = &serde_json::json!({
        "username": "fake-username",
        "password": "fake-password"
    });

    let response = test_app.post_register(body).await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");

    let html_page = test_app.get_login_html().await;
    assert!(html_page.contains("You have successfully registered, you can now log in."));
}

// Must persist a new user, when a new user is created with valid form data.
#[tokio::test]
async fn register_persists_the_new_user() {
    let test_app = create_and_run_test_app().await;

    let body = &serde_json::json!({
        "username": "fake-username",
        "password": "fake-password"
    });
    let body_username = body.get("username").unwrap().as_str().unwrap();

    test_app.post_register(body).await;

    let saved = sqlx::query!(
        "SELECT username, password_hash FROM users WHERE username=$1",
        body_username
    )
    .fetch_one(&test_app.db_pool)
    .await
    .expect("Failed to fetch the saved user");
    assert_eq!(saved.username, "fake-username");
}

// Must return a flash error message,
// when a `POST` request with invalid data is received at `/register`.
#[tokio::test]
async fn register_returns_an_error_flash_message_when_unsuccessful() {
    let test_app = create_and_run_test_app().await;

    let register_body = serde_json::json!({
        "username": "fake-username",
        "password": "fake-password"
    });

    test_app.post_register(&register_body).await;

    // Because the username is already taken, the registration should fail.
    let response = test_app.post_register(&register_body).await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/register");

    let html_page = test_app.get_register_html().await;
    assert!(html_page.contains("Failure to register, the username is probably already taken."));

    let html_page = test_app.get_register_html().await;
    assert!(!html_page.contains("Failure to register, the username is probably already taken."));
}
