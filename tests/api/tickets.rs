use crate::helpers::create_and_run_test_app;
use tessera::routes::{get_ticket, get_tickets};

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

// Must return a `200 Ok` response,
// when a `GET` request is received at `/dashboard/tickets`.
#[tokio::test]
async fn see_tickets_returns_a_200() {
    let test_app = create_and_run_test_app().await;
    test_app.test_user.login(&test_app).await;

    let body = "title=Issue with x&description=After doing x";
    let body2 = "title=Issue with y&description=After doing y";
    let body3 = "title=Issue with z&description=After doing z";

    test_app.post_tickets(body.into()).await;
    test_app.post_tickets(body2.into()).await;
    test_app.post_tickets(body3.into()).await;

    let response = test_app.get_see_tickets().await;
    assert_eq!(response.status().as_u16(), 200);
}

// Must return every ticket,
// when a `GET` request is received at `/dashboard/tickets`.
#[tokio::test]
async fn see_tickets_returns_tickets() {
    let test_app = create_and_run_test_app().await;
    test_app.test_user.login(&test_app).await;

    let body = "title=Issue with x&description=After doing x";
    let body2 = "title=Issue with y&description=After doing y";
    let body3 = "title=Issue with z&description=After doing z";

    test_app.post_tickets(body.into()).await;
    test_app.post_tickets(body2.into()).await;
    test_app.post_tickets(body3.into()).await;

    let saved = get_tickets(&test_app.db_pool).await.unwrap();

    let saved_ticket_x = saved.get(0).unwrap();
    assert_eq!(saved_ticket_x.id, 1);
    assert_eq!(saved_ticket_x.title, "Issue with x".to_string());
    assert_eq!(saved_ticket_x.description, "After doing x".to_string());
    assert!(saved_ticket_x.is_open);

    let saved_ticket_y = saved.get(1).unwrap();
    assert_eq!(saved_ticket_y.id, 2);
    assert_eq!(saved_ticket_y.title, "Issue with y".to_string());
    assert_eq!(saved_ticket_y.description, "After doing y".to_string());
    assert!(saved_ticket_y.is_open);

    let saved_ticket_z = saved.get(2).unwrap();
    assert_eq!(saved_ticket_z.id, 3);
    assert_eq!(saved_ticket_z.title, "Issue with z".to_string());
    assert_eq!(saved_ticket_z.description, "After doing z".to_string());
    assert!(saved_ticket_z.is_open);
}

// Must redirect an unknown user trying to see tickets.
#[tokio::test]
async fn see_tickets_redirects_if_not_logged_in() {
    let test_app = create_and_run_test_app().await;

    let response = test_app.get_see_tickets().await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");
}

// Must return a `200 Ok` response,
// when a `GET` request with a valid ticket id is received at `/dashboard/tickets/{id}`.
#[tokio::test]
async fn see_ticket_returns_a_200_when_valid_ticket_id() {
    let test_app = create_and_run_test_app().await;
    test_app.test_user.login(&test_app).await;

    let body = "title=Issue with x&description=After doing x";

    test_app.post_tickets(body.into()).await;

    let response = test_app.get_see_ticket(1).await;
    assert_eq!(response.status().as_u16(), 200);
}

// Must return a ticket,
// when a `GET` request with a valid ticket id is received at `/dashboard/tickets/{id}`.
#[tokio::test]
async fn see_ticket_returns_ticket() {
    let test_app = create_and_run_test_app().await;
    test_app.test_user.login(&test_app).await;

    let body = "title=Issue with x&description=After doing x";
    let body2 = "title=Issue with y&description=After doing y";
    let body3 = "title=Issue with z&description=After doing z";

    test_app.post_tickets(body.into()).await;
    test_app.post_tickets(body2.into()).await;
    test_app.post_tickets(body3.into()).await;

    let saved_ticket_x = get_ticket(&test_app.db_pool, 1).await.unwrap();
    assert_eq!(saved_ticket_x.id, 1);
    assert_eq!(saved_ticket_x.title, "Issue with x".to_string());
    assert_eq!(saved_ticket_x.description, "After doing x".to_string());
    assert!(saved_ticket_x.is_open);

    let saved_ticket_y = get_ticket(&test_app.db_pool, 2).await.unwrap();
    assert_eq!(saved_ticket_y.id, 2);
    assert_eq!(saved_ticket_y.title, "Issue with y".to_string());
    assert_eq!(saved_ticket_y.description, "After doing y".to_string());
    assert!(saved_ticket_y.is_open);

    let saved_ticket_z = get_ticket(&test_app.db_pool, 3).await.unwrap();
    assert_eq!(saved_ticket_z.id, 3);
    assert_eq!(saved_ticket_z.title, "Issue with z".to_string());
    assert_eq!(saved_ticket_z.description, "After doing z".to_string());
    assert!(saved_ticket_z.is_open);
}

// Must redirect an unknown user trying to see a ticket.
#[tokio::test]
async fn see_ticket_redirects_if_not_logged_in() {
    let test_app = create_and_run_test_app().await;

    let response = test_app.get_see_ticket(1).await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");
}

// Must return a `400 Bad Request` response,
// when a `GET` request with an invalid ticket id is received at `/dashboard/tickets/{id}`.
#[tokio::test]
async fn see_ticket_returns_a_400_when_invalid_ticket_id() {
    let test_app = create_and_run_test_app().await;

    let response = test_app.get_see_ticket(1).await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");
}

// Must return a `303 See Other` response,
// when a `POST` request with a valid ticket id is received at `/dashboard/tickets/{id}/close`.
#[tokio::test]
async fn close_ticket_returns_a_303_when_valid_ticket_id() {
    let test_app = create_and_run_test_app().await;
    test_app.test_user.login(&test_app).await;

    let body = "title=Issue with ...&description=After doing ...";

    test_app.post_tickets(body.into()).await;

    let response = test_app.post_close_ticket(1).await;
    assert_eq!(response.status().as_u16(), 303);

    let html_page = test_app.get_see_ticket_html(1).await;
    assert!(html_page.contains("You have successfully closed this ticket."));
}

// Must close a ticket,
// when a `POST` request with a valid ticket id is received at `/dashboard/tickets/{id}/close`.
#[tokio::test]
async fn close_ticket_closes_ticket() {
    let test_app = create_and_run_test_app().await;
    test_app.test_user.login(&test_app).await;

    let body = "title=Issue with x&description=After doing x";

    test_app.post_tickets(body.into()).await;

    test_app.post_close_ticket(1).await;

    let saved_ticket_x = get_ticket(&test_app.db_pool, 1).await.unwrap();
    assert!(!saved_ticket_x.is_open);
}

// Must redirect an unknown user trying to close a ticket.
#[tokio::test]
async fn close_ticket_redirects_if_not_logged_in() {
    let test_app = create_and_run_test_app().await;

    let response = test_app.post_close_ticket(1).await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");
}

// Must return a `303 See Other` response,
// when a `POST` request with an invalid ticket id is received at `/dashboard/tickets/{id}/close`.
#[tokio::test]
async fn close_ticket_returns_a_303_when_invalid_ticket_id() {
    let test_app = create_and_run_test_app().await;
    test_app.test_user.login(&test_app).await;

    let response = test_app.post_close_ticket(1).await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "/dashboard/tickets/1"
    );
}
