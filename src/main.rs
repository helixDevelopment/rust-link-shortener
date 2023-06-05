use std::collections::HashMap;
use std::sync::Mutex;

use rand::{distributions::Alphanumeric, Rng};

use rocket::{State, response::Redirect};


struct Store {
    links: Mutex<HashMap<String, String>>,
}

#[macro_use]
extern crate rocket;

#[get("/")]
fn home() -> String {
    format!("Hello world!")
}

#[get("/404")]
fn error_page() -> String {
    format!("404 This page doesn't exist")
}

#[get("/get/<slug>")]
fn get(store_db: &State<Store>, slug: &str) -> Redirect {
    let links = &store_db.links.lock().expect("lock shared data");

    println!("🔎 Looking for {}", slug);

    // Check if the slug exists in the hashmap
    match links.get(slug) {
        Some(url) => {
            println!("🔀 Redirecting to {}", url);
            Redirect::to(format!("https://{}", url.replace("https://", "")))
        }
        None => {
            println!("❌ Slug not found");
            Redirect::to("/404")
        },
    }
}

#[get("/shorten/<url>")]
fn shorten(store_db: &State<Store>, url: &str) -> String {
    let shared_data = store_db.inner();

    // Generate a random slug for the url
    let short: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    println!("🔗 Shortening {} to {}", url, short);
    println!("✅ New url: http://localhost:8000/get/{}", &short);

    // Insert the slug+url into the hashmap
    match shared_data.links.lock().unwrap().insert(short.clone(), url.to_string()) {
        Some(_) => {
            format!("New url: http://localhost:8000/get/{}", short)
        },
        None => {
            format!("New url: http://localhost:8000/get/{}", short)
        },
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Store {
            links: Mutex::new(HashMap::new()),
        })
        .mount("/", routes![home, error_page, shorten, get])
}
