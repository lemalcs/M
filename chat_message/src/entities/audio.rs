use crate::entities::message_header::MessageHeader;
use crate::entities::file_metadata::FileMetadata;

pub struct AudioMessage{
    header: MessageHeader,
    metadata: FileMetadata,

    duration: i64,
}