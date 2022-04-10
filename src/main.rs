#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod github;
use github::{GitHub, JobCareer};
use rocket::serde::json::{Json};
use rocket::http::{Status};


#[get("/careers")]
async fn careers() -> (Status, Json<Vec<JobCareer>>) {
    println!("Waiting while retrieving careers...");
    if let Ok(careers) = GitHub::careers().await {
        (Status::Ok, Json(careers))
    } else {
        println!("Unable to retrieve careers");
        let resp : Vec<JobCareer> = Vec::new();
        (Status::InternalServerError, Json(resp))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/github", routes![careers])
}
