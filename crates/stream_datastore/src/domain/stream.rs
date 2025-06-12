use std::fmt::Display;

use chrono::{DateTime, Duration, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow, Clone, Default)]
pub struct Stream {
    pub video_id: String,
    pub title: String,
    pub view_count: String,
    pub streamed_date: String,
    pub duration: String,
    pub summary: Option<String>,
}

impl Stream {
    /// Generates a YouTube watch URL for the stream.
    ///
    /// # Returns
    ///
    /// A `String` containing the full URL to watch the stream on YouTube.
    pub fn url(&self) -> String {
        format!("https://www.youtube.com/watch?v={}", self.video_id)
    }

    /// Attempts to parse the `streamed_date` field and convert it to a `DateTime<Utc>`.
    ///
    /// This method interprets strings in the format "X units ago" where units can be
    /// seconds, minutes, hours, days, weeks, months, or years. It then calculates
    /// an approximate timestamp based on the current time.
    ///
    /// # Returns
    ///
    /// - `Some(DateTime<Utc>)` if the `streamed_date` was successfully parsed.
    /// - `None` if the `streamed_date` couldn't be parsed or doesn't match the expected format.
    ///
    /// # Note
    ///
    /// The calculated timestamp is an approximation and may not be exact, especially
    /// for longer time periods like months or years due to varying month lengths and leap years.
    pub fn timestamp_from_time_ago(&self) -> Option<DateTime<Utc>> {
        let now = Utc::now();

        let re =
            regex::Regex::new(r"(\d+)\s+(second|minute|hour|day|week|month|year)s?\s+ago").unwrap();
        if let Some(captures) = re.captures(&self.streamed_date) {
            let amount: i64 = captures[1].parse().unwrap();
            let unit = &captures[2];

            let duration = match unit {
                "second" => Duration::seconds(amount),
                "minute" => Duration::minutes(amount),
                "hour" => Duration::hours(amount),
                "day" => Duration::days(amount),
                "week" => Duration::weeks(amount),
                "month" => Duration::days(amount * 30), // Approximation
                "year" => Duration::days(amount * 365), // Approximation
                _ => return None,
            };

            Some(now - duration)
        } else {
            None
        }
    }

    /// Attempts to determine the StreamCategory from a given title.
    ///
    /// This function searches for specific keywords in the title to identify
    /// the appropriate StreamCategory.
    pub fn category(&self) -> Option<StreamCategory> {
        if self.title.to_lowercase().contains("national assembly") {
            return Some(StreamCategory::NationalAssembly);
        }
        if self.title.to_lowercase().contains("senate") {
            return Some(StreamCategory::Senate);
        }
        None
    }
}

#[derive(Debug)]
pub enum StreamCategory {
    NationalAssembly,
    Senate,
}

impl Display for StreamCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamCategory::NationalAssembly => write!(f, "National Assembly"),
            StreamCategory::Senate => write!(f, "Senate"),
        }
    }
}
