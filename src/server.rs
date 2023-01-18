use tokio::{io,net::{TcpListener,TcpStream}};
use std::{collections::HashMap, sync::{Arc, Mutex}};
use std::sync::Once;

static mut STATE_APP: usize = 0;
static INIT: Once = Once::new();

trait servConfig {
    //return String it is temporarily, next version return handler
    fn getRoad(&self, addr: String) -> String;
    fn setRoad(&mut self, addr: String, handler: String) -> String;
}

trait App: servConfig{
    fn new() -> Box<dyn servConfig>;
}

struct ServerRoads{
    roads: Roads,
}
struct Roads{
    mapRoads: HashMap<String, String>,
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
    fn getRoad(&self, addr: String) -> String{
        //example
        self.server_roads.roads.mapRoads.get(&addr).unwrap().clone()
    }

    fn setRoad(&mut self, addr: String, handler: String) -> String {
        //example 
        self.server_roads.roads.mapRoads.insert(addr, handler).unwrap().clone()
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

    //return String it is temporarily, next version return Application
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
}

pub fn test_build_for_Application(){
    //let mut hash = HashMap::new();   
    //let a = hash.insert("awd".to_string(), "awda".to_string()).unwrap();
    unsafe{
        if STATE_APP == 1 {
            //return None;
        }
        //let app = Application::new();
        INIT.call_once(|| {
             STATE_APP = 1;
        });
        //app
    }
    
    
}