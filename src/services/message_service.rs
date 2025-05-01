use crate::models::message::Message;

pub fn process_message(msg: Message) -> Message {
    Message {
        content: format!("Echo: {}", msg.content),
    }
}
