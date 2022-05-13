use crate::entities::message_header::MessageHeader;

pub struct MarkdownMessage{
    header: MessageHeader,

    content: String,
}