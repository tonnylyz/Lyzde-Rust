use rocket::Route;

pub mod blog;

pub fn routes() -> Vec<Route> {
    routes![
        blog::blog_list,
    ]
}