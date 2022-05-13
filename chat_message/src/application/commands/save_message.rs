use crate::application::repos::chat_repo::ChatMessageRepository;
pub struct SaveHtmlMessageCommand{
    repo: Box<dyn ChatMessageRepository>,
}

impl SaveHtmlMessageCommand{
    fn new(repository: Box<dyn ChatMessageRepository>) -> Self {
        Self{
            repo: repository,
        }
    }
}