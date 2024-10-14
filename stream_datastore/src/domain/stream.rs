use std::fmt::Display;

use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Default)]
pub struct Stream {
    pub video_id: String,
    pub title: String,
    pub view_count: String,
    pub streamed_date: String,
    pub duration: String,
    pub closed_captions_summary: String,
}

impl Stream {
    /// Generates a YouTube watch URL for the stream.
    ///
    /// # Returns
    ///
    /// A `String` containing the full URL to watch the stream on YouTube.
    ///
    /// # Example
    ///
    /// ```
    /// use bunge_bits_datastore::Stream;
    ///
    /// let stream = Stream {
    ///     video_id: "dQw4w9WgXcQ".to_string(),
    ///     ..Default::default()
    /// };
    /// assert_eq!(stream.url(), "https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// use bunge_bits_datastore::Stream;
    ///
    /// let stream = Stream {
    ///     streamed_date: "2 hours ago".to_string(),
    ///     ..Default::default()
    /// };
    /// let timestamp = stream.timestamp_from_time_ago();
    /// assert!(timestamp.is_some());
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_stream(streamed_date: &str) -> Stream {
        Stream {
            video_id: "test_id".to_string(),
            title: "Test Stream".to_string(),
            view_count: "1000".to_string(),
            streamed_date: streamed_date.to_string(),
            duration: "1:00:00".to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_url_generation() {
        let stream = create_stream("1 hour ago");
        assert_eq!(stream.url(), "https://www.youtube.com/watch?v=test_id");
    }

    #[test]
    fn test_timestamp_seconds_ago() {
        let stream = create_stream("30 seconds ago");
        let timestamp = stream.timestamp_from_time_ago().unwrap();
        let expected = Utc::now() - Duration::seconds(30);
        assert!((timestamp - expected).num_seconds().abs() < 2);
    }

    #[test]
    fn test_timestamp_minutes_ago() {
        let stream = create_stream("15 minutes ago");
        let timestamp = stream.timestamp_from_time_ago().unwrap();
        let expected = Utc::now() - Duration::minutes(15);
        assert!((timestamp - expected).num_seconds().abs() < 2);
    }

    #[test]
    fn test_timestamp_hours_ago() {
        let stream = create_stream("2 hours ago");
        let timestamp = stream.timestamp_from_time_ago().unwrap();
        let expected = Utc::now() - Duration::hours(2);
        assert!((timestamp - expected).num_seconds().abs() < 2);
    }

    #[test]
    fn test_timestamp_days_ago() {
        let stream = create_stream("3 days ago");
        let timestamp = stream.timestamp_from_time_ago().unwrap();
        let expected = Utc::now() - Duration::days(3);
        assert!((timestamp - expected).num_seconds().abs() < 2);
    }

    #[test]
    fn test_timestamp_weeks_ago() {
        let stream = create_stream("2 weeks ago");
        let timestamp = stream.timestamp_from_time_ago().unwrap();
        let expected = Utc::now() - Duration::weeks(2);
        assert!((timestamp - expected).num_seconds().abs() < 2);
    }

    #[test]
    fn test_timestamp_months_ago() {
        let stream = create_stream("3 months ago");
        let timestamp = stream.timestamp_from_time_ago().unwrap();
        let expected = Utc::now() - Duration::days(3 * 30);
        assert!((timestamp - expected).num_seconds().abs() < 2);
    }

    #[test]
    fn test_timestamp_years_ago() {
        let stream = create_stream("2 years ago");
        let timestamp = stream.timestamp_from_time_ago().unwrap();
        let expected = Utc::now() - Duration::days(2 * 365);
        assert!((timestamp - expected).num_seconds().abs() < 2);
    }

    #[test]
    fn test_timestamp_singular_unit() {
        let stream = create_stream("1 year ago");
        let timestamp = stream.timestamp_from_time_ago().unwrap();
        let expected = Utc::now() - Duration::days(365);
        assert!((timestamp - expected).num_seconds().abs() < 2);
    }

    #[test]
    fn test_timestamp_invalid_format() {
        let stream = create_stream("invalid time ago");
        assert!(stream.timestamp_from_time_ago().is_none());
    }

    #[test]
    fn test_timestamp_empty_string() {
        let stream = create_stream("");
        assert!(stream.timestamp_from_time_ago().is_none());
    }

    #[test]
    fn test_timestamp_future_time() {
        let stream = create_stream("2 hours from now");
        assert!(stream.timestamp_from_time_ago().is_none());
    }
}
