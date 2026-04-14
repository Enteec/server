mod db;
use crate::db::{models::User, *};
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let _connection = &mut establish_connection();

    {
        // temp test code remove after using User struct and implementing register
        let user = User::new();
        println!("{}", user.uuid);
        println!("{}", user.name);
        println!("{}", user.created_at);
        println!("{}", user.updated_at);
        println!("{}", user.password_hash);
        println!("{}", user.salt);
    }

    let app = Router::new().route("/ping", get(ping));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to listen on IP:PORT!");

    axum::serve(listener, app)
        .await
        .expect("Failed to start a server!");
}

async fn ping() -> &'static str {
    "pong"
}
