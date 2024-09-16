//! # YouTube Scraper Module
//!
//! This module provides functionality to scrape and parse stream data from YouTube,
//! specifically tailored for the Parliament of Kenya Channel.
//!
//! ## Key Components
//!
//! - `Stream`: A struct representing a single YouTube stream.
//! - `parse_streams`: A function to parse multiple streams from YouTube JSON data.
//! - `extract_json_from_script`: A function to extract the `ytInitialData` JSON object from a YouTube page's HTML.

use bunge_bits_datastore::Stream;
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::error::Error;

/// Parses multiple streams from the provided JSON data.
///
/// # Parameters
/// * `json`: A reference to a `Value` containing the YouTube page's JSON data.
///
/// # Returns
/// * `Ok(Vec<Stream>)` containing all successfully parsed streams.
/// * `Err(YtScrapeError)` if the JSON structure is unexpected or parsing fails.
pub fn parse_streams(json: &Value) -> Result<Vec<Stream>, Error> {
    let mut streams = Vec::new();

    if let Some(contents) = json["contents"]["twoColumnBrowseResultsRenderer"]["tabs"]
        .get(2)
        .and_then(|tab| tab["tabRenderer"]["content"]["richGridRenderer"]["contents"].as_array())
    {
        for item in contents {
            if let Some(video_renderer) =
                item["richItemRenderer"]["content"]["videoRenderer"].as_object()
            {
                let StreamWrapper(stream) = StreamWrapper::try_from(video_renderer)?;
                streams.push(stream);
            }
        }
    } else {
        return Err(Error::ParseError(
            "Failed to get script contents, structure might have changed",
        ));
    }

    Ok(streams)
}

#[derive(Debug)]
struct StreamWrapper(Stream);

impl TryFrom<&Map<String, Value>> for StreamWrapper {
    type Error = Error;

    /// Attempts to create a `Stream` from a JSON object.
    ///
    /// # Parameters
    /// * `video_renderer`: A reference to a `Map<String, Value>` containing the video data.
    ///
    /// # Returns
    /// * `Ok(Stream)` if parsing is successful.
    /// * `Err(YtScrapeError)` if any required field is missing or cannot be parsed.
    fn try_from(video_renderer: &Map<String, Value>) -> Result<Self, Self::Error> {
        let video_id = video_renderer["videoId"].as_str().unwrap_or_default();
        let title =
            video_renderer["title"]["runs"][0]["text"]
                .as_str()
                .ok_or(Error::ParseError(
                    "Failed to get video title via ['title']['runs'][0]['text']",
                ))?;
        let view_count = video_renderer["viewCountText"]["simpleText"]
            .as_str()
            .ok_or(Error::ParseError(
                "Failed to get video view count via ['viewCountText']['simpleText']",
            ))?;
        let streamed_date = video_renderer["publishedTimeText"]["simpleText"]
            .as_str()
            .ok_or(Error::ParseError(
                "Failed to get streamed_date via ['publishedTimeText']['simpleText']",
            ))?;
        let duration =
            video_renderer["lengthText"]["simpleText"]
                .as_str()
                .ok_or(Error::ParseError(
                    "Failed to get duration via ['lengthText']['simpleText']",
                ))?;

        let stream = Stream {
            video_id: video_id.to_string(),
            title: title.to_string(),
            view_count: view_count.to_string(),
            streamed_date: streamed_date.to_string(),
            duration: duration.to_string(),
        };

        Ok(StreamWrapper(stream))
    }
}

