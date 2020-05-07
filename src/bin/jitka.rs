use jitka::server::Server;

#[async_std::main]
async fn main() {
    let server = Server::default();
    server.start().await;
}
