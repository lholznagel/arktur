use api::message::Message;
use guards::DBConnection;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::Json;

#[post("/", format = "application/json", data = "<message>")]
pub fn new(db: DBConnection, message: Json<Message>) -> status::Custom<&'static str> {
    let result = message.0.is_valid_hash();

    if result {
        status::Custom(Status::Ok, "")
    } else {
        info!("Bad hash!");
        status::Custom(Status::BadRequest, "")
    }
}