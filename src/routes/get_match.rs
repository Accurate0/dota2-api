use crate::{constants::DOTA2_GET_MATCHES, context::Context, types::Dota2Match};
use async_trait::async_trait;
use http::Response;
use lambda_http::{Body, IntoResponse, Request, RequestExt};
use simple_dispatcher::{Executor, ExecutorResult};
use util::{
    client::get_http_client, constants::CORRELATION_ID_HEADER, extensions::RequestExtensions,
};

pub struct GetMatch;

#[async_trait]
impl Executor<Context, Request, Response<Body>> for GetMatch {
    async fn execute(
        &self,
        context: &Context,
        request: &Request,
    ) -> ExecutorResult<Response<Body>> {
        let path_params = request.path_parameters();
        let match_id = path_params.first("matchId").expect("must have id");
        let correlation_id = request.get_correlation_id();
        let http_client = get_http_client();

        let response = http_client
            .get(DOTA2_GET_MATCHES)
            .header(CORRELATION_ID_HEADER, correlation_id)
            .query(&[("match_id", match_id)])
            .query(&[("key", context.config.steam_api_key.as_str())])
            .send()
            .await?;

        Ok(serde_json::to_value(response.json::<Dota2Match>().await?)?.into_response())
    }
}
