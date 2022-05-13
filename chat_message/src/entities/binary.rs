use crate::entities::message_header::MessageHeader;
use crate::entities::file_metadata::FileMetadata;

pub struct BinaryMessage{
    header: MessageHeader,
    metadata: FileMetadata,
}