use crate::helpers::create_and_run_test_app;

// Must return a `200 OK` response,
// when a `POST` request with valid form data is received at `/tickets/new`.
#[tokio::test]
async fn create_ticket_returns_a_200_when_valid_form_data() {
    let test_app = create_and_run_test_app().await;

    let body = "title=Issue with ...&description=After doing ...";

    let response = test_app.post_tickets(body.into()).await;
    assert_eq!(response.status().as_u16(), 200);
}

// Must return a `400 Bad Request` response,
// when a `POST` request with missing data is received at `/tickets/new`.
#[tokio::test]
async fn create_ticket_returns_a_400_when_missing_data() {
    let test_app = create_and_run_test_app().await;

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
