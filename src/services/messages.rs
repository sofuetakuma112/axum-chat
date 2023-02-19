use crate::{
    repositories::{
        message::{MessageRepository, MessageRepositoryForDb},
        user::{UserRepository, UserRepositoryForDb},
    },
    views::message::Message,
};

pub async fn list_messages(
    message_repository: MessageRepositoryForDb,
    user_repository: UserRepositoryForDb,
    room_id: i32,
) -> Vec<Message> {
    let messages = message_repository.find_by_room_id(room_id).await;
    let user_ids = messages
        .iter()
        .map(|message| message.user_id)
        .collect::<Vec<i32>>();
    let users = user_repository.find(&user_ids).await;
    messages
        .into_iter()
        .map(|x| {
            let posted_user = users
                .iter()
                .find(|&user| user.id.unwrap() == x.user_id)
                .unwrap();
            // .into() を呼び出し、Messageビューに変換している
            // Fromトレイトの実装を利用している
            (x, posted_user).into()
        })
        .collect::<Vec<Message>>()
}
