use crate::page_context::{PageContext, PageContent, ArticleItem};
use rocket_contrib::templates::Template;
use postgres::{Connection, TlsMode};

#[get("/article?<id>")]
pub fn article(id: i32) -> Option<Template> {
    let conn = Connection::connect(crate::config::conn(), TlsMode::None).unwrap();
    let r = conn.query("SELECT id, title, description, datetime, tag, content FROM article WHERE id = $1", &[ &id ]).unwrap();
    if r.is_empty() {
        return None;
    }
    let row = r.get(0);
    let (id, title, description, datetime, tag, content) : (_, String, _, _, String, _) = (
        row.get(0),
        row.get(1),
        row.get(2),
        row.get(3),
        row.get(4),
        row.get(5),
        );
    let tags = tag.split('|').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    Some(Template::render("article", &PageContext {
        title: "Lyzde - ".to_string() + &title,
        stylesheet: "article.css".to_string(),
        content: PageContent::Article(ArticleItem {
            id,
            title,
            description,
            datetime,
            tag: tags,
            content,
        }),
    }))
}
