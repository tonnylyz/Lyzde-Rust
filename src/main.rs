#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate mysql;
#[macro_use]
extern crate lazy_static;

extern crate chrono;

extern crate comrak;

mod page_context;

use crate::page_context::{PageContext, PageContent, BlogListItem, PostItem};

mod config;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    Template::render("index", &PageContext {
        title: "Lyzde".to_string(),
        stylesheet: "index.css".to_string(),
        content: PageContent::Nil,
    })
}

#[get("/blog")]
fn blog() -> Template {
    let pool = mysql::Pool::new(&**config::CONN_STR).unwrap();
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

#[get("/post?<id>")]
fn post(id: u32) -> Option<Template> {
    let pool = mysql::Pool::new(&**config::CONN_STR).unwrap();
    let post: Option<PostItem> = pool.first_exec("SELECT id, title, description, datetime, tag, content FROM article WHERE id = :id", params! {"id" => id})
        .map(|result| {
            match result {
                Some(row) => {
                    let (id, title, description, datetime, tag, content) = mysql::from_row(row);
                    let tag_string: String = tag;
                    let tags = tag_string.split('|').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
                    let content_md: String = content;
                    let content_html: String =comrak::markdown_to_html(content_md.as_ref(), &comrak::ComrakOptions::default());
                    Some(PostItem {
                        id,
                        title,
                        description,
                        datetime,
                        tag: tags,
                        content: content_html,
                    })
                }
                None => None
            }
        }).unwrap();

    match post {
        None => None,
        Some(post) => Some(Template::render("post", &PageContext {
            title: "Lyzde - ".to_owned() + &post.title,
            stylesheet: "article.css".to_string(),
            content: PageContent::Post(post),
        }))
    }
}

#[catch(404)]
fn not_found() -> Template {
    Template::render("error", &PageContext {
        title: "Lyzde".to_string(),
        stylesheet: "error.css".to_string(),
        content: PageContent::Nil,
    })
}

fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .mount("/", routes![index, blog, post])
        .attach(Template::fairing())
        .register(catchers![not_found])
        .launch();
}
