mod server;
mod request;

fn main() {
    let my_server = server::Server::new(String::from("localhost:8080"));
    my_server.run();
}
