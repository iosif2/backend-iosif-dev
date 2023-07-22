#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

mod database;
mod models;
mod schema;
mod controller;
mod birdcontroller;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![
        birdcontroller::index,
        birdcontroller::new_sighting,
        birdcontroller::all_sightings,
        birdcontroller::delete_sighting,
        controller::health_checker,
        ])
}