/// Extracts the `ytInitialData` JSON object from a YouTube page's HTML script.
///
/// # Context
/// YouTube dynamically loads much of its page content using JavaScript. The initial
/// data for the page, including video information, is embedded in the HTML as a
/// JavaScript variable named `ytInitialData`. This function extracts that data,
/// allowing us to access it without executing JavaScript.
///
/// # How it works
/// 1. Uses a regular expression to find the `ytInitialData` variable assignment in the script.
/// 2. Extracts the JSON string from within that assignment.
/// 3. Parses the extracted string into a Serde JSON Value.
///
/// # Parameters
/// * `document`: The entire HTML content of the YouTube page as a string.
///
/// # Returns
/// * `Option<T>`: Some(T) if the JSON was successfully extracted and parsed,
///                    None if the JSON couldn't be found or parsed.
///
/// # Note
/// This method is somewhat fragile as it depends on the specific structure of YouTube's
/// HTML. If YouTube changes how they embed this data, this function may need to be updated.
pub fn extract_json_from_script<T: for<'a> Deserialize<'a>>(document: &str) -> Result<T, Error> {
    let re =
        regex::Regex::new(r"(?s)<script[^>]*>\s*var\s+ytInitialData\s*=\s*(\{.*?\});\s*</script>")
            .unwrap();
    let result = re
        .captures(document)
        .and_then(|cap| cap.get(1))
        .and_then(|m| serde_json::from_str(m.as_str()).ok())
        .ok_or(Error::ParseError(
            "Failed to extract ytInitialData from the page's script tag",
        ));

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_successful_extraction() {
        let html_content = r#"
            <html>
                <head>
                    <script nonce="gZTn8MILMQFuWon1rDk2VA">
                        var ytInitialData = {"key": "value", "number": 42};
                    </script>
                </head>
                <body>
                    <p>Some content</p>
                </body>
            </html>
        "#;

        let result = extract_json_from_script::<Value>(html_content);
        println!("Extraction result: {:?}", result);
        assert!(result.is_ok(), "Failed to extract JSON: {:?}", result.err());
        let json = result.unwrap();
        assert_eq!(json, json!({"key": "value", "number": 42}));
    }

    #[test]
    fn test_extraction_with_special_characters() {
        let html_content_special = r#"
            <script nonce="gZTn8MILMQFuWon1rDk2VA">
                var ytInitialData = {
                    "key": "value with \"quotes\" and \n newline"
                };
            </script>
        "#;

        let result = extract_json_from_script::<Value>(html_content_special);
        println!("Extraction result: {:?}", result);
        assert!(
            result.is_ok(),
            "Failed to extract JSON with special characters: {:?}",
            result.err()
        );
        let json = result.unwrap();
        assert_eq!(
            json["key"].as_str().unwrap().trim(),
            "value with \"quotes\" and \n newline"
        );
    }

    #[test]
    fn test_extraction_with_multiple_occurrences() {
        let html_content_multiple = r#"
            <script nonce="gZTn8MILMQFuWon1rDk2VA">var ytInitialData = {"first": true};</script>
            <script nonce="gZTn8MILMQFuWon1rDk2VA">
                var ytInitialData = {"second": true};
            </script>
        "#;

        let result = extract_json_from_script::<Value>(html_content_multiple);
        println!("Extraction result: {:?}", result);
        assert!(
            result.is_ok(),
            "Failed to extract first JSON: {:?}",
            result.err()
        );
        let json = result.unwrap();
        assert_eq!(json, json!({"first": true}));
    }

    #[test]
    fn test_extraction_with_no_data() {
        let html_content_no_data = r#"
            <html>
                <body>
                    <p>No ytInitialData here</p>
                </body>
            </html>
        "#;

        let result = extract_json_from_script::<Value>(html_content_no_data);
        println!("Extraction result: {:?}", result);
        assert!(result.is_err(), "Expected an error, but got: {:?}", result);
        assert!(matches!(result, Err(Error::ParseError(_))));
    }

    #[test]
    fn test_extraction_with_invalid_json() {
        let html_content_invalid_json = r#"
            <script nonce="gZTn8MILMQFuWon1rDk2VA">
                var ytInitialData = {invalid: json};
            </script>
        "#;

        let result = extract_json_from_script::<Value>(html_content_invalid_json);
        println!("Extraction result: {:?}", result);
        assert!(result.is_err(), "Expected an error, but got: {:?}", result);
        assert!(matches!(result, Err(Error::ParseError(_))));
    }
}
