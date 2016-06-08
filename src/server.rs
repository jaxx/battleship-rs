use std::thread;
use std::collections::HashMap;
use std::sync::{Arc, mpsc, Mutex};

use websocket::{Server, WebSocketStream};
use websocket::sender;

pub fn start() {
    thread::spawn(listen);
}

fn listen() {
    let server = Server::bind("0.0.0.0:3001").unwrap();

    println!("Server listening at: 0.0.0.0.3001");

    let (sender, receiver) = mpsc::channel();
    let connected_clients = Arc::new(Mutex::new(HashMap::new()));

    thread::spawn(|| game_thread(connected_clients, receiver));

    for connection in server {
        let sender = sender.clone();

        thread::spawn(move || {
            let request = connection.unwrap().read_request().unwrap();
            let response = request.accept();
            let mut client = response.send().unwrap();

            let ip = client.get_mut_sender()
                .get_mut()
                .peer_addr()
                .unwrap();

            let ip_string = format!("{}", ip);

            thread::spawn(move || client_thread(ip_string, sender));
        });
    }
}

fn game_thread(clients: Arc<Mutex<HashMap<String, sender::Sender<WebSocketStream>>>>,
               receiver: mpsc::Receiver<String>) {
    match receiver.recv() {
        Ok(data) => println!("connected user from: {}", data),
        Err(_) => panic!("Error while receiving ip-address."),
    }
}

fn client_thread(ip: String, sender: mpsc::Sender<String>) {
    println!("client_thread - sending info...");
    sender.send(ip).unwrap();
}