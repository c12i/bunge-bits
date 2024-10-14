//! # Yt Types
//!
//! This module contains type definitions for working with the `ytInitialData` object.
//!
//! It provides a small sub-set of structs that matches its structure, allowing for easy
//! deserialization and manipulation of video data.
//!
//! ## Note on Optional Fields
//!
//! Many fields in these structs are wrapped in `Option<T>`. This is because the
//! YouTube API doesn't always return all fields for every request. Using `Option`
//! allows our types to handle cases where certain fields are missing from the response.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoRenderer {
    #[serde(rename = "videoId")]
    pub video_id: String,
    pub thumbnail: Thumbnail,
    pub title: TextRuns,
    #[serde(rename = "publishedTimeText")]
    pub published_time_text: Option<SimpleText>,
    #[serde(rename = "viewCountText")]
    pub view_count_text: Option<SimpleText>,
    #[serde(rename = "lengthText")]
    pub length_text: Option<AccessibilityText>,
    #[serde(rename = "upcomingEventData")]
    pub upcoming_event_data: Option<UpcomingEventData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextRuns {
    pub runs: Vec<TextRun>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextRun {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SimpleText {
    #[serde(rename = "simpleText")]
    pub simple_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityText {
    pub accessibility: Accessibility,
    #[serde(rename = "simpleText")]
    pub simple_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accessibility {
    #[serde(rename = "accessibilityData")]
    pub accessibility_data: AccessibilityData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityData {
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnail {
    pub thumbnails: Vec<ThumbnailItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThumbnailItem {
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpcomingEventData {
    #[serde(rename = "isReminderSet")]
    pub is_reminder_set: bool,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "upcomingEventText")]
    pub upcoming_event_text: TextRuns,
}
