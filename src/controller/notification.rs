use rocket::serde::json::Json;
use crate::model::notification::Notification;
use crate::model::subscriber::SubscriberRequest;
use crate::service::notification::NotificationService;
use crate::Result;

#[post("/subscribe/<product_type>")]
pub fn subscribe(product_type: &str) -> Result<Json<SubscriberRequest>> {
    return match NotificationService::subscribe(product_type) {
        Ok(f) => Ok(Json::from(f)),
        Err(e) => Err(e)
    };
}

#[post("/unsubscribe/<product_type>")]
pub fn unsubscribe(product_type: &str) -> Result<Json<SubscriberRequest>> {
    return match NotificationService::unsubscribe(product_type) {
        Ok(f) => Ok(Json::from(f)),
        Err(e) => Err(e)
    };
}

#[post("/receive", data = "<payload>")]
pub fn receive(payload: Json<Notification>) -> Result<Json<Notification>> {
    return match NotificationService::receive_notification(payload.into_inner()) {
        Ok(f) => Ok(Json::from(f)),
        Err(e) => Err(e)
    };
}
