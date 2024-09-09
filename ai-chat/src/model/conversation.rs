use serde::Deserialize;
use serde::Serialize;

// The code defines a Conversation struct that contains a list of Message structs.
// Each Message has a user flag and some text.
// The Conversation struct has a new() method to create an empty conversation.
// Both structs can be serialized to and deserialized from formats like JSON, can be cloned, and can be debug-printed.

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub messages: Vec<Message>
}

impl Conversation {
    pub fn new() -> Conversation {
        Conversation {
            messages: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub user: bool,
    pub text: String
}