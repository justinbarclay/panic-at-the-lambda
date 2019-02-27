use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use std::collections::HashMap;
use lambda_runtime::{lambda, Context, error::HandlerError};
use serde_json::json;

mod reddit;
use reddit::get_top_aww_post;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  lambda!(cuteness_handler);
  Ok(())
}
fn default_gateway_response(body: Option<String>) -> ApiGatewayProxyResponse{
  let mut headers = HashMap::new();
  headers.insert("Content-Type".to_string(), "application/json".to_string());
  ApiGatewayProxyResponse {
    status_code: 200,
    multi_value_headers: HashMap::new(),
    headers,
    body,
    is_base64_encoded: Some(true),
  }
}
fn cuteness_handler(_request: ApiGatewayProxyRequest, _ctx: Context) -> Result<ApiGatewayProxyResponse, HandlerError> {
  match get_top_aww_post("https://www.reddit.com/r/aww/top/.json?limit=10"){
    Ok(message) => {
      Ok(default_gateway_response(Some(json!(message).to_string())))},
    Err(error) => {
      Ok(default_gateway_response(Some(json!({"error": error}).to_string())))
    }
  }
}
