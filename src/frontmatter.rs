use serde::Deserialize;
use chrono::prelude::*;

#[derive(Debug, Deserialize)]
pub struct RawFrontMatter {
    pub title: String,
    pub slug: String,
    pub tags: Vec<String>,
    pub published: Option<String>,
    #[serde(rename = "meta-image")]
    pub meta_image: Option<String>,
    #[serde(rename = "")]
    pub large_meta_image: Option<bool>,
    #[serde(rename = "preview-image")]
    pub preview_image: Option<String>,
    #[serde(rename = "preview-summary")]
    pub preview_summary: Option<String>,
}

pub struct FrontMatter {
    pub title: String,
    pub slug: String,
    pub tags: Vec<String>,
    pub date: DateTime<Utc>,
}

impl From<RawFrontMatter> for Option<FrontMatter> {
    fn from(raw: RawFrontMatter) -> Option<FrontMatter> {
        #[allow(unused)]
        let RawFrontMatter {
            title, 
            slug, 
            tags, 
            published, 
            meta_image, 
            large_meta_image, 
            preview_image, 
            preview_summary, 
        } = raw;
        if published.is_none() { return None; }
        
        let date: DateTime<FixedOffset> = match DateTime::parse_from_rfc3339(published.as_ref().unwrap()) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("unexpected published date format `{}` for slug `{}` (expected %Y-%m-%dT%H:%M:%S%z): {:?}", published.unwrap(), slug, e);
                return None;
            }
        };
        let date: DateTime<Utc> = date.into();

        Some(FrontMatter {
            title,
            slug,
            tags,
            date,
        })
    }
}
