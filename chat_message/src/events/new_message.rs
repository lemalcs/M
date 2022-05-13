use chrono::prelude::*;

pub struct NewMessageArrivedEvent{
    sender: String,
    arrive_date: DateTime<Utc>,
    message_id: i32,
}