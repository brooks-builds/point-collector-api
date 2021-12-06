use rocket::{
    serde::{json::Json, Deserialize},
    Data,
};

#[macro_use]
extern crate rocket;

#[derive(Deserialize, Debug)]
struct TwitchMessage {
    helix_token: String,
    channel_id: String,
    client_id: String,
    user_id: String,
}

#[post("/", data = "<twitch_message>")]
async fn index(twitch_message: Json<TwitchMessage>) -> &'static str {
    // let result = reqwest::blocking::Client::new().get("https://api.twitch.tv/helix/users?id=75574155").header("Authorization", "Extension eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJvcGFxdWVfdXNlcl9pZCI6IlViUnp5MGZER2prT0F5a1hFbExiWCIsImNsaWVudF9pZCI6ImcyY25xODRsZWVmYTNrNnVmYW95dWhuc3prNzFnZiIsImNyZWF0ZWRfYXQiOjE2Mzg3NTk0MTF9.KG8zDFXxgNQPo8lBxBf6yc2BR_CPscqSopbZVYTNIx8").header("Client-Id", "g2cnq84leefa3k6ufaoyuhnszk71gf").send().unwrap();
    let result = reqwest::Client::new()
        .get("https://api.twitch.tv/helix/users?id=id of user")
        .header("Authorization", "Extension put extension token here")
        .header("Client-Id", "put client id here")
        .send()
        .await
        .unwrap();
    dbg!(twitch_message, result);
    "Hello, world!!!!!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
