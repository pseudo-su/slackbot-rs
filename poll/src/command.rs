use serde::{Deserialize, Serialize};
use std::error::Error;
use http::header::CONTENT_TYPE;
use lambda_http::{lambda, IntoResponse, Request, Response, Body};
// use lambda_http::body::{Body};
use lambda_runtime::{error::HandlerError, Context};
use simple_logger;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(handler);

    Ok(())
}

#[derive(Serialize)]
pub struct Poll {
    text: String,
}

impl IntoResponse for Poll {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .header(CONTENT_TYPE, "application/json")
            .body(
                serde_json::to_string(&self)
                    .expect("unable to serialize serde_json::Value")
                    .into(),
            )
            .unwrap()
    }
}

struct CommandEvent {
    command: String,
    user_id: String,
    team_id: String,
    channel_id: String,
    text: String,
}

impl From<Request> for CommandEvent {
    fn from(req: Request) -> Self {
        CommandEvent{
            command: "".to_owned(),
            user_id: "".to_owned(),
            team_id: "".to_owned(),
            channel_id: "".to_owned(),
            text: "".to_owned() 
        }
    }
}

fn handler(req: Request, c: Context) -> Result<impl IntoResponse, HandlerError> {
    // 1. parse input string (e.text)
    // 2. create a poll object from input str
    // 3. Persist poll state
    // 4. return formatted poll JSON payload to user

    let ev: CommandEvent = req.into();
    let poll = Poll{ text: ev.text };

    Ok(poll)

    // Ok(match e.query_string_parameters().get("first_name") {
    //     Some(first_name) => format!("Hello, {}!", first_name).into_response(),
    //     _ => {
    //         error!("Empty first name in request {}", c.aws_request_id);
    //         Response::builder()
    //             .status(400)
    //             .body("Empty first name".into())
    //             .expect("failed to render response")
    //     }
    // })
}
