use rocket::Route;

mod article;
mod blog;
mod index;

pub fn routes() -> Vec<Route> {
    routes![
        article::article,
        index::index,
        blog::blog,
    ]
}
