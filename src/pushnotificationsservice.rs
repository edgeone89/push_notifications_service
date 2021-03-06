#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribePushNotificationRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribePushNotificationResponce {
    #[prost(string, tag="1")]
    pub from_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub from_user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushNotificationRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub to_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub from_user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushNotificationResponce {
    #[prost(int32, tag="1")]
    pub response_code: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnSubscribePushNotificationRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnSubscribePushNotificationResponce {
}
/// Generated client implementations.
pub mod push_notifications_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct PushNotificationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PushNotificationsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
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
        T::Error: Into<StdError>,
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PushNotificationsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PushNotificationsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn subscribe_to_push_notifications(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribePushNotificationRequest>,
        ) -> Result<
                tonic::Response<
                    tonic::codec::Streaming<super::SubscribePushNotificationResponce>,
                >,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pushnotificationsservice.PushNotifications/SubscribeToPushNotifications",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn send_push_notification(
            &mut self,
            request: impl tonic::IntoRequest<super::PushNotificationRequest>,
        ) -> Result<tonic::Response<super::PushNotificationResponce>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        pub async fn un_subscribe_push_notification(
            &mut self,
            request: impl tonic::IntoRequest<super::UnSubscribePushNotificationRequest>,
        ) -> Result<
                tonic::Response<super::UnSubscribePushNotificationResponce>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pushnotificationsservice.PushNotifications/UnSubscribePushNotification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod push_notifications_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tokio::sync::Mutex;
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with PushNotificationsServer.
    #[async_trait]
    pub trait PushNotifications: Send + Sync + 'static {
        ///Server streaming response type for the SubscribeToPushNotifications method.
        type SubscribeToPushNotificationsStream: futures_core::Stream<
                Item = Result<super::SubscribePushNotificationResponce, tonic::Status>,
            >
            + Send
            + 'static;
        async fn subscribe_to_push_notifications(
            &mut self,
            request: tonic::Request<super::SubscribePushNotificationRequest>,
        ) -> Result<
                tonic::Response<Self::SubscribeToPushNotificationsStream>,
                tonic::Status,
            >;
        async fn send_push_notification(
            &mut self,
            request: tonic::Request<super::PushNotificationRequest>,
        ) -> Result<tonic::Response<super::PushNotificationResponce>, tonic::Status>;
        async fn un_subscribe_push_notification(
            &mut self,
            request: tonic::Request<super::UnSubscribePushNotificationRequest>,
        ) -> Result<
                tonic::Response<super::UnSubscribePushNotificationResponce>,
                tonic::Status,
            >;
    }
    #[derive(Debug)]
    pub struct PushNotificationsServer<T: PushNotifications> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<Mutex<T>>);
    impl<T: PushNotifications> PushNotificationsServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(Mutex::new(inner)))
        }
        pub fn from_arc(inner: Arc<Mutex<T>>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PushNotificationsServer<T>
    where
        T: PushNotifications,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pushnotificationsservice.PushNotifications/SubscribeToPushNotifications" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeToPushNotificationsSvc<T: PushNotifications>(
                        pub Arc<Mutex<T>>,
                    );
                    impl<
                        T: PushNotifications,
                    > tonic::server::ServerStreamingService<
                        super::SubscribePushNotificationRequest,
                    > for SubscribeToPushNotificationsSvc<T> {
                        type Response = super::SubscribePushNotificationResponce;
                        type ResponseStream = T::SubscribeToPushNotificationsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribePushNotificationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.subscribe_to_push_notifications(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeToPushNotificationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pushnotificationsservice.PushNotifications/SendPushNotification" => {
                    #[allow(non_camel_case_types)]
                    struct SendPushNotificationSvc<T: PushNotifications>(pub Arc<Mutex<T>>);
                    impl<
                        T: PushNotifications,
                    > tonic::server::UnaryService<super::PushNotificationRequest>
                    for SendPushNotificationSvc<T> {
                        type Response = super::PushNotificationResponce;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
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
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendPushNotificationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pushnotificationsservice.PushNotifications/UnSubscribePushNotification" => {
                    #[allow(non_camel_case_types)]
                    struct UnSubscribePushNotificationSvc<T: PushNotifications>(
                        pub Arc<Mutex<T>>,
                    );
                    impl<
                        T: PushNotifications,
                    > tonic::server::UnaryService<
                        super::UnSubscribePushNotificationRequest,
                    > for UnSubscribePushNotificationSvc<T> {
                        type Response = super::UnSubscribePushNotificationResponce;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnSubscribePushNotificationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.un_subscribe_push_notification(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnSubscribePushNotificationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PushNotifications> Clone for PushNotificationsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: PushNotifications> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PushNotifications> tonic::transport::NamedService
    for PushNotificationsServer<T> {
        const NAME: &'static str = "pushnotificationsservice.PushNotifications";
    }
}
