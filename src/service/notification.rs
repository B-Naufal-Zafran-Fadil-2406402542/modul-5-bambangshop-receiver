use std::thread;
use crate::model::notification::Notification;
use crate::model::subscriber::SubscriberRequest;
use crate::repository::notification::NotificationRepository;
use crate::{Result, APP_INSTANCE_NAME, APP_INSTANCE_ROOT_URL, APP_PUBLISHER_ROOT_URL, REQWEST_CLIENT};

pub struct NotificationService;

impl NotificationService {
    #[rocket::tokio::main]
    async fn subscribe_request(product_type: String) -> Result<SubscriberRequest> {
        let payload = SubscriberRequest {
            url: format!("{}/notification/receive", *APP_INSTANCE_ROOT_URL),
            name: APP_INSTANCE_NAME.clone(),
        };
        let response = REQWEST_CLIENT
            .post(format!("{}/notification/subscribe/{}", *APP_PUBLISHER_ROOT_URL, product_type))
            .json(&payload)
            .send()
            .await?;
        return Ok(response.json().await?);
    }

    pub fn subscribe(product_type: &str) -> Result<SubscriberRequest> {
        let product_type_clone = String::from(product_type);
        return thread::spawn(move || Self::subscribe_request(product_type_clone)).join().unwrap();
    }

    #[rocket::tokio::main]
    async fn unsubscribe_request(product_type: String) -> Result<SubscriberRequest> {
        let url = format!("{}/notification/receive", *APP_INSTANCE_ROOT_URL);
        let response = REQWEST_CLIENT
            .post(format!("{}/notification/unsubscribe/{}?url={}", *APP_PUBLISHER_ROOT_URL, product_type, url))
            .send()
            .await?;
        return Ok(response.json().await?);
    }

    pub fn unsubscribe(product_type: &str) -> Result<SubscriberRequest> {
        let product_type_clone = String::from(product_type);
        return thread::spawn(move || Self::unsubscribe_request(product_type_clone)).join().unwrap();
    }

    pub fn receive_notification(payload: Notification) -> Result<Notification> {
        return Ok(NotificationRepository::add(payload));
    }

    pub fn list_messages() -> Vec<String> {
        return NotificationRepository::list_all_as_string();
    }
}
