use std::thread;

use websocket::Server;

pub fn start() {
    let server = Server::bind("0.0.0.0:3001").unwrap();

    for connection in server {
        thread::spawn(move || {
            let request = connection.unwrap().read_request().unwrap();
            let response = request.accept();
            let mut client = response.send().unwrap();

            let ip = client.get_mut_sender()
				.get_mut()
				.peer_addr()
				.unwrap();
                
           println!("Connection from {}", ip);
        });
    }
}