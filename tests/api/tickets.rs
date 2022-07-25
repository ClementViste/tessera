use crate::helpers::spawn_test_app;

// Must return a `200 OK` response,
// when a `POST` request is received at `/tickets` with valid form data.
#[tokio::test]
async fn create_ticket_returns_a_200_for_valid_form_data() {
    // Build and then run the test application.
    let app = spawn_test_app().await;

    // Create the body of the request.
    let body = "title=Issue with ...&description=After doing ...";

    // Send a request and then return the response.
    let response = app.post_tickets(body.into()).await;

    // Check.
    assert_eq!(response.status().as_u16(), 200);
}

// Must persist the new ticket,
// when a `POST` request is received at `/tickets` with valid form data .
#[tokio::test]
async fn create_ticket_persists_the_new_ticket() {
    // Build and then run the test application.
    let app = spawn_test_app().await;

    // Create the body of the request.
    let body = "title=Issue with ...&description=After doing ...";

    // Send a request.
    app.post_tickets(body.into()).await;

    // Fetch the saved ticket.
    let saved = sqlx::query!("SELECT title, description FROM tickets",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch the saved ticket");

    // Check.
    assert_eq!(saved.title, "Issue with ...");
    assert_eq!(saved.description, "After doing ...");
}

// Must return a `400 Bad Request` response,
// when a `POST` request is received at `/tickets` with missing data.
#[tokio::test]
async fn create_ticket_returns_a_400_when_data_is_missing() {
    // Build and then run the test application.
    let app = spawn_test_app().await;

    // Create the body of the request.
    let test_cases = vec![
        ("title=Issue with ...", "missing the description"),
        ("description=After doing ...", "missing the title"),
        ("", "missing both title and description"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Send a request and then return the response.
        let response = app.post_tickets(invalid_body.into()).await;

        // Check.
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with a `400 Bad Request` response when the payload was {}.",
            error_message
        );
    }
}

// Must return a `400 Bad Request` response,
// when a `POST` request is received at `/tickets` with invalid data.
#[tokio::test]
async fn create_ticket_returns_a_400_when_fields_are_present_but_invalid() {
    // Build and then run the test application.
    let app = spawn_test_app().await;

    // Create the body of the request.
    let test_cases = vec![
        (
            "title=Issue with ...&description=",
            "missing a correct description",
        ),
        (
            "title=&description=After doing ...",
            "missing a correct title",
        ),
    ];
    for (invalid_body, error_message) in test_cases {
        // Send a request and then return the response.
        let response = app.post_tickets(invalid_body.into()).await;

        // Check.
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with a `400 Bad Request` response when the payload was {}.",
            error_message
        );
    }
}
