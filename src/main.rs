use std::collections::HashMap;
use std::{sync::Mutex, path::{Path, PathBuf}};
use rocket::serde::json::Json;

use rand::{distributions::Alphanumeric, Rng};

use rocket::{State, response::{Redirect}};
use rocket::fs::NamedFile;
use serde::{Serialize};


struct Store {
    links: Mutex<HashMap<String, String>>,
}

#[derive(Serialize)]
struct ReturnData {
    short: String,
}

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Redirect {
    Redirect::permanent("/index.html")
}

#[get("/<file..>")]
async fn build_dir(file: PathBuf) -> Option<NamedFile> {
    println!("📁 Serving {}", Path::new("static/").join(&file).display());
    NamedFile::open(Path::new("static/").join(file)).await.ok()
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
fn shorten(store_db: &State<Store>, url: &str) -> Json<ReturnData> {
    let shared_data = store_db.inner();

    // Generate a random slug for the url
    let short: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    println!("🔗 Shortening {} to {}", url, short);

    // Insert the slug+url into the hashmap
    shared_data.links.lock().unwrap().insert(short.clone(), url.to_string());
    
    Json(ReturnData {
        short
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Store {
            links: Mutex::new(HashMap::new()),
        })
        .mount("/", routes![index, build_dir, error_page, shorten, get])
}
