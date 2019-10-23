use crate::page_context::{PageContext, PageContent};
use rocket_contrib::templates::Template;

#[get("/blog")]
pub fn blog() -> Template {
    Template::render("blog", &PageContext {
        title: "Lyzde - Blog".to_string(),
        stylesheet: "blog.css".to_string(),
        content: PageContent::Blog(crate::controller::blog::get_blog_list()),
    })
}
