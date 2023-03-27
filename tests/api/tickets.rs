use crate::helpers::create_and_run_test_app;

// Must return a `303 See Other` response,
// when a `POST` request with valid form data is received at `/dashboard/tickets/new`.
#[tokio::test]
async fn create_ticket_returns_a_303_when_valid_form_data() {
    let test_app = create_and_run_test_app().await;
    test_app.test_user.login(&test_app).await;

    let body = "title=Issue with ...&description=After doing ...";

    let response = test_app.post_tickets(body.into()).await;
    assert_eq!(response.status().as_u16(), 303);

    let html_page = test_app.get_create_tickets_html().await;
    assert!(html_page.contains("You have successfully created a new ticket."));
}

// Must persist a ticket,
// when a `POST` request with valid form data is received at `/dashboard/tickets/new`.
#[tokio::test]
async fn create_ticket_persists_the_new_ticket() {
    let test_app = create_and_run_test_app().await;
    test_app.test_user.login(&test_app).await;

    let body = "title=Issue with ...&description=After doing ...";

    test_app.post_tickets(body.into()).await;

    let saved = sqlx::query!("SELECT title, description FROM tickets")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch the saved ticket");
    assert_eq!(saved.title, "Issue with ...");
    assert_eq!(saved.description, "After doing ...");
}

// Must redirect an unknown user trying to access the create ticket form.
#[tokio::test]
async fn create_ticket_form_redirects_when_not_logged_in() {
    let test_app = create_and_run_test_app().await;

    let response = test_app.get_create_tickets().await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");
}

// Must be logged in to create a new ticket.
#[tokio::test]
async fn create_ticket_redirects_when_not_logged_in() {
    let test_app = create_and_run_test_app().await;

    let body = "title=Issue with ...&description=After doing ...";

    let response = test_app.post_tickets(body.into()).await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");
}

// Must return a `400 Bad Request` response,
// when a `POST` request with missing data is received at `/dashboard/tickets/new`.
#[tokio::test]
async fn create_ticket_returns_a_400_when_missing_data() {
    let test_app = create_and_run_test_app().await;
    test_app.test_user.login(&test_app).await;

    let test_cases = vec![
        ("title=Issue with ...", "missing the description"),
        ("description=After doing ...", "missing the title"),
        ("", "missing both title and description"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = test_app.post_tickets(invalid_body.into()).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with a `400 Bad Request` response when the payload was {}.",
            error_message
        );
    }
}

// Must return a `400 Bad Request` response,
// when a `POST` request with invalid data is received at `/dashboard/tickets/new`.
#[tokio::test]
async fn create_ticket_returns_a_400_when_invalid_data() {
    let test_app = create_and_run_test_app().await;
    test_app.test_user.login(&test_app).await;

    let test_cases = vec![
        (
            "title=Issue with ...&description=",
            "missing a correct description",
        ),
        (
            "title=&description=After doing ...",
            "missing a correct title",
        ),
        (
            "title=&description=",
            "missing a correct title and description",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = test_app.post_tickets(invalid_body.into()).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with a `400 Bad Request` response when the payload was {}.",
            error_message
        );
    }
}

// Must return a `500 Internal Server Error` response,
// when a `POST` request triggering a fatal database error is received at `/dashboard/tickets/new`.
#[tokio::test]
async fn create_ticket_returns_a_500_when_fatal_database_error() {
    let test_app = create_and_run_test_app().await;
    test_app.test_user.login(&test_app).await;

    let body = "title=Issue with ...&description=After doing ...";

    sqlx::query!("ALTER TABLE tickets DROP COLUMN title",)
        .execute(&test_app.db_pool)
        .await
        .expect("Failed to drop the title column from the tickets table");

    // Because of the dropped column this will trigger a fatal database error.
    let response = test_app.post_tickets(body.into()).await;
    assert_eq!(response.status().as_u16(), 500);
}
