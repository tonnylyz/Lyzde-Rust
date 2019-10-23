use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum PageContent {
    Nil,
    Index(IndexItem),
    Blog(Vec<BlogListItem>),
    Article(ArticleItem),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageContext {
    pub title: String,
    pub stylesheet: String,
    pub content: PageContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkItem {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexSkillItem {
    pub title: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub links: Vec<LinkItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexItem {
    pub introduction: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlogListItem {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub datetime: DateTime<Local>,
    pub tag: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleItem {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub datetime: DateTime<Local>,
    pub tag: Vec<String>,
    pub content: String,
}