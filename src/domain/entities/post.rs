use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Post {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub readable_publish_date: Option<String>,
    pub slug: Option<String>,
    pub path: Option<String>,
    pub url: Option<String>,
    pub comments_count: Option<i32>,
    pub public_reactions_count: Option<i32>,
    pub collection_id: Option<i32>,
    pub published_timestamp: Option<String>,
    pub positive_reactions_count: Option<i32>,
    pub cover_image: Option<String>,
    pub social_image: Option<String>,
    pub canonical_url: Option<String>,
    pub created_at: Option<String>,
    pub edited_at: Option<String>,
    pub crossposted_at: Option<String>,
    pub published_at: Option<String>,
    pub last_comment_at: Option<String>,
    pub reading_time_minutes: Option<i32>,
    pub tag_list: Option<Vec<String>>,
    pub tags: Option<String>,
}
