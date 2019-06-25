use crate::page_context::{PageContext, PageContent, IndexItem};
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index() -> Template {
    let index = IndexItem {
        introduction: crate::website_helper::fetch_kv("introduction", None),
    };
    Template::render("index", &PageContext {
        title: "Lyzde".to_string(),
        stylesheet: "index.css".to_string(),
        content: PageContent::Index(index),
    })
}
