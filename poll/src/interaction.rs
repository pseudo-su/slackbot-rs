use serde::{Deserialize, Serialize};
use std::error::Error;

use lambda_runtime::{error::HandlerError, lambda, Context};
// use log::{error};
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

#[derive(Deserialize)]
struct SlackInteractions {
    name: String,
    value: String,
}

#[derive(Deserialize)]
struct InteractionEvent {
    #[serde(rename = "actions")]
    interactions: Vec<SlackInteractions>,
}

fn handler(
    e: InteractionEvent, _: Context
) -> Result<Poll, HandlerError> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    
    let poll = Poll{ text: "'Do you like the thing?' Y N".to_owned() };

    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    Ok(poll)
}
