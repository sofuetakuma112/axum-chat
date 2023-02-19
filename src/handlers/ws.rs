use std::{
    borrow::Cow,
    collections::HashMap,
    net::SocketAddr,
    ops::{ControlFlow, Deref, DerefMut},
    sync::Arc,
};

use crate::{
    entities::messages::Message as MessageEntity,
    errors::CustomError,
    repositories::{
        message::MessageRepository, room_member::RoomMemberRepository, user::UserRepository,
    },
    request::Claims,
    views::message::Message as MessageView,
    AppState,
};
use axum::{
    extract::{
        ws::Message,
        ws::{CloseFrame, WebSocket},
        ConnectInfo, Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::broadcast;

pub async fn ws_test_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    println!("`{} connected.", addr.to_string());
    // アップグレードコールバックを返すことで、アップグレード処理を確定します。
    // アドレスなどの追加情報を送信することで、コールバックをカスタマイズすることができます。
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

/// 実際のウェブソケットステートマシン（接続ごとに1つ生成される）
async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    // 物事を開始して応答を取得するためだけに ping を送信する (一部のブラウザーではサポートされていません)
    if let Ok(_) = socket.send(Message::Ping(vec![1, 2, 3])).await {
        println!("Pinged {}...", who);
    } else {
        println!("Could not send ping {}!", who);
        // 接続を閉じるしかないので、ここでErrorはない。
        // メッセージを送信できない場合、いずれにせよstatemachineを救済する方法はない。
        return;
    }

    // クライアントから単一のメッセージを受信します (ソケットを使用して受信または送信できます)。
    // これはおそらく、PingのPongか、クライアントからのhelloメッセージになります。
    // クライアントからのメッセージを待つと、
    // このタスクはブロックされるが、他のクライアントの接続はブロックされない。
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg, who).is_break() {
                return;
            }
        } else {
            println!("client {} abruptly disconnected", who);
            return;
        }
    }

    // 各クライアントは個別のステートマシンを取得するため、
    // 必要に応じて処理を一時停止し、
    // 外部イベント（この場合はスリープで図示）を待つことができる。
    // このクライアントが挨拶文を取得し終わるのを待っても、
    // 他のクライアントがサーバーに接続して挨拶文を取得することは妨げられない。
    for i in 1..5 {
        if socket
            .send(Message::Text(String::from(format!("Hi {} times!", i))))
            .await
            .is_err()
        {
            println!("client {} abruptly disconnected", who);
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    // ソケットを分割することで、送信と受信を同時に行うことができます。
    // この例では、サーバーの内部イベント(例えばタイマー)に基づいて、
    // クライアントに未承諾のメッセージを送信する予定です。
    let (mut sender, mut receiver) = socket.split();

    // クライアントにいくつかのメッセージをプッシュするタスクを起動する (クライアントが何をするかは関係ない)
    let mut send_task = tokio::spawn(async move {
        let n_msg = 20;
        for i in 0..n_msg {
            // ウェブソケットエラーが発生した場合は、終了します。
            if sender
                .send(Message::Text(format!("Server message {} ...", i)))
                .await
                .is_err()
            {
                return i;
            }

            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }

        println!("Sending close to {}...", who);
        if let Err(e) = sender
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Cow::from("Goodbye"),
            })))
            .await
        {
            println!("Could not send Close due to {}, probably it is ok?", e);
        }
        n_msg
    });

    // この2番目のタスクは、クライアントからのメッセージを受信し、サーバーコンソールに表示します。
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            // プリントメッセージと指示された場合のブレーク
            if process_message(msg, who).is_break() {
                break;
            }
        }
        cnt
    });

    // いずれかのタスクが終了した場合、他のタスクを中止する。
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => println!("{} messages sent to {}", a, who),
                Err(a) => println!("Error sending messages {:?}", a)
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => println!("Received {} messages", b),
                Err(b) => println!("Error receiving messages {:?}", b)
            }
            send_task.abort();
        }
    }

    // ハンドラから戻ると、ウェブソケット接続が閉じられます。
    println!("Websocket context {} destroyed", who);
}

