mod error;
mod parser;
mod process_stream;
pub mod summary;
pub mod tracing;
pub mod types;

use parser::{extract_json_from_script, parse_streams};
pub use process_stream::fetch_and_process_streams;
