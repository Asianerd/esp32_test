use std::{fs, path::Path, sync::Mutex};

use rocket::State;

#[macro_use] extern crate rocket;

mod cors;

#[get("/")]
pub fn index() -> String {
    "can you understand me?".to_string()
}

#[get("/")]
pub fn get_current(w: &State<Mutex<String>>) -> String {
    w.lock().unwrap().to_string()
}

#[get("/<word>")]
pub fn set_current(w: &State<Mutex<String>>, word: String) -> String {
    let mut w = w.lock().unwrap();
    *w = urlencoding::decode(&word).unwrap().to_string();
    w.to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::custom(rocket::config::Config::figment().merge(("port", 9000)))
        .manage(Mutex::new("lorem ipsum".to_string()))
        .mount("/", routes![index])

        .mount("/get_current", routes![get_current])
        .mount("/set_current", routes![set_current])

        .attach(cors::CORS)
}