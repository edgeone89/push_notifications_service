use futures_core::stream::Stream;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};
//use std::time::Duration;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
//use std::ops::Deref;

mod pushnotificationsservice;
use pushnotificationsservice::push_notifications_server::{
    PushNotifications, PushNotificationsServer,
};
use pushnotificationsservice::{
    PushNotificationRequest, PushNotificationResponce, SubscribePushNotificationRequest,
    SubscribePushNotificationResponce, UnSubscribePushNotificationRequest,
    UnSubscribePushNotificationResponce,
};

const PUSH_NOTIFICATION_SERVER_ADDRESS: &str = "192.168.0.100:50052";

struct MsgFromUser {
    from_user_id: String,
    msg: String,
    from_user_name: String
}
pub struct DropReceiver<'a, T> {
    inner_rx: Receiver<T>,
    user_id: String,
    push_notification_service: Arc<std::sync::Mutex<&'a mut HabPushNotification>>
}
impl<'a, T> Stream for DropReceiver<'a, T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        return Pin::new(&mut self.inner_rx).poll_recv(cx);
    }
}
/*impl<T> Deref for DropReceiver<T> {
    type Target = Receiver<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner_rx
    }
}*/
impl<'a, T> Drop for DropReceiver<'a, T> {
    fn drop(&mut self) {
        println!("REcEiVER {} has BEEN DROPPED", &self.user_id);
        let user_id = self.user_id.clone();
        if let Ok(push_notification_service) = self.push_notification_service.lock() {
            //use futures::executor;
            use futures_executor::block_on;
            block_on(async {
                let subscribed_peers = &mut (*(push_notification_service.subscribed_peers.write().await));
                subscribed_peers.remove(&user_id);
            });
        }
    }
}

#[derive(Default)]
pub struct HabPushNotification {
    subscribed_peers: Arc<RwLock<HashMap<String, Sender<Result<SubscribePushNotificationResponce, Status>>>>>,
    pending_messages: Arc<Mutex<HashMap<String, VecDeque<MsgFromUser>>>>
}

fn extend_lifetime<'short_lifetime>(ref_hab_push: &'short_lifetime mut HabPushNotification) -> &'static mut HabPushNotification {
    return unsafe {
        std::mem::transmute::<&'short_lifetime mut HabPushNotification, &'static mut HabPushNotification>(ref_hab_push)
    };
}

#[tonic::async_trait]
impl PushNotifications for HabPushNotification {
    //type SubscribeToPushNotificationsStream=mpsc::Receiver<Result<SubscribePushNotificationResponce,Status>>;
    type SubscribeToPushNotificationsStream =
        DropReceiver<'static, Result<SubscribePushNotificationResponce, Status>>;
    async fn subscribe_to_push_notifications(
        &mut self,
        request: Request<SubscribePushNotificationRequest>,
    ) -> Result<Response<Self::SubscribeToPushNotificationsStream>, Status> 
    {
        let user_id_from_request = request.get_ref().user_id.clone();
        println!("new subscriber, id: {}", &user_id_from_request);
        let (tx, rx) = mpsc::channel(1000);

        {
            let subscribed_peers = &mut (*(self.subscribed_peers.write().await));
            subscribed_peers
                .entry(user_id_from_request.clone())
                .or_insert(tx.clone());
        }

        if self.pending_messages.lock().await.contains_key(&user_id_from_request) == true
        {
            let mut messages_awaited = self.pending_messages.lock().await;
            let messages = messages_awaited.get_mut(&user_id_from_request);
            if let Some(msgs) = messages {
                while msgs.len() > 0 {
                    let message = msgs.pop_front();
                    if let Some(msg) = message {
                        let reply = SubscribePushNotificationResponce {
                            from_user_id: msg.from_user_id,
                            message: msg.msg,
                            from_user_name: msg.from_user_name
                        };
                        let tx_tmp = tx.clone();
                        tokio::spawn(async move {
                            let res = tx_tmp.send(Ok(reply)).await;
                            if let Err(e) = res {
                                println!("error while sending queued messages: {}", e);
                            }
                        });
                    }
                }
            }
        }

        let drop_receiver = DropReceiver {
            inner_rx: rx,
            user_id: user_id_from_request.clone(),
            push_notification_service: Arc::new(std::sync::Mutex::new(extend_lifetime(self)))
        };
        return Ok(Response::new(drop_receiver));
    }

