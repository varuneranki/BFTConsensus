use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Define a message type
#[derive(Debug, Clone)]
enum Message {
    Attack,
    Retreat,
}

// Define a struct to represent a server
struct Server {
    id: usize,
    messages: Arc<Mutex<Vec<Message>>>,
}

impl Server {
    // Constructor for creating a new server
    fn new(id: usize) -> Server {
        Server {
            id,
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Method for sending a message to all other servers
    fn send_message(&self, message: Message, servers: &[Server]) {
        for server in servers {
            if server.id != self.id {
                let mut messages = server.messages.lock().unwrap();
                messages.push(message.clone());
            }
        }
    }

    // Method for receiving messages from other servers and performing consensus
    fn receive_messages(&self, servers: &[Server]) -> Message {
        // Collect messages from all other servers
        let mut received_messages = Vec::new();
        for server in servers {
            if server.id != self.id {
                let messages = server.messages.lock().unwrap();
                received_messages.extend(messages.iter().cloned());
            }
        }

        // Perform simple consensus
        let attack_count = received_messages.iter().filter(|&m| *m == Message::Attack).count();
        let retreat_count = received_messages.iter().filter(|&m| *m == Message::Retreat).count();

        if attack_count > retreat_count {
            Message::Attack
        } else {
            Message::Retreat
        }
    }
}

fn main() {
    // Define the number of servers
    let num_servers = 3;

    // Create servers
    let mut servers = Vec::new();
    for i in 0..num_servers {
        servers.push(Server::new(i));
    }

    // Simulate communication between servers
    for server in &servers {
        let servers_ref = servers.clone();
        let server_ref = server.clone();
        thread::spawn(move || {
            // Simulate sending messages
            for _ in 0..3 {
                server_ref.send_message(Message::Attack, &servers_ref);
                thread::sleep(Duration::from_secs(1));
                server_ref.send_message(Message::Retreat, &servers_ref);
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    // Simulate receiving and making decisions based on received messages
    for server in &servers {
        let servers_ref = servers.clone();
        let server_ref = server.clone();
        thread::spawn(move || {
            // Simulate receiving messages and making decisions
            for _ in 0..3 {
                let decision = server_ref.receive_messages(&servers_ref);
                println!("Server {} decision: {:?}", server_ref.id, decision);
                thread::sleep(Duration::from_secs(2));
            }
        });
    }

    // Keep the main thread alive
    thread::sleep(Duration::from_secs(10));
}

//Compile fails with 4 errors. Sample code failure. 