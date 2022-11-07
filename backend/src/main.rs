#[macro_use]
extern crate rocket;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
extern crate base64;

mod engine;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test/<position>/<depth>")]
fn test(position: String, depth: u8) -> String {
    let mut pos = "".to_string();
    for i in base64::decode(position.clone()).unwrap() {
        pos.push(i as char);
    }
    engine::minimax_genesis(pos, 3, true)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, test])
        .attach(CORS)
}
