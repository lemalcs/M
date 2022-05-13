//use crate::application::common::message_repo::MessageRepository;
use crate::{application::repos::chat_repo::ChatMessageRepository, entities::html::HtmlMessage};
pub struct SendTextMessageCommand{
    repo: Box<dyn ChatMessageRepository>,
}

impl SendTextMessageCommand{
    fn new(repository: Box<dyn ChatMessageRepository>) -> Self{
        Self{
            repo: repository,
        }
    }
}

// pub struct SendTextMessageCommand2{
//     repo:Box<dyn ChatMessageRepository>,
// }

// impl SendTextMessageCommand2{
//     fn new(repository: impl ChatMessageRepository) -> Self{
//         Self{
//             repo: repository,
//         }
//     }
// }

// fn nothing(par: ChatMessageRepository){
//     par.save( crate::entities::html::HtmlMessage{
//         content: String::from("luis"),
//         header: crate::entities::message_header::MessageHeader{
//             id_replied_message: 0,
//         }
//     });
// }