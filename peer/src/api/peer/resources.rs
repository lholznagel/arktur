use api::peer::{Message, PeerService, Register};
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
    let mut is_first: bool = true;
    let mut result: String = String::from("[");

    for row in &db.0
        .query(
            "SELECT name
            FROM peers",
            &[],
        )
        .unwrap()
    {
        if !is_first {
            result.push_str(",");
        }

        let register = Register {
            name: row.get(0)
        };

        result.push_str(register.as_json().as_str());
        is_first = false;
    }

    result.push_str("]");
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