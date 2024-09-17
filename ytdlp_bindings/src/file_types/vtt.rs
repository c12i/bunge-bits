use std::path::Path;

use crate::error::YtDlpError;

pub trait VttProcessor {
    fn read_vtt_file<P: AsRef<Path>>(&self, vtt_path: P) -> Result<String, YtDlpError>;

    fn stream_vtt_file<P: AsRef<Path>>(
        &self,
        vtt_path: P,
    ) -> Box<dyn Iterator<Item = Result<String, YtDlpError>>>;

    fn process_vtt_file<P: AsRef<Path>>(
        &self,
        vtt_path: P,
    ) -> Result<Vec<SubtitleEntry>, YtDlpError>;
}

#[derive(Debug, Clone)]
pub struct SubtitleEntry {
    pub start_time: String,
    pub end_time: String,
    pub text: String,
}

pub fn parse_vtt_content(content: &str) -> Vec<SubtitleEntry> {
    let mut entries = Vec::new();
    let mut lines = content.lines();

    // Skip the "WEBVTT" header
    lines.next();

    while let Some(line) = lines.next() {
        if line.contains("-->") {
            let times = line.split("-->").collect::<Vec<&str>>();
            if times.len() == 2 {
                let start_time = times[0].trim().to_string();
                let end_time = times[1].trim().to_string();
                let mut text = String::new();

                while let Some(text_line) = lines.next() {
                    if text_line.is_empty() {
                        break;
                    }
                    text.push_str(text_line);
                    text.push('\n');
                }

                entries.push(SubtitleEntry {
                    start_time,
                    end_time,
                    text: text.trim().to_string(),
                });
            }
        }
    }

    entries
}
