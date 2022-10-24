#![feature(decl_macro)]

mod api_response;
mod api_controller;

#[macro_use] extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut _rocket = rocket::build();

    _rocket = api_controller::routes(_rocket);

    let _ = _rocket.launch().await?;

    Ok(())
}
