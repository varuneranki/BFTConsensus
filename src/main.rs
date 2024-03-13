use rand::Rng;

#[derive(Debug, Clone)]
enum Message {
    Attack,
    Retreat,
}

struct Server {
    id: u32,
    peers: Vec<u32>,
    received_messages: Vec<Message>,
}

impl Server {
    fn new(id: u32, peers: Vec<u32>) -> Self {
        Server { id, peers, received_messages: Vec::new() }
    }

    fn send_message(&self, message: Message) {
        for peer in self.peers.iter() {
            // Replace this with your communication library
            println!("Sending {:?} to server {}", message, peer);
        }
    }

    fn handle_message(&mut self, message: Message) {
        self.received_messages.push(message);
    }

    fn perform_consensus(&mut self) -> Option<Message> {
        let mut counts = [0; 2];
        for message in self.received_messages.iter() {
            match message {
                Message::Attack => counts[0] += 1,
                Message::Retreat => counts[1] += 1,
            }
        }

        if counts[0] > counts[1] {
            Some(Message::Attack)
        } else if counts[1] > counts[0] {
            Some(Message::Retreat)
        } else {
            None // No consensus yet
        }
    }

    fn run(&mut self) {
        let mut rng = rand::thread_rng();
        let initial_message = if rng.gen_bool(0.5) {
            Message::Attack
        } else {
            Message::Retreat
        };

        self.send_message(initial_message);

        loop {
            let messages_from_peers = // Receive messages from peers
                // Replace this with your communication library
                vec![Message::Attack]; // Example message

            /*for message in messages_from_peers.iter() {
                self.handle_message(*message);
            }*/

            for message in messages_from_peers.iter() {
                self.handle_message(message.clone());
            }

            let consensus = self.perform_consensus();

            if let Some(decision) = consensus {
                println!("Server {} reached consensus: {:?}", self.id, decision);
                break;
            }

            let next_message = if rng.gen_bool(0.5) {
                Message::Attack
            } else {
                Message::Retreat
            };

            self.send_message(next_message);
        }
    }
}

fn main() {
   /* let server1 = Server::new(1, vec![2, 3]);
    let server2 = Server::new(2, vec![1, 3]);
    let server3 = Server::new(3, vec![1, 2]);

    server1.run();
    server2.run();
    server3.run();*/
    let server1 = Server::new(1, vec![2, 3]).run();
    let server2 = Server::new(2, vec![1, 3]).run();
    let server3 = Server::new(3, vec![1, 2]).run();
}

