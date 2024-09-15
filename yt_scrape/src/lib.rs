mod datastore;
mod error;
mod parser;

pub use datastore::{DataStore, StreamClosedCaptions};
pub use parser::{extract_json_from_script, parse_streams, Stream};
