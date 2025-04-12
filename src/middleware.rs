use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, Ready};
use log::{info, warn};
use std::time::Instant;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Logger;

impl<S, B> Transform<S, ServiceRequest> for Logger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggerMiddleware { service })
    }
}

pub struct LoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let path = req.path().to_string();
        let method = req.method().to_string();
        let _headers = req.headers().clone();

        let fut = self.service.call(req);
        
        Box::pin(async move {
            let res = fut.await?;
            let elapsed = start.elapsed();
            
            let status = res.status();
            let status_code = status.as_u16();
            
            // Log request details
            info!(
                "{} {} {} {}ms",
                method,
                path,
                status_code,
                elapsed.as_millis()
            );
            
            // Log warning for 4xx and 5xx responses
            if status_code >= 400 {
                warn!(
                    "Error response: {} {} {} {}ms",
                    method,
                    path,
                    status_code,
                    elapsed.as_millis()
                );
            }
            
            Ok(res)
        })
    }
} 