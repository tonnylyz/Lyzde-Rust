#![feature(proc_macro_hygiene, decl_macro, try_trait)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

extern crate chrono;
extern crate postgres;

mod page;
mod controller;
mod page_context;
mod config;

use rocket::config::{Config, Environment};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

fn main() {
    println!("Database connection string: {}", config::conn());
    let static_dir = format!("{}/static", std::env::current_dir().unwrap().display());
    println!("Static directory: {}", static_dir);
    println!("Listen port: {}", config::port());

    let config = Config::build(Environment::Production)
        .address("127.0.0.1")
        .port(config::port())
        .unwrap();

    rocket::custom(config)
        .mount("/static", StaticFiles::from(static_dir))
        .mount("/", [page::routes(), controller::routes()].concat())
        .attach(Template::fairing())
        .launch();
}
