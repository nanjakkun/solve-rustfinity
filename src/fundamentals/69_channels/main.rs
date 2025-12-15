/*
Implement three key functions that work with a message processing system:

    create_message_channel() - Creates a channel for sending Message structs
    create_producer_thread() - Creates a thread that analyzes and forwards messages
    create_consumer_thread() - Creates a thread that formats and collects messages

Producer Thread

    Receives a vector of messages and a sender channel
    Updates priority based on content keywords:
        "ERROR" → Critical
        "WARNING" → High
        "DEBUG" → Medium
        Others become Low
    Sends each updated message through the channel

Consumer Thread

    Receives messages until the channel is closed
    Formats each message as: [PRIORITY|SENDER_ID] CONTENT where PRIORITY is one of: LOW, MED, HIGH, CRIT
    Returns a vector of formatted message strings

*/

use std::fmt;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;

#[derive(Debug)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pri = match self {
            Priority::Low => "LOW",
            Priority::Medium => "MED",
            Priority::High => "HIGH",
            Priority::Critical => "CRIT",
        };
        write!(f, "{}", pri)
    }
}

pub struct Message {
    pub content: String,
    pub sender_id: u32,
    pub priority: Priority,
}

pub fn create_message_channel() -> (Sender<Message>, Receiver<Message>) {
    // 1. Implement this function to create and return a message channel
    mpsc::channel::<Message>()
}

pub fn create_producer_thread(messages: Vec<Message>, tx: Sender<Message>) -> JoinHandle<()> {
    // TODO: Create a thread that:
    // - Updates the priority based on content
    // - Sends the updated message through the channel
    std::thread::spawn(move || {
        for message in messages {
            let priority = if message.content.starts_with("ERROR") {
                Priority::Critical
            } else if message.content.starts_with("WARNING") {
                Priority::High
            } else if message.content.starts_with("DEBUG") {
                Priority::Medium
            } else {
                Priority::Low
            };
            let updated_message = Message {
                content: message.content,
                sender_id: message.sender_id,
                priority,
            };
            tx.send(updated_message).unwrap();
        }
    })
}

pub fn create_consumer_thread(rx: Receiver<Message>) -> JoinHandle<Vec<String>> {
    // TODO: Create a thread that:
    // - Receives messages from the channel
    // - Formats them as "[PRIORITY|SENDER_ID] CONTENT"
    // - Returns a vector of formatted messages
    std::thread::spawn(move || {
        let mut messages = vec![];
        while let Ok(msg) = rx.recv() {
            let formatted = format!("[{}|{}] {}", msg.priority, msg.sender_id, msg.content);
            messages.push(formatted);
        }
        messages
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
