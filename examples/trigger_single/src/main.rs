extern crate novu_rust;
use novu_rust::subscribers::SubscriberPayloadDto;
use novu_rust::Novu;
use novu_rust::TriggerPayload;
#[async_std::main]
async fn main() {
    let novu = Novu::new(std::env::var("API_KEY").unwrap(), None).unwrap();
    let p = novu.subscribers.get("1".to_string()).await;
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
