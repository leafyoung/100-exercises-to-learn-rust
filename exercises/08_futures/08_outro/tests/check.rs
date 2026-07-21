use axum_test::TestServer;
use outro_08::{app, CreateTicketResponse, GetTicketResponse, PatchTicket, Server};
use serde_json::json;

#[tokio::test]
async fn test_health_check() {
    // Instantiate the test server with the shared router setup
    let server = TestServer::new(app(Server::new()));
    let response = server.get("/health").await;
    response.assert_status_ok();
    assert_eq!(response.text(), "OK");
}

#[tokio::test]
async fn test_create_user() {
    let server = TestServer::new(app(Server::new()));

    let response = server
        .post("/ticket")
        .json(&json!({ "title": "1", "description": "2" }))
        .await;

    response.assert_status_ok();
    let new_ticket_id: CreateTicketResponse = response.json();
    assert_eq!(new_ticket_id.id, 1);

    {
        let response = server
            .post("/get")
            .json(&json!({ "id": new_ticket_id.id }))
            .await;

        response.assert_status_ok();
        let new_ticket: GetTicketResponse = response.json();
        assert!(new_ticket.ticket.is_some());

        let new_ticket = new_ticket.ticket.unwrap();
        assert_eq!(new_ticket.id, new_ticket_id.id);
        assert_eq!(new_ticket.title, "1");
        assert_eq!(new_ticket.description, "2");
        assert_eq!(new_ticket.status, "ToDo");
    }

    {
        let response = server
            .post("/patch")
            .json(&PatchTicket {
                id: new_ticket_id.id,
                title: Some("111".to_owned()),
                description: Some("222".to_owned()),
                status: Some("InProgress".to_owned()),
            })
            .await;

        response.assert_status_ok();
        let new_ticket: GetTicketResponse = response.json();
        assert!(new_ticket.ticket.is_some());

        let new_ticket = new_ticket.ticket.unwrap();
        assert_eq!(new_ticket.id, new_ticket_id.id);
        assert_eq!(new_ticket.title, "111");
        assert_eq!(new_ticket.description, "222");
        assert_eq!(new_ticket.status, "InProgress");
    }
}
