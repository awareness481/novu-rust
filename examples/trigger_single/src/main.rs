extern crate novu_rust;
use novu_rust::{
    models::{SubscriberPayloadDto, TriggerEventRequestDto, TriggerEventRequestDtoActor},
    Novu, TriggerPayload,
};
use std::{collections::HashMap, env};
#[async_std::main]
async fn main() {
    let novu = Novu::new("5608b1568795f64ee5c39d79d2479dca", None).unwrap();

    let result = novu
        .trigger(TriggerPayload {
            name: "testing".into(),
            to: SubscriberPayloadDto::new("Test".into()),
        })
        .await;

    match result {
        Ok(event) => {
            dbg!(&event);
        }
        Err(api_error) => println!("An error occurred: {}", api_error),
    }
}
