//! Defines Actix web middleware for the application.
//! Currently includes a logging middleware.

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
use actix_web::http::header::{USER_AGENT, CONTENT_TYPE};

/// Actix middleware factory for logging requests and responses.
///
/// Logs information such as the request method, path, response status,
/// duration, User-Agent, and Content-Type.
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

/// The actual logging middleware service.
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
        let headers = req.headers().clone();

        let user_agent = headers.get(USER_AGENT).map_or("-".to_string(), |h| h.to_str().unwrap_or("-").to_string());
        let content_type = headers.get(CONTENT_TYPE).map_or("-".to_string(), |h| h.to_str().unwrap_or("-").to_string());

        let fut = self.service.call(req);
        
        Box::pin(async move {
            let res = fut.await?;
            let elapsed = start.elapsed();
            
            let status = res.status();
            let status_code = status.as_u16();
            
            info!(
                "{} {} - Status: {}, Duration: {}ms, User-Agent: '{}', Content-Type: '{}'",
                method,
                path,
                status_code,
                elapsed.as_millis(),
                user_agent,
                content_type
            );
            
            if status_code >= 400 {
                warn!(
                    "Error response: {} {} - Status: {}, Duration: {}ms",
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