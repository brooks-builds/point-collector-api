use rocket::{
    catch, get,
    http::Status,
    post,
    response::{
        self,
        status::{self, BadRequest, Custom},
        Responder,
    },
    serde::json::Json,
    Response,
};
use serde::Serialize;

use crate::responses::{Error, ErrorResponse, Points, SuccessResponse};

#[get("/")]
pub async fn index() -> &'static str {
    "Hello, world!!!!!"
}

#[post("/")]
pub async fn create() -> Result<Json<SuccessResponse<Points>>, Error> {
    // return Err(ErrorResponse::create_error("something went horribly wrong"));
    let points = Points { points: 10 };
    // Ok(response.into())
    Ok(Json(SuccessResponse {
        data: points,
        code: 200,
    }))
}
