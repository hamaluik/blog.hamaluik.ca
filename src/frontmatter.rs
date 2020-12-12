use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RawFrontMatter {
    pub title: String,
    pub slug: String,
    pub tags: Vec<String>,
    pub published: Option<String>,
    pub summary: String,
    pub section: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct FrontMatter {
    pub title: String,
    pub slug: String,
    pub tags: Vec<String>,
    pub date: DateTime<Utc>,
    pub summary: String,
    pub section: String,
}

impl From<RawFrontMatter> for Option<FrontMatter> {
    fn from(raw: RawFrontMatter) -> Option<FrontMatter> {
        #[allow(unused)]
        let RawFrontMatter {
            title,
            slug,
            tags,
            published,
            summary,
            section,
        } = raw;
        if published.is_none() {
            return None;
        }

        let date: DateTime<FixedOffset> = match DateTime::parse_from_rfc3339(
            published.as_ref().unwrap(),
        ) {
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
            summary,
            section: section.unwrap_or("Miscellaneous".to_owned()),
        })
    }
}
