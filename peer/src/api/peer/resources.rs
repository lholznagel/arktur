use api::peer::{Message, PeerService, Register};
use guards::DBConnection;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::Json;

/// API-Endpoint for registering a new peer
#[post("/", format = "application/json", data = "<message>")]
pub fn register(db: DBConnection, message: Json<Message<Register>>) -> status::Custom<&'static str> {
    //let message = message.0.clone();
    let message = message.0.validate_hash();

    if message.is_valid_hash {
        PeerService::save_peer(&db, &message.content);
        status::Custom(Status::Ok, "")
    } else {
        info!("Bad hash!");
        status::Custom(Status::BadRequest, "")
    }
}