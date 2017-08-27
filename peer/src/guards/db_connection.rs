use r2d2;
use r2d2_postgres::PostgresConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

type Pool = r2d2::Pool<PostgresConnectionManager>;

pub struct DBConnection(pub r2d2::PooledConnection<PostgresConnectionManager>);

impl<'a, 'r> FromRequest<'a, 'r> for DBConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DBConnection, ()> {
        let pool = request.guard::<State<Pool>>()?;

        match pool.get() {
            Ok(connection) => Outcome::Success(DBConnection(connection)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}