    async fn send_push_notification(
        &mut self,
        request: Request<PushNotificationRequest>,
    ) -> Result<Response<PushNotificationResponce>, Status> 
    {
        let from_user_id_from_request = request.get_ref().user_id.clone();
        let to_user_id_from_request = request.get_ref().to_user_id.clone();
        let message_from_request = request.get_ref().message.clone();
        let from_user_name_from_request = request.get_ref().from_user_name.clone();

        let subscribed_peers = &(*(self.subscribed_peers.read().await));
        if subscribed_peers.contains_key(&to_user_id_from_request) == true {
            let reply = SubscribePushNotificationResponce {
                from_user_id: from_user_id_from_request.clone(),
                message: message_from_request.clone(),
                from_user_name: from_user_name_from_request.clone()
            };
            let tx_tmp_option = subscribed_peers.get(&to_user_id_from_request);
            if let Some(tx_tmp_ref) = tx_tmp_option {
                let tx_tmp = tx_tmp_ref.clone();
                tokio::spawn(async move {
                    let res = tx_tmp.send(Ok(reply)).await;
                    if let Err(e) = res {
                        println!("error while send_push_notification: {}", e);
                    }
                });
            } else {
                if self.pending_messages.lock().await.contains_key(&to_user_id_from_request) == true
                {
                    let mut messages_awaited = self.pending_messages.lock().await;
                    let messages = messages_awaited.get_mut(&to_user_id_from_request);
                    if let Some(msgs) = messages {
                        let msg_from_user = MsgFromUser {
                            from_user_id: from_user_id_from_request.clone(),
                            msg: message_from_request,
                            from_user_name: from_user_name_from_request.clone()
                        };
                        msgs.push_back(msg_from_user); // fixme: insertion order is not kept
                    }
                } else {
                    let mut messages: VecDeque<MsgFromUser> = VecDeque::new();
                    let msg_from_user = MsgFromUser {
                        from_user_id: from_user_id_from_request,
                        msg: message_from_request,
                        from_user_name: from_user_name_from_request.clone()
                    };
                    messages.push_back(msg_from_user);
                    self.pending_messages
                        .lock()
                        .await
                        .insert(to_user_id_from_request, messages);
                }
            }
        } else {
            if self.pending_messages.lock().await.contains_key(&to_user_id_from_request) == true
            {
                let mut messages_awaited = self.pending_messages.lock().await;
                let messages = messages_awaited.get_mut(&to_user_id_from_request);
                if let Some(msgs) = messages {
                    let msg_from_user = MsgFromUser {
                        from_user_id: from_user_id_from_request,
                        msg: message_from_request,
                        from_user_name: from_user_name_from_request.clone()
                    };
                    msgs.push_back(msg_from_user); // fixme: insertion order is not kept
                }
            } else {
                let mut messages: VecDeque<MsgFromUser> = VecDeque::new();
                let msg_from_user = MsgFromUser {
                    from_user_id: from_user_id_from_request,
                    msg: message_from_request,
                    from_user_name: from_user_name_from_request.clone()
                };
                messages.push_back(msg_from_user);
                self.pending_messages
                    .lock()
                    .await
                    .insert(to_user_id_from_request, messages);
            }
        }

        let push_notification_responce = PushNotificationResponce { response_code: 1 };
        return Ok(Response::new(push_notification_responce));
    }

    async fn un_subscribe_push_notification(
        &mut self,
        request: Request<UnSubscribePushNotificationRequest>,
    ) -> Result<Response<UnSubscribePushNotificationResponce>, Status>
    {
        println!("unsubscribed");
        let user_id_from_request = request.get_ref().user_id.clone();
        {
            let subscribed_peers = &mut (*(self.subscribed_peers.write().await));
            subscribed_peers.remove(&user_id_from_request);
        }
        let response = UnSubscribePushNotificationResponce {};
        return Ok(Response::new(response));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = PUSH_NOTIFICATION_SERVER_ADDRESS.parse()?;
    let push_notification_service = HabPushNotification::default();

    println!("PushNotificationsServer listening on {}", addr);

    Server::builder()
        .add_service(PushNotificationsServer::new(push_notification_service))
        .serve(addr)
        .await?;

    return Ok(());
}