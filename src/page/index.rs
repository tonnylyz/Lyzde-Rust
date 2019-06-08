use crate::page_context::{PageContext, PageContent};
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index() -> Template {
    Template::render("index", &PageContext {
        title: "Lyzde".to_string(),
        stylesheet: "index.css".to_string(),
        content: PageContent::Nil,
    })
}
