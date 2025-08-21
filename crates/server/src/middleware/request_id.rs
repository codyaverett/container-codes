use axum::{
    extract::Request,
    http::{header::HeaderValue, HeaderMap},
    middleware::Next,
    response::Response,
};
use tower::{Layer, Service};
use uuid::Uuid;

const REQUEST_ID_HEADER: &str = "x-request-id";

#[derive(Clone)]
pub struct RequestIdLayer;

impl RequestIdLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdService { inner }
    }
}

#[derive(Clone)]
pub struct RequestIdService<S> {
    inner: S,
}

impl<S> Service<Request> for RequestIdService<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request) -> Self::Future {
        let request_id = request
            .headers()
            .get(REQUEST_ID_HEADER)
            .and_then(|v| v.to_str().ok())
            .unwrap_or_else(|| {
                let id = Uuid::new_v4().to_string();
                request.headers_mut().insert(
                    REQUEST_ID_HEADER,
                    HeaderValue::from_str(&id).unwrap(),
                );
                // This is a bit of a hack to get the string back out
                request.headers().get(REQUEST_ID_HEADER).unwrap().to_str().unwrap()
            })
            .to_string();

        let future = self.inner.call(request);

        Box::pin(async move {
            let mut response = future.await?;
            
            // Add request ID to response headers
            response.headers_mut().insert(
                REQUEST_ID_HEADER,
                HeaderValue::from_str(&request_id).unwrap(),
            );

            Ok(response)
        })
    }
}