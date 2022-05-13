use chrono::prelude::*;

struct MessageHeader {
    id: i32,
    id_sender: i32,
    id_receiver: i32,
    send_date: DateTime<Utc>,
    receive_date: DateTime<Utc>,
    uid: String,
    read: bool,
    id_replied_message: i32,
    state: i32,
    time_id: i128,
    content: MessageContent,
}

