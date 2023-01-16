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


pub fn test_build_for_Application(){
    //let mut hash = HashMap::new();   
    //let a = hash.insert("awd".to_string(), "awda".to_string()).unwrap();
    unsafe{
        INIT.call_once(|| {
             STATE_APP = 1;
        });
    }
    
    
}