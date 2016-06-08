use std::thread;
use std::collections::HashMap;
use std::sync::{Arc, mpsc, Mutex};

use websocket::{Message, receiver, sender, Sender, Server, WebSocketStream};

pub fn start() {
    thread::spawn(listen);
}

fn listen() {
    let server = Server::bind("localhost:3001").unwrap();

    println!("Server listening at: localhost:3001");

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
            let (mut ws_sender, ws_receiver) = client.split();

            ws_sender.send_message(&Message::text(format!("Welcome, {}!", ip_string))).unwrap();

            thread::spawn(move || client_thread(ip_string, sender, ws_receiver));
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

fn client_thread(ip: String,
                 sender: mpsc::Sender<String>,
                 ws_receiver: receiver::Receiver<WebSocketStream>) {
    println!("client_thread - sending info...");
    sender.send(ip).unwrap();
}