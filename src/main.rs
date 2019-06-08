#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate mysql;
#[macro_use]
extern crate lazy_static;

extern crate chrono;

mod page;
mod page_context;
mod config;

use std::env;

use rocket::config::{Config, Environment};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use crate::page_context::{PageContext, PageContent};

#[catch(404)]
fn not_found() -> Template {
    Template::render("error", &PageContext {
        title: "Lyzde".to_string(),
        stylesheet: "error.css".to_string(),
        content: PageContent::Nil,
    })
}

fn main() {
    println!("MySQL connection string: {}", config::conn());
    let static_dir = format!("{}/static", env::current_dir().unwrap().display());
    println!("Static directory: {}", static_dir);
    println!("Listen port: {}", config::port());

    let config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(config::port())
        .unwrap();

    rocket::custom(config)
        .mount("/static", StaticFiles::from(static_dir))
        .mount("/", page::routes())
        .attach(Template::fairing())
        .register(catchers![not_found])
        .launch();
}
