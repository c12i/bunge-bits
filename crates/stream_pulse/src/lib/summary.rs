use std::{error::Error, future::Future, pin::Pin, sync::Arc};

/// Summarizes a transcript chunk string using a linear sequential approach.
///
/// This function processes the input transcript string by splitting it into chunks based on a delimiter,
/// summarizing each chunk sequentially while maintaining context from previous summaries,
/// and then combining these summaries into a final result.
///
/// See original documentation for usage details.
pub async fn summarize_linear<FnSummary, FnCombine>(
    chunk: &str,
    delimiter: &str,
    summarize_chunk: FnSummary,
    combine_summaries: FnCombine,
) -> anyhow::Result<String>
where
    FnSummary: Fn(
        String,
        Option<Arc<String>>,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<String>> + Send>>,
    FnCombine: Fn(Vec<String>) -> Pin<Box<dyn Future<Output = anyhow::Result<String>> + Send>>,
{
    let contents = chunk.split(delimiter);
    let mut summaries = Vec::new();
    let mut context = None;

    for chunk in contents {
        let chunk = chunk.trim();

        if chunk.is_empty() {
            continue;
        }

        let summary = summarize_chunk(chunk.to_owned(), context.clone()).await?;

        context = Some(Arc::new(match context {
            Some(current_context) => format!("{}\n{}", *current_context, summary),
            None => summary.to_string(),
        }));

        summaries.push(summary);
    }

    combine_summaries(summaries).await
}
