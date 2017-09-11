use peer::{Message, service as PeerService, Register};
use guards::DBConnection;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::Json;

use rocket::response::content;

/// API-Endpoint for getting all registered peers
///
/// Result:
/// ``` json
/// [
///     {
///         "name": "Name of the peer"
///     },
///     {
///         "name": "Name of another peer"
///     }
/// ]
/// ```
#[get("/")]
pub fn list(db: DBConnection) -> status::Custom<content::Json<String>> {
    let result = PeerService::get_all_peers(&db);
    status::Custom(Status::Ok, content::Json(result))
}

/// API-Endpoint for registering a new peer
#[post("/", format = "application/json", data = "<message>")]
pub fn register(
    db: DBConnection,
    message: Json<Message<Register>>,
) -> status::Custom<&'static str> {
    let message = message.0.validate_hash();

    if message.is_valid_hash {
        PeerService::save_peer(&db, &message.content);
        status::Custom(Status::Ok, "")
    } else {
        info!("Bad hash!");
        status::Custom(Status::BadRequest, "")
    }
}