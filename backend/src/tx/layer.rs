use axum::response::IntoResponse;
use bytes::Bytes;
use futures_core::future::BoxFuture;
use http_body::Body;

use super::extension::Extension;
use super::state::State;


pub struct Layer {
    state: State,
}

impl Layer
{
    pub fn new(state: State) -> Self {
        Self {
            state,
        }
    }
}

impl Clone for Layer {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

impl<S> tower_layer::Layer<S> for Layer
{
    type Service = Service<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Service {
            inner,
            state: self.state.clone(),
        }
    }
}

pub struct Service<S> {
    inner: S,
    state: State,
}

impl<S: Clone> Clone for Service<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            state: self.state.clone(),
        }
    }
}

impl<S, ReqBody, ResBody> tower_service::Service<http::Request<ReqBody>> for Service<S>
where 
    S: tower_service::Service<
        http::Request<ReqBody>, 
        Response = http::Response<ResBody>, 
        Error = std::convert::Infallible,
    >,
    S::Future: Send + 'static,
    ResBody: Body<Data = Bytes> + Send + 'static,
    ResBody::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    type Response = http::Response<axum_core::body::Body>;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|err| match err {})
    }

    fn call(&mut self, mut req: http::Request<ReqBody>) -> Self::Future {
        let ext = Extension::new(self.state.clone());
        req.extensions_mut().insert(ext.clone());

        let res = self.inner.call(req);

        Box::pin(async move {
            let res = res.await.unwrap(); // inner service is infallible
            if !res.status().is_server_error() && !res.status().is_client_error() {
                if let Err(error) = ext.resolve().await {
                    return Ok(error.into_response());
                }
            }
            Ok(res.map(axum_core::body::Body::new))
        })

    }

}
