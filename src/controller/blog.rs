use crate::page_context::{BlogListItem};
use rocket_contrib::json::Json;
use postgres::{Connection, TlsMode};

pub fn get_blog_list() -> Vec<BlogListItem> {
    let conn = Connection::connect(crate::config::conn(), TlsMode::None).unwrap();
    let r = conn.query("SELECT id, title, description, datetime, tag FROM article ORDER BY id DESC", &[]).unwrap();
    r.iter().map(|row| {
        let (id, title, description, datetime, tag) : (_, _, _, _, String) = (
            row.get(0),
            row.get(1),
            row.get(2),
            row.get(3),
            row.get(4),
        );
        let tags = tag.split('|').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        BlogListItem {
            id,
            title,
            description,
            datetime,
            tag: tags,
        }
    }).collect::<Vec<_>>()
}

#[get("/ajax/blog/list")]
pub fn blog_list() -> Json<Vec<BlogListItem>> {
    Json(get_blog_list())
}
