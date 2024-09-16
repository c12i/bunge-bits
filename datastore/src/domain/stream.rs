#[derive(Debug, sqlx::FromRow)]
pub struct Stream {
    pub video_id: String,
    pub title: String,
    pub view_count: String,
    pub streamed_date: String,
    pub duration: String,
}

impl Stream {
    pub fn url(&self) -> String {
        format!("https://www.youtube.com/watch?v={}", self.video_id)
    }
}
