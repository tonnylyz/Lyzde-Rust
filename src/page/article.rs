use crate::page_context::{PageContext, PageContent, ArticleItem};
use rocket_contrib::templates::Template;

#[get("/article?<id>")]
pub fn article(id: u32) -> Option<Template> {
    let pool = mysql::Pool::new(crate::config::conn()).unwrap();
    let post: Option<ArticleItem> = pool.first_exec("SELECT id, title, description, datetime, tag, content FROM article WHERE id = :id", params! {"id" => id})
        .map(|result| {
            match result {
                Some(row) => {
                    let (id, title, description, datetime, tag, content) = mysql::from_row(row);
                    let tag_string: String = tag;
                    let tags = tag_string.split('|').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
                    let content_md: String = content;
                    Some(ArticleItem {
                        id,
                        title,
                        description,
                        datetime,
                        tag: tags,
                        content: content_md,
                    })
                }
                None => None
            }
        }).unwrap();

    match post {
        None => None,
        Some(post) => Some(Template::render("article", &PageContext {
            title: String::from("Lyzde - ") + &post.title,
            stylesheet: String::from("article.css"),
            content: PageContent::Article(post),
        }))
    }
}
