mod sliding_window;

use anyhow::Error;
use sliding_window::SlidingWindow;
use std::{future::Future, pin::Pin, sync::Arc};

/// Summarizes a VTT/ transcript (Video Text Tracks) string using a sliding window approach.
///
/// This function processes the input VTT/ transcript string by sliding a window over its content,
/// summarizing each chunk, and then combining these summaries into a final result.
///
/// # Arguments
///
/// * `vtt` - A `String` containing the VTT/ transcript content to be summarized.
/// * `summarize_chunk` - A function that summarizes a chunk of the VTT/ transcript content.
///   It takes two parameters:
///   - A `String` representing the current window of text to summarize.
///   - An `Arc<String>` containing the context from previous summaries.
///
/// It returns a `Future` that resolves to a `Result<String, Error>`.
/// * `combine_summaries` - A function that combines individual summaries into a final summary.
///   It takes a `Vec<String>` of individual summaries and returns a `Future` that resolves
///   to a `Result<String, Error>`.
///
/// # Type Parameters
///
/// * `FnSummary` - The type of the `summarize_chunk` function.
/// * `FnCombine` - The type of the `combine_summaries` function.
///
/// # Returns
///
/// Returns a `Result<String, Error>` which is the final combined summary if successful,
/// or an error if any part of the summarization process fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The `summarize_chunk` function returns an error for any chunk.
/// - The `combine_summaries` function returns an error.
///
/// # Note
///
/// This function uses a [`SlidingWindow`](crate::sliding_window::SlidingWindow) struct internally to manage the windowing process.
/// The window size, slide size, and context size are determined by constants defined in the
/// `SlidingWindow` implementation.
pub async fn summarize_with_sliding_window<FnSummary, FnCombine>(
    vtt: &str,
    summarize_chunk: FnSummary,
    combine_summaries: FnCombine,
) -> Result<String, Error>
where
    FnSummary: Fn(
        String,
        Option<Arc<String>>,
    ) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send>>,
    FnCombine: Fn(Vec<String>) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send>>,
{
    let mut window = SlidingWindow::new(vtt);
    let mut summaries = Vec::new();

    loop {
        let summary =
            summarize_chunk(window.current_window().to_owned(), window.get_context()).await?;

        window.update_context(&summary);
        summaries.push(summary);

        if !window.slide() {
            break;
        }
    }

    combine_summaries(summaries).await
}

/// Summarizes a VTT (Video Text Tracks)/ transcript string using a linear sequential approach.
///
/// This function processes the input VTT/ transcript string by splitting it into chunks based on a delimiter,
/// summarizing each chunk sequentially while maintaining context from previous summaries,
/// and then combining these summaries into a final result.
///
/// # Arguments
///
/// * `vtt` - A string slice containing the VTT/ transcript content to be summarized.
/// * `delimiter` - A string slice used to split the VTT/ transcript content into chunks.
/// * `summarize_chunk` - A function that summarizes a chunk of the VTT/ transcript content.
///   It takes two parameters:
///   - A `String` representing the current chunk of text to summarize.
///   - An `Option<Arc<String>>` containing the context from the previous summary (None for the first chunk).
///
/// It returns a `Future` that resolves to a `Result<String, Error>`.
/// * `combine_summaries` - A function that combines individual summaries into a final summary.
///   It takes a `Vec<String>` of individual summaries and returns a `Future` that resolves
///   to a `Result<String, Error>`.
///
/// # Type Parameters
///
/// * `FnSummary` - The type of the `summarize_chunk` function.
/// * `FnCombine` - The type of the `combine_summaries` function.
///
/// # Returns
///
/// Returns a `Result<String, Error>` which is the final combined summary if successful,
/// or an error if any part of the summarization process fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The `summarize_chunk` function returns an error for any chunk.
/// - The `combine_summaries` function returns an error.
///
/// # Differences from `summarize_with_sliding_window`
///
/// Unlike `summarize_with_sliding_window` which uses overlapping windows, this function:
/// - Processes chunks sequentially without overlap
/// - Uses a delimiter to split the input rather than fixed-size windows
/// - Passes the entire previous summary as context rather than a fixed context size
pub async fn summarize_linear<FnSummary, FnCombine>(
    vtt: &str,
    delimiter: &str,
    summarize_chunk: FnSummary,
    combine_summaries: FnCombine,
) -> Result<String, Error>
where
    FnSummary: Fn(
        String,
        Option<Arc<String>>,
    ) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send>>,
    FnCombine: Fn(Vec<String>) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send>>,
{
    let contents = vtt.split(delimiter);
    let mut summaries = Vec::new();
    let mut context = None;

    for chunk in contents {
        let summary = summarize_chunk(chunk.to_owned(), context.to_owned()).await?;

        context = Some(Arc::new(match context {
            Some(current_context) => format!("{}\n{}", *current_context, summary),
            None => summary.to_string(),
        }));

        summaries.push(summary);
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
            TEST_VTT,
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
            TEST_VTT,
            |chunk, context| {
                Box::pin(async move {
                    Ok(format!(
                        "Summary (prev: {}): {}",
                        context.unwrap_or_default().len(),
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
