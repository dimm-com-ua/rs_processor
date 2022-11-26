mod application;
mod adapters;
mod infrastructure;

use rocket::fs::FileServer;
use crate::adapters::api::tools;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut _rocket = rocket::build()
        .mount("/static", FileServer::from("static"));

    _rocket = tools::heath_check::routes(_rocket);

    let _ = _rocket.launch().await?;

    Ok(())
}
