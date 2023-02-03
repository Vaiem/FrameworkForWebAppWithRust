mod server;
mod Errors;
use server::servConfig;
use tokio::{net::TcpStream, io::AsyncWriteExt};
use tokio::macros::support::Future;
use std::{sync::Arc};
use core::pin::Pin;
#[tokio::main]
async fn main() {
    //example
    let mut serv_build = server::Application::Build("127.0.0.1:7878".to_string()).await.unwrap().unwrap();
    serv_build.setRoad("GET / HTTP/1.1\r\n".to_string(), handler_user_requst);
    let serv = Arc::new(serv_build);

    serv.run().await.unwrap();
}

fn handler_user_requst(mut stream: TcpStream) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>{
    Box::pin(async move{
        let status_line = "HTTP/1.1 200 OK";
        let content = tokio::fs::read_to_string("Views/helloy.html").await.unwrap();
        let lenght = content.len();
        let response = format!("{status_line}\r\nContent-Length: {lenght}\r\n\r\n{content}");
        stream.write_all(response.as_bytes()).await.unwrap();
    })
}