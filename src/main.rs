use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::collections::VecDeque;

mod pushnotificationsservice;
use pushnotificationsservice::push_notifications_server::{PushNotifications, PushNotificationsServer};
use pushnotificationsservice::{SubscribePushNotificationRequest, SubscribePushNotificationResponce, 
    PushNotificationRequest, PushNotificationResponce
};

struct MsgFromUser{
    from_user_id: String,
    msg: String
}

#[derive(Default)]
pub struct HabPushNotification {
    subscribed_peers: HashMap<String, Sender<Result<SubscribePushNotificationResponce, Status>>>,
    pending_messages: Arc<Mutex<HashMap<String, VecDeque<MsgFromUser>>>>
}

#[tonic::async_trait]
impl PushNotifications for HabPushNotification{
    type SubscribeToPushNotificationsStream=mpsc::Receiver<Result<SubscribePushNotificationResponce,Status>>;
    async fn subscribe_to_push_notifications(
        &mut self,
        request: Request<SubscribePushNotificationRequest>,
    ) -> Result<Response<Self::SubscribeToPushNotificationsStream>, Status>{
println!("new subscriber");
        let user_id_from_request = request.get_ref().user_id.clone();
        let (tx, rx) = mpsc::channel(10);
        self.subscribed_peers.entry(user_id_from_request.clone()).or_insert(tx.clone());

        if self.pending_messages.lock().await.contains_key(&user_id_from_request) == true {
            let mut messages_awaited = self.pending_messages.lock().await;
            let messages = messages_awaited.get_mut(&user_id_from_request);
            if let Some(msgs) = messages {
                while msgs.len() > 0 {
                    let message = msgs.pop_front();
                    if let Some(msg) = message {
                        let reply = SubscribePushNotificationResponce {
                            from_user_id: msg.from_user_id,
                            message: msg.msg
                        };
                        let mut tx_tmp = tx.clone();
                        tokio::spawn(async move {
                            tx_tmp.send(Ok(reply)).await;
                        });
                    }
                }
            }
        }

        return Ok(Response::new(rx));
    }
    
    async fn send_push_notification(
        &mut self,
        request: Request<PushNotificationRequest>,
    ) -> Result<Response<PushNotificationResponce>, Status>{

        let user_id_from_request = request.get_ref().user_id.clone();
        let to_user_id_from_request = request.get_ref().to_user_id.clone();
        let message_from_request = request.get_ref().message.clone();

        if self.subscribed_peers.contains_key(&to_user_id_from_request) == true {
            let reply = SubscribePushNotificationResponce {
                from_user_id: user_id_from_request,
                message: message_from_request
            };
            let tx_tmp_option = self.subscribed_peers.get(&to_user_id_from_request);
            if let Some(tx_tmp_ref) = tx_tmp_option {
                let mut tx_tmp = tx_tmp_ref.clone();
                tokio::spawn(async move {
                    tx_tmp.send(Ok(reply)).await;
                });
            }
        } else {
            if self.pending_messages.lock().await.contains_key(&to_user_id_from_request) == true {
                let mut messages_awaited = self.pending_messages.lock().await;
                let messages = messages_awaited.get_mut(&user_id_from_request);
                if let Some(msgs) = messages {
                    let msg_from_user = MsgFromUser{
                        from_user_id: user_id_from_request,
                        msg: message_from_request
                    };
                    msgs.push_back(msg_from_user);// fixme: insertion order is not kept
                }
            } else {
                let mut messages:VecDeque<MsgFromUser> = VecDeque::new();
                let msg_from_user = MsgFromUser{
                    from_user_id: user_id_from_request,
                    msg: message_from_request
                };
                messages.push_back(msg_from_user);
                self.pending_messages.lock().await.insert(to_user_id_from_request, messages);
            }
        }

        let push_notification_responce = PushNotificationResponce{
            response_code: 1
        };
        return Ok(Response::new(push_notification_responce));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "192.168.0.100:50052".parse()?;
    let pushnotificationsservice = HabPushNotification::default();

    println!("ChatServer listening on {}", addr);

    Server::builder()
        .add_service(PushNotificationsServer::new(pushnotificationsservice))
        .serve(addr)
        .await?;

    Ok(())
}