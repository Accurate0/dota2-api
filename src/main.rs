use config::ApiConfig;
use constants::{CONFIG_BUCKET, CONFIG_FILE};
use context::Context;
use http::StatusCode;
use lambda_http::request::RequestContext;
use lambda_http::{service_fn, Body, Error, Request, RequestExt, Response};
use routes::{Fallback, GetMatch};
use simple_dispatcher::RouteDispatcher;
use util::config::load_from_s3;
use util::constants::DEFAULT_AWS_REGION;
use util::extensions::{RequestExtensions, ResponseExtensions};
use util::logging;

mod config;
mod constants;
mod context;
mod routes;
pub mod types;

#[tokio::main]
async fn main() -> Result<(), Error> {
    logging::setup_logging();
    let shared_config = aws_config::from_env()
        .region(DEFAULT_AWS_REGION)
        .load()
        .await;

    let config = load_from_s3::<ApiConfig>(&shared_config, CONFIG_BUCKET, CONFIG_FILE).await?;
    let ref dispatcher =
        RouteDispatcher::new(Context { config }, Fallback).add_route("/match/{matchId}", GetMatch);

    let handler = move |request: Request| async move {
        request.log();
        let response = match dispatcher
            .dispatch(&request, || -> Option<String> {
                let context = request.request_context();
                match context {
                    RequestContext::ApiGatewayV1(r) => r.resource_path,
                    _ => None,
                }
            })
            .await
        {
            Ok(r) => r,
            Err(e) => {
                log::error!("{:?}", e);
                let status_code = StatusCode::INTERNAL_SERVER_ERROR;
                Response::builder()
                    .status(status_code.as_u16())
                    .body(Body::Empty.into())?
            }
        };

        response.log();
        Ok(response)
    };

    lambda_http::run(service_fn(handler)).await?;
    Ok(())
}
