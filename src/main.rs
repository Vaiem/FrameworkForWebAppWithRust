mod server;
use server::servConfig;
use tokio::net::TcpStream;
use tokio::macros::support::Future;
use std::{sync::Arc};
use core::pin::Pin;
#[tokio::main]
async fn main() {
    //example
    let mut serv_build = server::Application::Build("adsada".to_string()).await.unwrap();
    serv_build.setRoad("omegalol".to_string(), handler_user_requst).unwrap();
    let serv = Arc::new(serv_build);

    serv.run().await;
}

fn handler_user_requst(stream: TcpStream) -> Pin<Box<dyn Future<Output = ()>+ Send + 'static>>{
    Box::pin(async move{
        5;
    })
}