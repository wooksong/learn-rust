use std::fmt;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = match self {
            Priority::Critical => "CRITICAL",
            Priority::High => "HIGH",
            Priority::Medium => "MEDIUM",
            _ => "LOW",
        };
        write!(f, "{}", p)
    }
}

pub struct Message {
    pub content: String,
    pub sender_id: u32,
    pub priority: Priority,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}|{}] {}", self.priority, self.sender_id, self.content)
    }
}

pub fn create_message_channel() -> (Sender<Message>, Receiver<Message>) {
    mpsc::channel::<Message>()
}

pub fn create_producer_thread(messages: Vec<Message>, tx: Sender<Message>) -> JoinHandle<()> {
    thread::spawn(move || {
        for mut message in messages {
            if message.content.contains("ERROR") {
                message.priority = Priority::Critical;
            } else if message.content.contains("WARNING") {
                message.priority = Priority::High;
            } else if message.content.contains("DEBUG") {
                message.priority = Priority::Medium;
            } else {
                message.priority = Priority::Low;
            }
            tx.send(message).unwrap();
        }
    })
}

pub fn create_consumer_thread(rx: Receiver<Message>) -> JoinHandle<Vec<String>> {
    // TODO: Create a thread that:
    // - Receives messages from the channel
    // - Formats them as "[PRIORITY|SENDER_ID] CONTENT"
    // - Returns a vector of formatted messages
    thread::spawn(move || {
        let mut result = Vec::new();
        for msg in rx {
            println!("{msg}");
            result.push(msg.to_string());
        }
        result
    })
}

// Example Usage
pub fn main() {
    let (tx, rx) = create_message_channel();

    let mut producer_handles = vec![];
    for id in 0..3 {
        let tx_clone = tx.clone();
        let messages = vec![
            Message {
                content: format!("Normal message from producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("WARNING: System running hot on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("ERROR: Connection lost on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
        ];
        let handle = create_producer_thread(messages, tx_clone);
        producer_handles.push(handle);
    }

    drop(tx);
    let consumer_handle = create_consumer_thread(rx);

    for handle in producer_handles {
        handle.join().unwrap();
    }

    let results = consumer_handle.join().unwrap();
    println!("Processed messages:");
    for msg in results {
        println!("{}", msg);
    }
}
