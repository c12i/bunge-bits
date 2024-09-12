use serde_json::{Map, Value};

use crate::error::YtScrapeError;

#[derive(Debug)]
pub struct Stream {
    pub id: String,
    pub title: String,
    pub view_count: String,
    pub streamed_date: String,
    pub duration: String,
}

impl Stream {
    pub fn url(&self) -> String {
        format!("https://www.youtube.com/watch?v={}", self.id)
    }
}

impl TryFrom<&Map<String, Value>> for Stream {
    type Error = YtScrapeError;

    fn try_from(video_renderer: &Map<String, Value>) -> Result<Self, Self::Error> {
        let id = video_renderer["videoId"].as_str().unwrap_or_default();
        let title = video_renderer["title"]["runs"][0]["text"].as_str().ok_or(
            YtScrapeError::ParseError("Failed to get video title via ['title']['runs'][0]['text']"),
        )?;
        let view_count = video_renderer["viewCountText"]["simpleText"]
            .as_str()
            .ok_or(YtScrapeError::ParseError(
                "Failed to get video view count via ['viewCountText']['simpleText']",
            ))?;
        let streamed_date = video_renderer["publishedTimeText"]["simpleText"]
            .as_str()
            .ok_or(YtScrapeError::ParseError(
                "Failed to get streamed_date via ['publishedTimeText']['simpleText']",
            ))?;
        let duration = video_renderer["lengthText"]["simpleText"].as_str().ok_or(
            YtScrapeError::ParseError("Failed to get duration via ['lengthText']['simpleText']"),
        )?;

        Ok(Stream {
            id: id.to_string(),
            title: title.to_string(),
            view_count: view_count.to_string(),
            streamed_date: streamed_date.to_string(),
            duration: duration.to_string(),
        })
    }
}

pub fn parse_streams(json: &Value) -> Result<Vec<Stream>, YtScrapeError> {
    let mut streams = Vec::new();

    if let Some(contents) = json["contents"]["twoColumnBrowseResultsRenderer"]["tabs"]
        .get(2)
        .and_then(|tab| tab["tabRenderer"]["content"]["richGridRenderer"]["contents"].as_array())
    {
        for item in contents {
            if let Some(video_renderer) =
                item["richItemRenderer"]["content"]["videoRenderer"].as_object()
            {
                let stream = Stream::try_from(video_renderer)?;
                streams.push(stream);
            }
        }
    } else {
        return Err(YtScrapeError::ParseError(
            "Failed to get script contents, structure might have changed",
        ));
    }

    Ok(streams)
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
/// * `Option<Value>`: Some(Value) if the JSON was successfully extracted and parsed,
///                    None if the JSON couldn't be found or parsed.
///
/// # Note
/// This method is somewhat fragile as it depends on the specific structure of YouTube's
/// HTML. If YouTube changes how they embed this data, this function may need to be updated.
pub fn extract_json_from_script(document: &str) -> Result<Value, YtScrapeError> {
    let re = regex::Regex::new(r"var ytInitialData = (.+?);</script>").unwrap();
    let result = re
        .captures(document)
        .and_then(|cap| cap.get(1))
        .and_then(|m| serde_json::from_str(m.as_str()).ok())
        .ok_or(YtScrapeError::ParseError(
            "Failed to extract ytInitialData from the page's script tag",
        ));

    result
}
