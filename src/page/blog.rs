use crate::page_context::{PageContext, PageContent, BlogListItem};
use rocket_contrib::templates::Template;

#[get("/blog")]
pub fn blog() -> Template {
    let pool = mysql::Pool::new(crate::config::conn()).unwrap();
    let blog_list: Vec<BlogListItem> =
        pool.prep_exec("SELECT id, title, description, datetime, tag FROM article ORDER BY id DESC", ())
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, title, description, datetime, tag) = mysql::from_row(row);
                    let tag_string: String = tag;
                    let tags = tag_string.split('|').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
                    BlogListItem {
                        id,
                        title,
                        description,
                        datetime,
                        tag: tags,
                    }
                }).collect()
            }).unwrap();

    Template::render("blog", &PageContext {
        title: "Lyzde - Blog".to_string(),
        stylesheet: "blog.css".to_string(),
        content: PageContent::Blog(blog_list),
    })
}
