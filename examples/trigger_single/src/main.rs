extern crate novu_rust;
use novu_rust::{
    models::{SubscriberPayloadDto, TriggerEventRequestDto, TriggerEventRequestDtoActor},
    Novu, TriggerPayload,
};
#[async_std::main]
async fn main() {
    let novu = Novu::new(std::env::var("API_KEY").unwrap(), None).unwrap();

    let result = novu
        .trigger(TriggerPayload {
            name: "testing".into(),
            to: SubscriberPayloadDto {
                first_name: (Some("Test".to_string())),
                email: None,
                last_name: None,
                phone: None,
                avatar: None,
                locale: None,
                data: None,
            },
        })
        .await;

    match result {
        Ok(event) => {
            dbg!(&event);
        }
        Err(api_error) => println!("An error occurred: {}", api_error),
    }
}
