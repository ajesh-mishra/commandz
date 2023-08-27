use axum::Router;
use std::net::SocketAddr;

use routes::create_router;

pub async fn serve() {
    let app: Router = create_router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}