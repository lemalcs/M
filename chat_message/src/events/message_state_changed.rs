use crate::message_state::MessageState;

pub struct MessageStateChangedEvent{
    message_id: i32,
    state: MessageState,  
}