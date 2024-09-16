/// Represents the closed captions associated with a YouTube stream.
#[derive(Debug, sqlx::FromRow)]
pub struct StreamClosedCaptions {
    pub video_id: String,
    pub closed_caption_text: String,
    pub closed_caption_summary: Option<String>,
}
