
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribePushNotificationRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribePushNotificationResponce {
    #[prost(string, tag = "1")]
    pub from_user_id: std::string::String,
    #[prost(string, tag = "2")]
    pub message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushNotificationRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    #[prost(string, tag = "2")]
    pub message: std::string::String,
    #[prost(string, tag = "3")]
    pub to_user_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushNotificationResponce {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnSubscribePushNotificationRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnSubscribePushNotificationResponce {}
#[doc = r" Generated client implementations."]
pub mod push_notifications_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct PushNotificationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PushNotificationsClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PushNotificationsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn subscribe_to_push_notifications(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribePushNotificationRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::SubscribePushNotificationResponce>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pushnotificationsservice.PushNotifications/SubscribeToPushNotifications",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn send_push_notification(
            &mut self,
            request: impl tonic::IntoRequest<super::PushNotificationRequest>,
        ) -> Result<tonic::Response<super::PushNotificationResponce>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pushnotificationsservice.PushNotifications/SendPushNotification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PushNotificationsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PushNotificationsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PushNotificationsClient {{ ... }}")
        }
    }
}

#[doc = r" Generated server implementations."]
pub mod push_notifications_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    use tokio::sync::Mutex;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PushNotificationsServer."]
    #[async_trait]
    pub trait PushNotifications: Send + Sync + 'static {
        #[doc = "Server streaming response type for the SubscribeToPushNotifications method."]
        type SubscribeToPushNotificationsStream: Stream<Item = Result<super::SubscribePushNotificationResponce, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn subscribe_to_push_notifications(
            &mut self,
            request: tonic::Request<super::SubscribePushNotificationRequest>,
        ) -> Result<tonic::Response<Self::SubscribeToPushNotificationsStream>, tonic::Status>;
        async fn send_push_notification(
            &mut self,
            request: tonic::Request<super::PushNotificationRequest>,
        ) -> Result<tonic::Response<super::PushNotificationResponce>, tonic::Status>;
        async fn un_subscribe_push_notification(
            &mut self,
            request: tonic::Request<super::UnSubscribePushNotificationRequest>,
        ) -> Result<tonic::Response<super::UnSubscribePushNotificationResponce>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct PushNotificationsServer<T: PushNotifications> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<Mutex<T>>, Option<tonic::Interceptor>);
    impl<T: PushNotifications> PushNotificationsServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(Mutex::new(inner));
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(Mutex::new(inner));
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for PushNotificationsServer<T>
    where
        T: PushNotifications,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pushnotificationsservice.PushNotifications/SubscribeToPushNotifications" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeToPushNotificationsSvc<T: PushNotifications>(pub Arc<Mutex<T>>);
                    impl<T: PushNotifications>
                        tonic::server::ServerStreamingService<
                            super::SubscribePushNotificationRequest,
                        > for SubscribeToPushNotificationsSvc<T>
                    {
                        type Response = super::SubscribePushNotificationResponce;
                        type ResponseStream = T::SubscribeToPushNotificationsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribePushNotificationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { 
                                    let mut tmp_inner = inner.lock().await;
                                    tmp_inner.subscribe_to_push_notifications(request).await
                                };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = SubscribeToPushNotificationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pushnotificationsservice.PushNotifications/SendPushNotification" => {
                    #[allow(non_camel_case_types)]
                    struct SendPushNotificationSvc<T: PushNotifications>(pub Arc<Mutex<T>>);
                    impl<T: PushNotifications>
                        tonic::server::UnaryService<super::PushNotificationRequest>
                        for SendPushNotificationSvc<T>
                    {
                        type Response = super::PushNotificationResponce;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PushNotificationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.send_push_notification(request).await
                             };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendPushNotificationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pushnotificationsservice.PushNotifications/UnSubscribePushNotification" => {
                    #[allow(non_camel_case_types)]
                    struct UnSubscribePushNotificationSvc<T: PushNotifications>(pub Arc<Mutex<T>>);
                    impl<T: PushNotifications>
                        tonic::server::UnaryService<super::UnSubscribePushNotificationRequest>
                        for UnSubscribePushNotificationSvc<T>
                    {
                        type Response = super::UnSubscribePushNotificationResponce;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnSubscribePushNotificationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { 
                                    let mut tmp_inner = inner.lock().await;
                                    tmp_inner.un_subscribe_push_notification(request).await
                                };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UnSubscribePushNotificationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: PushNotifications> Clone for PushNotificationsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: PushNotifications> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PushNotifications> tonic::transport::NamedService for PushNotificationsServer<T> {
        const NAME: &'static str = "pushnotificationsservice.PushNotifications";
    }
}
