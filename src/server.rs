use tokio::{io::{self, AsyncReadExt},net::{TcpListener,TcpStream}, join};
use std::{collections::HashMap, sync::{Arc, Mutex}, future::Future};
use std::sync::Once;

static mut STATE_APP: usize = 0;
static INIT: Once = Once::new();

// update Handler later
type Handler = Box<dyn Future<Output = ()> +'static>;

trait servConfig {
    //return String it is temporarily, next version return handler
    fn getRoad(&self, addr: String) -> &Handler;
    fn setRoad(&mut self, addr: String, handler: Handler) -> Result<(), io::Error>;
}

trait App: servConfig{
    fn new() -> Box<dyn servConfig>;
}

struct ServerRoads{
    roads: Roads,
}
struct Roads{
    mapRoads: HashMap<String, Handler>,
}
struct Conection{
    tcpListener: TcpListener,
}

struct Application{
    conection: Conection,
    server_roads: ServerRoads,
}

struct MementoApplication{
    historyApp: Vec<Application>,
}

impl servConfig for Application {
    fn getRoad(&self, addr: String) -> &Handler{
        //example
        self.server_roads.roads.mapRoads.get(&addr).unwrap()
    }

    fn setRoad(&mut self, addr: String, handler: Handler) -> Result<(), io::Error>{
        //example 
        //create ERROR later
        self.server_roads.roads.mapRoads.insert(addr, handler).unwrap();
        Ok(())
    }
}

impl Application {

   async fn new(addrListener: String) -> Application{
        //Create new object Application
        Application{
            //create error later
            conection: Conection { tcpListener: TcpListener::bind(addrListener).await.unwrap() },
            server_roads: ServerRoads {
                 roads: Roads {
                     mapRoads: HashMap::new()
            } }
        }
    } 

    pub async fn Build() -> Option<Application>{
        unsafe{
            if STATE_APP == 1 {
                return None;
            }
            let app = Application::new("127.0.0.1:6379".to_string()).await;
            INIT.call_once(|| {
                 STATE_APP = 1;
            });
            Some(app)
        }
    }

    pub async fn run(&self){
        loop {
            let stream = self.conection.tcpListener.accept().await.unwrap();
            
            tokio::spawn(async move{
                 processing_request(stream.0).await;
            });      
             
        }
    }
    
}

async fn processing_request(stream: TcpStream){
     //impl later   
}

async fn Get_handler_to_request<'a>(app: &'a Application, stream:&mut TcpStream) -> Option<&'a Handler>{
    let mut buffer = vec![0;1024];
    stream.read(&mut buffer).await.unwrap();
    let keys = app.server_roads.roads.mapRoads.keys();
    for key in keys {
        if buffer.starts_with(key.as_bytes()){
            return Some(app.getRoad(key.into()));
        }
    }
    None
}

 //delet
/*pub fn createHandler( f: impl Future<Output = ()> + 'static){
        //self.setRoad("/test".to_string(), Box::new(f)).unwrap();  
        //f(stream).await;
}
pub fn processing_requst(stream: TcpStream) -> Box<dyn Future<Output = ()> + 'static>{
    Box::new(async move{
        //stream.ready(interest).await;
    })
}

pub async fn handler(){

}

pub async fn test(stream: TcpStream){
    //createHandler(processing_requst(stream));
    let a = test_build_for_Application;
    let d = a(5,processing_requst).await;
} 
pub async fn test_build_for_Application(d: i32, f: fn(TcpStream) -> Box<dyn Future<Output = ()> + 'static>) -> i32{
    5
}*/