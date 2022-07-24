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
