use chrono::prelude::*;

pub struct MessageHeader {
    message_id: i32,
    sender_token: String,
    receiver_token: String,
    send_date: DateTime<Utc>,
    receive_date: DateTime<Utc>,
    uid: String,
    read: bool,
    id_replied_message: i32,
    state: i32,
    time_id: i128,
}