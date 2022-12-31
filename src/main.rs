use chrono::{Utc};

use serde_json::json;
use uuid::Uuid;
use crate::proto::*;

mod proto;

fn main() {
    // Create a 'Input::Join' enum variant from a JSON object
    let input: Input = serde_json::from_value(json!({
        "type": "join",
        "payload": {
            "name": "Leo",
        },
    }))
        .unwrap();
    assert_eq!(input, Input::Join(JoinInput { name: String::from("Leo") }));
    // Un-serialize as a pretty-printed string
    println!("Input = {}", serde_json::to_string_pretty(&input).unwrap());

    let message_uuid = Uuid::new_v4();
    let user_uuid = Uuid::new_v4();
    let message_created_at = Utc::now();
    // Create a 'Output::UserPosted' enum variant from a JSON object
    let output: Output = serde_json::from_value(json!({
        "type": "user-posted",
        "payload": {
            "message": {
                "id": message_uuid,
                "user": {
                    "id": user_uuid,
                    "name": "Ruben",
                },
                "body": "Hello, there!",
                "createdAt": message_created_at,
            },
        },
    }))
        .unwrap();
    assert_eq!(output, Output::UserPosted(UserPostedOutput::new(MessageOutput::new(
        message_uuid,
        UserOutput::new(user_uuid, "Ruben"),
        "Hello, there!",
        message_created_at,
    ))));
    // Un-serialize as a pretty-printed string
    println!("Output::UserPosted = {}", serde_json::to_string_pretty(&output).unwrap());
}
