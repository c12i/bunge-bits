use std::sync::Arc;

/// A struct representing a sliding window over a text, with context management.
///
/// `SlidingWindow` is used to process large texts in chunks, maintaining a context
/// of previous processing results. It's particularly useful for summarization tasks
/// where context from previous summaries may be relevant.
///
/// The behavior of `SlidingWindow` is governed by three constants:
/// * `WINDOW_SIZE`: The size of the sliding window.
/// * `SLIDE_SIZE`: The amount by which the window moves in each slide.
#[derive(Debug)]
pub struct SlidingWindow {
    pub text: String,
    pub start: usize,
    pub end: usize,
    pub context: Option<Arc<String>>,
}

impl SlidingWindow {
    const WINDOW_SIZE: usize = 2000;
    const SLIDE_SIZE: usize = 1000;

    pub fn new(text: &str) -> Self {
        SlidingWindow {
            text: text.to_string(),
            start: 0,
            end: Self::WINDOW_SIZE.min(text.len()),
            context: None,
        }
    }

    pub fn slide(&mut self) -> bool {
        if self.end >= self.text.len() {
            return false;
        }
        self.start += Self::SLIDE_SIZE;
        self.end = (self.start + Self::WINDOW_SIZE).min(self.text.len());
        true
    }

    pub fn current_window(&self) -> &str {
        &self.text[self.start..self.end]
    }

    pub fn update_context(&mut self, new_summary: &str) {
        let new_context = match &self.context {
            Some(current_context) => format!("{}\n{}", current_context, new_summary),
            None => new_summary.to_string(),
        };

        self.context = Some(Arc::new(new_context));
    }

    pub fn get_context(&self) -> Option<Arc<String>> {
        self.context.clone()
    }
}
