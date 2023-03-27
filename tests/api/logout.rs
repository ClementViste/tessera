use crate::helpers::create_and_run_test_app;

// Must clear the session state, when a user logged out.
#[tokio::test]
async fn logout_clears_session_state() {
    let test_app = create_and_run_test_app().await;
    test_app.test_user.login(&test_app).await;
    test_app.test_user.logout(&test_app).await;

    // Send a request and then return the response.
    //
    // Because the user has logged out, the user is redirected to `/login`.
    let response = test_app.get_dashboard().await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");
}
