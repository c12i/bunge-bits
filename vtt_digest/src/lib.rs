mod sliding_window;

use anyhow::Error;
use sliding_window::SlidingWindow;
use std::{future::Future, pin::Pin};

pub async fn summarize_with_sliding_window<FnSummary, FnCombine>(
    vtt: String,
    summarize_chunk: FnSummary,
    combine_summaries: FnCombine,
) -> Result<String, Error>
where
    FnSummary: Fn(String, String) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send>>,
    FnCombine: Fn(Vec<String>) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send>>,
{
    let mut window = SlidingWindow::new(&vtt);
    let mut summaries = Vec::new();

    loop {
        let summary =
            summarize_chunk(window.current_window().to_owned(), window.context.clone()).await?;
        window.update_context(&summary);
        summaries.push(summary);

        if !window.slide() {
            break;
        }
    }

    combine_summaries(summaries).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    const TEST_VTT: &str = include_str!("../sample.vtt");

    #[tokio::test]
    async fn test_basic_summarization() -> Result<()> {
        let result = summarize_with_sliding_window(
            TEST_VTT.to_string(),
            |chunk, _context| {
                Box::pin(
                    async move { Ok(format!("Summary: {}", &chunk.lines().next().unwrap_or(""))) },
                )
            },
            |summaries| Box::pin(async move { Ok(summaries.join(" ")) }),
        )
        .await?;

        assert!(result.starts_with("Summary: WEBVTT"));
        assert!(result.len() < TEST_VTT.len());
        Ok(())
    }

    #[tokio::test]
    async fn test_context_awareness() -> Result<()> {
        let result = summarize_with_sliding_window(
            TEST_VTT.to_string(),
            |chunk, context| {
                Box::pin(async move {
                    Ok(format!(
                        "Summary (prev: {}): {}",
                        context.len(),
                        &chunk.lines().next().unwrap_or("")
                    ))
                })
            },
            |summaries| Box::pin(async move { Ok(summaries.join(" ")) }),
        )
        .await?;

        assert!(result.contains("prev: 0"));
        Ok(())
    }
}