/// メッセージの内容を標準出力に出力するためのヘルパーです。Closeに対して特別な処理を行う。
fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> {} sent str: {:?}", who, t);
        }
        Message::Binary(d) => {
            println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {} somehow sent close message without CloseFrame", who);
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> {} sent pong with {:?}", who, v);
        }
        // Message::Ping を手動で処理する必要はありません。
        // axum の websocket ライブラリが Pong で応答し、
        // 仕様に従って v をコピーすることで自動的に処理するためです。
        // ただし、ping の内容が必要な場合は、ここで確認できます。
        Message::Ping(v) => {
            println!(">>> {} sent ping with {:?}", who, v);
        }
    }
    ControlFlow::Continue(())
}

/// 部屋名をキー、ウェブソケットデータを値として含むハッシュマップ。
#[derive(Default, Debug)]
pub struct WsRooms(HashMap<i32, broadcast::Sender<WsMessage>>);

// WsRooms構造体のインスタンスからHashMapのメソッドを呼び出すために必要
impl Deref for WsRooms {
    type Target = HashMap<i32, broadcast::Sender<WsMessage>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WsRooms {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub async fn rooms_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(user_id): Path<i32>,
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

pub async fn messages_handler(
    ws: WebSocketUpgrade,
    Path((user_id, room_id)): Path<(i32, i32)>,
    claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError> {
    if user_id != claims.user_id {
        return Err(CustomError::AccessingUnauthorisedResources);
    }

    // リクエストを送信したユーザーがroom_idのルームのメンバーかチェックする
    let room_members = state.room_member_repository.find_members(room_id).await;
    let is_room_member = room_members.iter().any(|x| x.member_id == claims.user_id);

    if !is_room_member {
        return Err(CustomError::AccessingUnauthorisedResources);
    }

    Ok(ws.on_upgrade(move |socket| {
        handle_socket_for_messages(socket, claims, (user_id, room_id), state)
    }))
}

/// ソケットハンドラ
///
/// # 引数
///
/// - socket : クライアントとサーバー間の通信に使用される構造体。
/// - state : スレッド間で共有されるデータ。
/// - room : ルーム名。
/// - user : 接続されているユーザーの情報。
async fn handle_socket_for_messages(
    socket: WebSocket,
    claims: Claims,
    (user_id, room_id): (i32, i32),
    state: Arc<AppState>,
) {
    // sender: 接続してきたユーザーに対してメッセージを送信するのに使う
    // receiver: 接続してきたユーザーから送信されたメッセージを受信するのに使う
    let (mut sender, mut receiver) = socket.split();

    // room_idのルームに接続しているユーザーに対してメッセージをブロードキャストするためのSenderを取得する
    let tx = {
        // 特定のルームに接続している購読者全体にメッセージを送信するためのSenderを取得
        let mut rooms = state.txs.lock().unwrap();
        match rooms.get(&room_id) {
            Some(v) => v.clone(), // room_idに紐づくSender
            None => {
                // ルームの新規作成
                let (tx, _rx) = broadcast::channel(1000);
                rooms.insert(room_id, tx.clone());
                tx
            }
        }
    };

    // txからSendしたメッセージをrxで受信する
    let mut rx = tx.subscribe();

    let mut send_to_listener_task = tokio::spawn(async move {
        // ブロードキャストされたメッセージをここで受け取る
        while let Ok(ws_message) = rx.recv().await {
            // TODO: アームの中の処理を共通化する
            match ws_message {
                WsMessage::Receive(message) => {
                    if let Ok(msg) = serde_json::to_string(&WsMessage::Receive(message)) {
                        if sender.send(Message::Text(msg)).await.is_err() {
                            break;
                        }
                    }
                }
                WsMessage::OldMessagesRetrieved { messages, user_id } => {
                    if claims.user_id == user_id {
                        if let Ok(msg) = serde_json::to_string(&WsMessage::OldMessagesRetrieved {
                            messages,
                            user_id,
                        }) {
                            // ウェブソケットエラーが発生した場合は、ループを解除する。
                            if sender.send(Message::Text(msg)).await.is_err() {
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    });

    // このタスクは、クライアントからメッセージを受信し、購読者にブロードキャストする。
    let mut recv_from_listener_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if let Ok(msg) = serde_json::from_str(text.as_str()) {
                match msg {
                    // WsMessage::Close => break,
                    // WsMessage::ClientKeepAlive => continue,
                    WsMessage::Send(ws_message) => {
                        let message =
                            MessageEntity::create(room_id, user_id, ws_message.content.clone());
                        // TODO: リポジトリ層でエラーを返すようにしたらエラーハンドリングする
                        let message = state.message_repository.store(&message).await;

                        let user = state
                            .user_repository
                            .find_by_user_id(user_id)
                            .await
                            .unwrap();

                        let message: MessageView = (message, &user).into();
                        let _ = tx.send(WsMessage::Receive(message));
                    }
                    WsMessage::RequestOldMessages => {
                        // 新しく接続してきたクライアントに対して、過去のメッセージをDBから取得して送信する
                        // TODO: handlers/messages.rs のget_messagesと処理が重複しているのでサービスに切り出す
                        let messages = state.message_repository.find_by_room_id(room_id).await;
                        let user_ids = messages
                            .iter()
                            .map(|message| message.user_id)
                            .collect::<Vec<i32>>();
                        let users = state.user_repository.find(&user_ids).await;
                        let messages = messages
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
                            .collect::<Vec<MessageView>>();

                        let _ = tx.send(WsMessage::OldMessagesRetrieved {
                            messages,
                            user_id, // 送信先のuser_id
                        });
                    }
                    // WsMessage::Seen(messages) => {
                    //     if let Err(e) =
                    //         WsMessageContent::mark_as_seen(&messages, &state.pg_pool).await
                    //     {
                    //         tracing::error!(
                    //             "An error happened while updating the messsages : {:?}.",
                    //             e
                    //         );
                    //     } else {
                    //         let _ = tx.send(
                    //             serde_json::to_string(&WsMessage::MessagesSeen(messages)).unwrap(),
                    //         );
                    //     }
                    // }
                    _ => {}
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_to_listener_task) => recv_from_listener_task.abort(),
        _ = (&mut recv_from_listener_task) => send_to_listener_task.abort(),
    };
}

/// ユーザー間で共有されるメッセージの種類。
///
/// WSメッセージの中には、部屋の全員に送信しなければならないデータ（つまり送信）や、
/// クライアント側（OldMessagesRetrieved）
/// またはサーバー側（RequestOldMessages）で実行すべきアクションを含むものがあります。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum WsMessage {
    /// アプリケーションの全契約者間で共有されるコンテンツ。
    ///
    /// { "send": { "content": "メッセージの内容" } } を送信する
    Send(MessagePayload),
    /// クライアント側で表示されるコンテンツ。
    Receive(MessageView),
    /// クライアントがルームの全メッセージを取得するために送信するアクション。
    ///
    /// 接続してきたクライアントが過去のメッセージを取得する際に使用する
    ///
    /// { "retrieveMessages": null } を送信する
    RequestOldMessages,
    /// すべてのメッセージが取得され、クライアントに送信されたことを知らせるために、サーバーから送信される情報。
    OldMessagesRetrieved {
        messages: Vec<MessageView>,
        user_id: i32,
    },
    /// ユーザーがメッセージを見たことを示す。
    MessagesSeen(Vec<i32>),
    /// 当事者の一方が接続を終了させたいと考えていることを意識してください。
    Close,
    /// 接続を閉じないように生かす。
    ClientKeepAlive,
    /// メッセージを見たことを知らせる。
    Seen(Vec<i32>),
}

/// ユーザーから受け取るメッセージの型
#[derive(
    Debug, Clone, Serialize, Deserialize, derivative::Derivative, PartialEq, Eq, Hash, sqlx::FromRow,
)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct MessagePayload {
    /// メッセージの識別子で、一意でなければならない。
    // #[derivative(Default(value = "Uuid::new_v4()"))]
    // pub uuid: Uuid,
    /// メッセージの作成者。
    ///
    /// メッセージがサーバーから発信された場合は空です。
    // #[sqlx(flatten)]
    // pub author: PartialUser,
    /// メッセージが発信されたとき。
    // #[derivative(Default(value = "chrono::offset::Utc::now()"))]
    // pub timestamp: DateTime<Utc>,
    /// メッセージが発信された部屋。
    // pub room: String,
    /// メッセージを受信したかどうか。
    // pub reception_status: WsReceptionStatus,

    /// メッセージの内容です。
    pub content: String,
}

// impl MessagePayload {
//     fn new(content: String) -> Self {
//         Self { content }
//     }
// }
