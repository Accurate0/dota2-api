use async_trait::async_trait;
use http::Response;
use lambda_http::{Body, Request};
use simple_dispatcher::{Executor, ExecutorResult};

use crate::context::Context;

pub struct Fallback;

#[async_trait]
impl Executor<Context, Request, Response<Body>> for Fallback {
    async fn execute(&self, _ctx: &Context, _request: &Request) -> ExecutorResult<Response<Body>> {
        Ok(Response::builder().status(404).body(Body::Empty)?)
    }
}
