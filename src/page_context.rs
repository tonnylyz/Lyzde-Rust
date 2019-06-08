use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum PageContent {
    Nil,
    Blog(Vec<BlogListItem>),
    Article(ArticleItem),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageContext {
    pub title: String,
    pub stylesheet: String,
    pub content: PageContent,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlogListItem {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub datetime: NaiveDateTime,
    pub tag: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleItem {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub datetime: NaiveDateTime,
    pub tag: Vec<String>,
    pub content: String,
}