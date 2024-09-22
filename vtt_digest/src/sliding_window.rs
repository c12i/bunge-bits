const WINDOW_SIZE: usize = 2000;
const SLIDE_SIZE: usize = 1000;
const CONTEXT_SIZE: usize = 500;

#[derive(Debug)]
pub struct SlidingWindow {
    pub text: String,
    pub start: usize,
    pub end: usize,
    pub context: String,
}

impl SlidingWindow {
    pub fn new(text: &str) -> Self {
        SlidingWindow {
            text: text.to_string(),
            start: 0,
            end: WINDOW_SIZE.min(text.len()),
            context: String::new(),
        }
    }

    pub fn slide(&mut self) -> bool {
        if self.end >= self.text.len() {
            return false;
        }
        self.start += SLIDE_SIZE;
        self.end = (self.start + WINDOW_SIZE).min(self.text.len());
        true
    }

    pub fn current_window(&self) -> &str {
        &self.text[self.start..self.end]
    }

    pub fn update_context(&mut self, new_summary: &str) {
        self.context = format!("{}\n{}", self.context, new_summary);
        if self.context.len() > CONTEXT_SIZE {
            self.context = self.context[self.context.len() - CONTEXT_SIZE..].to_string();
        }
    }
}
