use axum_test::TestServer;

use outro_08::server;

#[tokio::test]
async fn test_health_check() {
    let server = TestServer::new(server::Server::new().app());

    let response = server.get("/health").await;

    response.assert_status_ok();
    assert_eq!(response.text(), "OK");
}

#[tokio::test]
async fn test_server() {
    let server = TestServer::new(server::Server::new().app());

    let ticket = server::CreateTicket {
        title: "1".into(),
        description: "2".into(),
    };
    let response = server.post("/create").json(&ticket).await;
    response.assert_status_ok();

    let returned_id: server::CreateTicketResponse = response.json();
    assert_eq!(returned_id.id, 0);

    let ticket = server::CreateTicket {
        title: "2".into(),
        description: "3".into(),
    };
    let response = server.post("/create").json(&ticket).await;
    response.assert_status_ok();

    let returned_id: server::CreateTicketResponse = response.json();
    assert_eq!(returned_id.id, 1);

    let ticket = server::GetTicket { id: 1 };
    let response = server.post("/get").json(&ticket).await;
    response.assert_status_ok();
    let returned_ticket: server::GetTicketResponse = response.json();
    let returned_ticket = returned_ticket.ticket.unwrap();
    assert_eq!(returned_ticket.id, 1);
    assert_eq!(returned_ticket.title, "2");
    assert_eq!(returned_ticket.description, "3");
    assert_eq!(returned_ticket.status, "ToDo");

    let ticket = server::GetTicket { id: 2 };
    let response = server.post("/get").json(&ticket).await;
    response.assert_status_ok();
    let returned_ticket: server::GetTicketResponse = response.json();
    assert_eq!(returned_ticket.ticket.is_none(), true);

    let patch_ticket = server::PatchTicket {
        id: 2,
        title: None,
        description: None,
        status: None,
    };
    let response = server.post("/patch").json(&patch_ticket).await;
    response.assert_status_ok();
    let returned_ticket: server::GetTicketResponse = response.json();
    assert_eq!(returned_ticket.ticket.is_none(), true);

    let patch_ticket = server::PatchTicket {
        id: 1,
        title: None,
        description: None,
        status: Some("InProgress".into()),
    };
    let response = server.post("/patch").json(&patch_ticket).await;
    response.assert_status_ok();
    let returned_ticket: server::GetTicketResponse = response.json();
    assert_eq!(returned_ticket.ticket.is_some(), true);
    let returned_ticket = returned_ticket.ticket.unwrap();
    assert_eq!(returned_ticket.id, 1);
    assert_eq!(returned_ticket.title, "2");
    assert_eq!(returned_ticket.description, "3");
    assert_eq!(returned_ticket.status, "InProgress");

    let patch_ticket = server::PatchTicket {
        id: 1,
        title: Some("XX".into()),
        description: Some("4".into()),
        status: Some("DONE".into()),
    };
    let response = server.post("/patch").json(&patch_ticket).await;
    response.assert_status_ok();
    let returned_ticket: server::GetTicketResponse = response.json();
    assert_eq!(returned_ticket.ticket.is_some(), true);
    let returned_ticket = returned_ticket.ticket.unwrap();
    assert_eq!(returned_ticket.id, 1);
    assert_eq!(returned_ticket.title, "XX");
    assert_eq!(returned_ticket.description, "4");
    assert_eq!(returned_ticket.status, "Done");
}
