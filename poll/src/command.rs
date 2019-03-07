use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;

fn main() {
    lambda!(handler)
}

fn handler(_: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    
    Ok(
        json!({
        "response_type": "in_channel",
        "replace_original": false,
        "text": "Hello Slack! attachment",
        "attachments": [
            {
                "callback_id": "1234",
                "text": "test",
                "fallback": "attachments not supported",
                "actions": [{
                    "name": "poll-action",
                    "type": "button",
                    "text": "Delete",
                    "style": "danger",
                    "value": "delete",
                    // "confirm": {
                    //     "title": "Delete Poll?",
                    //     "text": "This poll will be completely removed, are you sure?",
                    //     "ok_text": "Yes",
                    //     "dismiss_text": "No",
                    // },
                }],
            }
            
        ]
    }))
}
