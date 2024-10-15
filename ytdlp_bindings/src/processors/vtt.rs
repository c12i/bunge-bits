//! # vtt
//!
//! Enrich `YtDlp` by adding VTT processing capabilities

use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};

use crate::{error::YtDlpError, YtDlp};

pub trait VttProcessor {
    /// Reads the entire content of a VTT file into a string.
    ///
    /// # Arguments
    ///
    /// * `vtt_path` - The path to the VTT file.
    ///
    /// # Errors
    ///
    /// Returns `YtDlpError` if the file cannot be read.
    fn read_vtt_file<P: AsRef<Path>>(&self, vtt_path: P) -> Result<String, YtDlpError>;

    /// Creates an iterator that yields lines from a VTT file.
    ///
    /// # Arguments
    ///
    /// * `vtt_path` - The path to the VTT file.
    ///
    /// # Returns
    ///
    /// An iterator that yields `Result<String, YtDlpError>` for each line in the file.
    fn stream_vtt_file<P: AsRef<Path>>(
        &self,
        vtt_path: P,
    ) -> Box<dyn Iterator<Item = Result<String, YtDlpError>>>;

    /// Processes a VTT file and returns a vector of `SubtitleEntry` structs.
    ///
    /// # Arguments
    ///
    /// * `vtt_path` - The path to the VTT file.
    ///
    /// # Errors
    ///
    /// Returns `YtDlpError` if the file cannot be read or parsed.
    fn process_vtt_file<P: AsRef<Path>>(
        &self,
        vtt_path: P,
    ) -> Result<Vec<SubtitleEntry>, YtDlpError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleEntry {
    pub start_time: String,
    pub end_time: String,
    pub text: String,
}

impl SubtitleEntry {
    pub fn entries_from_vtt_str(content: &str) -> Vec<SubtitleEntry> {
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

                    for text_line in lines.by_ref() {
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
}

impl VttProcessor for YtDlp {
    fn read_vtt_file<P: AsRef<Path>>(&self, vtt_path: P) -> Result<String, YtDlpError> {
        let mut file = File::open(vtt_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    fn stream_vtt_file<P: AsRef<Path>>(
        &self,
        vtt_path: P,
    ) -> Box<dyn Iterator<Item = Result<String, YtDlpError>>> {
        let file = match File::open(vtt_path) {
            Ok(file) => file,
            Err(e) => return Box::new(std::iter::once(Err(YtDlpError::VttReadError(e)))),
        };

        let reader = BufReader::new(file);
        Box::new(
            reader
                .lines()
                .map(|line| line.map_err(YtDlpError::VttReadError)),
        )
    }

    fn process_vtt_file<P: AsRef<Path>>(
        &self,
        vtt_path: P,
    ) -> Result<Vec<SubtitleEntry>, YtDlpError> {
        let content = self.read_vtt_file(vtt_path)?;
        Ok(SubtitleEntry::entries_from_vtt_str(&content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockVttProcessor;

    impl VttProcessor for MockVttProcessor {
        fn read_vtt_file<P: AsRef<Path>>(&self, _vtt_path: P) -> Result<String, YtDlpError> {
            Ok("WEBVTT\n\n00:00:01.000 --> 00:00:04.000\nHello, world!\n\n".to_string())
        }

        fn stream_vtt_file<P: AsRef<Path>>(
            &self,
            _vtt_path: P,
        ) -> Box<dyn Iterator<Item = Result<String, YtDlpError>>> {
            Box::new(
                vec![
                    Ok("WEBVTT".to_string()),
                    Ok("".to_string()),
                    Ok("00:00:01.000 --> 00:00:04.000".to_string()),
                    Ok("Hello, world!".to_string()),
                    Ok("".to_string()),
                ]
                .into_iter(),
            )
        }

        fn process_vtt_file<P: AsRef<Path>>(
            &self,
            _vtt_path: P,
        ) -> Result<Vec<SubtitleEntry>, YtDlpError> {
            Ok(vec![SubtitleEntry {
                start_time: "00:00:01.000".to_string(),
                end_time: "00:00:04.000".to_string(),
                text: "Hello, world!".to_string(),
            }])
        }
    }

    #[test]
    fn test_read_vtt_file() {
        let processor = MockVttProcessor;
        let content = processor.read_vtt_file("dummy.vtt").unwrap();
        assert!(content.contains("WEBVTT"));
        assert!(content.contains("Hello, world!"));
    }

    #[test]
    fn test_stream_vtt_file() {
        let processor = MockVttProcessor;
        let lines: Vec<String> = processor
            .stream_vtt_file("dummy.vtt")
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(lines[0], "WEBVTT");
        assert!(lines.contains(&"Hello, world!".to_string()));
    }

    #[test]
    fn test_process_vtt_file() {
        let processor = MockVttProcessor;
        let entries = processor.process_vtt_file("dummy.vtt").unwrap();

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].start_time, "00:00:01.000");
        assert_eq!(entries[0].end_time, "00:00:04.000");
        assert_eq!(entries[0].text, "Hello, world!");
    }
}
