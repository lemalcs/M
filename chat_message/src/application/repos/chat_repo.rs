use crate::entities::html::HtmlMessage;

pub trait ChatMessageRepository{
    fn save(&self, message: HtmlMessage);
}


// Implement `ChatMessageRepository` for a struct
//  impl dyn ChatMessageRepository {
//      fn save(message:HtmlMessage){

//      }
//  }