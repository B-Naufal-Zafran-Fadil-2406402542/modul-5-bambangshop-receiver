pub mod notification;

use rocket::fairing::AdHoc;
use crate::service::notification::NotificationService;

pub fn route_stage() -> AdHoc {
    return AdHoc::on_ignite("Initializing controller routes...", rocket async {
        rocket
            .mount("/notification", routes![notification::subscribe, notification::unsubscribe, notification::receive])
            .mount("/", routes![index])
    });
}

#[get("/")]
pub fn index() -> String {
    let messages = NotificationService::list_messages();
    if messages.is_empty() {
        return String::from("No notifications yet.");
    }
    return messages.join("\n");
}
