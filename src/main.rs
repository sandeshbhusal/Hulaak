use std::thread::JoinHandle;

use anyhow::Result;

type Message = String;

trait Actor {
    fn get_name(&self) -> &'static str;
    fn consume_message(&self, message: Message);
    fn emit_message(&self) -> Option<Message>;
    fn handle_message(&self, message: Message) {
    }
}

struct SaysHello;
struct SaysWhat;

impl Actor for SaysHello {
    fn get_name(&self) -> &'static str {
        "SaysHello"
    }

    fn consume_message(&self, message: Message) {
        println!("{} says: {}", self.get_name(), message);
    }

    fn emit_message(&self) -> Option<Message> {
        Some("Hello!".to_string())
    }
}

impl Actor for SaysWhat {
    fn get_name(&self) -> &'static str {
        "SaysWhat"
    }

    fn consume_message(&self, message: Message) {
        println!("{} says: {}", self.get_name(), message);
    }

    fn emit_message(&self) -> Option<Message> {
        Some("What?".to_string())
    }
}

fn main() -> Result<()> {
    let mut actor1 = Box::new(SaysHello);
    let mut actor2 = Box::new(SaysWhat);

    Ok(())
}
