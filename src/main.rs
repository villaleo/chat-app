use chat_app::server::Server;

#[tokio::main]
async fn main() {
    env_logger::init();

    let server = Server::new(8080);
    server.run().await;
}
