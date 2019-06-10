use crate::page_context::{BlogListItem};
use chrono::NaiveDateTime;
use rocket_contrib::json::Json;

#[get("/ajax/blog/list")]
pub fn blog_list() -> Json<Vec<BlogListItem>> {
    let pool = mysql::Pool::new(crate::config::conn()).unwrap();
    let blog_list =
        pool.prep_exec("SELECT id, title, description, datetime, tag FROM article ORDER BY id DESC", ())
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, title, description, datetime, tag) = mysql::from_row::<(u32, String, String, NaiveDateTime, String)>(row);
                    let tags = tag.split('|').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
                    BlogListItem {
                        id,
                        title,
                        description,
                        datetime,
                        tag: tags,
                    }
                }).collect::<Vec<_>>()
            }).unwrap();
    Json(blog_list)
}
