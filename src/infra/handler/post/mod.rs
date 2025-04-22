use warp::Filter;
use super::models::post::Post;

pub async fn get_post() -> Result<impl warp::Reply, warp::Rejection> {
}

// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Clone)]
// pub struct Post {
//     pub id: i32,
//     pub title: String,
//     pub description: String,
//     pub readable_publish_date: String,
//     pub slug: String,
//     pub path: String,
//     pub url: String,
//     pub comments_count: i32,
//     pub public_reactions_count: i32,
//     pub collection_id: Option<i32>,
//     pub published_timestamp: String,
//     pub positive_reactions_count: i32,
//     pub cover_image: Option<String>,
//     pub social_image: String,
//     pub canonical_url: String,
//     pub created_at: String,
//     pub edited_at: Option<String>,
//     pub crossposted_at: Option<String>,
//     pub published_at: String,
//     pub last_comment_at: String,
//     pub reading_time_minutes: i32,
//     pub tag_list: Vec<String>,
//     pub tags: String,
// }