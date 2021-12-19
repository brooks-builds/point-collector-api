use point_collector_api::routes::{create, index};
use rocket::{
    serde::{json::Json, Deserialize},
    Data,
};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1", routes![index, create])
}
