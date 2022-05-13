use crate::entities::message_header::MessageHeader;

pub struct TextMessage{
    header: MessageHeader,

    xaml_package: Vec<i8>,
}