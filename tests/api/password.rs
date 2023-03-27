use crate::helpers::create_and_run_test_app;
use uuid::Uuid;

// Must successfully change password.
#[tokio::test]
async fn change_password_works() {
    let test_app = create_and_run_test_app().await;

    let login_body = serde_json::json!({
        "username": &test_app.test_user.username,
        "password": &test_app.test_user.password
    });

    let response = test_app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/dashboard/");

    let new_password = Uuid::new_v4().to_string();

    let response = test_app
        .post_change_password(&serde_json::json!({
            "current_password": &test_app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "/dashboard/password"
    );

    let html_page = test_app.get_change_password_html().await;
    assert!(html_page.contains("Your password has been changed."));

    let response = test_app.post_logout().await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");

    let html_page = test_app.get_login_html().await;
    assert!(html_page.contains("You have successfully logged out."));

    let login_body = serde_json::json!({
        "username": &test_app.test_user.username,
        "password": &new_password
    });

    let response = test_app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/dashboard/");

    let html_page = test_app.get_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", test_app.test_user.username)));
}

// Must redirect an unknown user trying to access the change password form.
#[tokio::test]
async fn change_password_form_redirects_when_not_logged_in() {
    let test_app = create_and_run_test_app().await;

    let response = test_app.get_change_password().await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");
}

// Must be logged in to change password.
#[tokio::test]
async fn change_password_redirects_when_not_logged_in() {
    let test_app = create_and_run_test_app().await;

    let new_password = Uuid::new_v4().to_string();

    let response = test_app
        .post_change_password(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");
}

// Must have a valid current password.
#[tokio::test]
async fn change_password_returns_an_error_when_current_password_is_invalid() {
    let test_app = create_and_run_test_app().await;

    test_app
        .post_login(&serde_json::json!({
            "username": &test_app.test_user.username,
            "password": &test_app.test_user.password
        }))
        .await;

    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    let response = test_app
        .post_change_password(&serde_json::json!({
            "current_password": &wrong_password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "/dashboard/password"
    );

    let html_page = test_app.get_change_password_html().await;
    assert!(html_page.contains("The current password is incorrect."));
}

// Must have similar new password.
#[tokio::test]
async fn change_password_returns_an_error_when_new_passwords_are_different() {
    let test_app = create_and_run_test_app().await;

    test_app
        .post_login(&serde_json::json!({
            "username": &test_app.test_user.username,
            "password": &test_app.test_user.password
        }))
        .await;

    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();

    let response = test_app
        .post_change_password(&serde_json::json!({
            "current_password": &test_app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &another_new_password,
        }))
        .await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "/dashboard/password"
    );

    let html_page = test_app.get_change_password_html().await;
    assert!(
        html_page.contains("You entered two different new passwords, the field values must match.")
    );
}
