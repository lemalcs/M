use crate::entities::message_header::MessageHeader;
use crate::entities::file_metadata::FileMetadata;

pub struct GifMessage{
    header: MessageHeader,
    metadata: FileMetadata,

    width: i32,
    height: i32,
    
    thumbnail: Vec<i8>,
}