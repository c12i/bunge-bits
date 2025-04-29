//! # vtt
//!
//! Enrich `YtDlp` by adding web-VTT (web Video Text Tracks) processing capabilities

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};
use webvtt_parser::OwnedVtt;

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

    /// Read a VTT file as while and processes a VTT file and returns an `OwnedVtt` struct.
    ///
    /// # Arguments
    ///
    /// * `vtt_path` - The path to the VTT file.
    ///
    /// # Errors
    ///
    /// Returns `YtDlpError` if the file cannot be read or parsed.
    fn process_vtt_file<P: AsRef<Path>>(&self, vtt_path: P) -> Result<OwnedVtt, YtDlpError>;
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
            Err(e) => {
                return Box::new(std::iter::once(Err(YtDlpError::VttReadError(
                    e.to_string(),
                ))))
            }
        };

        let reader = BufReader::new(file);
        Box::new(
            reader
                .lines()
                .map(|line| line.map_err(|e| YtDlpError::VttReadError(e.to_string()))),
        )
    }

    fn process_vtt_file<P: AsRef<Path>>(&self, vtt_path: P) -> Result<OwnedVtt, YtDlpError> {
        let content = self.read_vtt_file(vtt_path)?;
        OwnedVtt::parse(&content).map_err(|e| YtDlpError::VttReadError(e.to_string()))
    }
}
