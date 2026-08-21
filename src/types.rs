use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Format {
    Markdown,
    Html,
    RawHtml,
    Links,
    Structured,
    Summary,
    Json,
    Brand,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScrapeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formats: Option<Vec<Format>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_in_cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_structure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_main_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_base64_images: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ads: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_links: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub credits_used: Option<i64>,
    pub credits_remaining: Option<i64>,
    pub freshness: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScrapeResult {
    pub url: String,
    pub final_url: Option<String>,
    pub redirected: Option<bool>,
    pub status: Option<i64>,
    pub cached: Option<bool>,
    pub warning: Option<String>,
    pub metadata: Option<Value>,
    pub usage: Option<Usage>,
    pub markdown: Option<String>,
    pub html: Option<String>,
    pub raw_html: Option<String>,
    pub links: Option<Vec<String>>,
    pub structured: Option<Value>,
    pub summary: Option<String>,
    pub json: Option<Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MapOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_links: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_query_parameters: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapResult {
    pub url: String,
    pub links: Vec<String>,
    pub count: Option<i64>,
    pub total: Option<i64>,
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrape_results: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrape_options: Option<ScrapeOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_domains: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchItem {
    pub title: Option<String>,
    pub url: String,
    pub snippet: Option<String>,
    pub rank: Option<i64>,
    pub data: Option<ScrapeResult>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResult {
    pub query: String,
    pub status: Option<String>,
    pub results: Vec<SearchItem>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_structure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_web_search: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_sources: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_entities: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtractItem {
    pub url: String,
    pub data: Option<Value>,
    pub fields: Option<Value>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtractResult {
    pub results: Vec<ExtractItem>,
    pub sources: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrawlOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_paths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_paths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_subdomains: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrape_options: Option<ScrapeOptions>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrape_options: Option<ScrapeOptions>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobPage {
    pub url: String,
    pub requested_url: Option<String>,
    pub status: String,
    pub depth: Option<i64>,
    pub error: Option<String>,
    pub error_code: Option<String>,
    pub retryable: Option<bool>,
    pub data: Option<ScrapeResult>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: Option<String>,
    pub kind: Option<String>,
    pub status: String,
    pub seed_url: Option<String>,
    pub total: Option<i64>,
    pub completed: Option<i64>,
    pub failed: Option<i64>,
    pub credits_used: Option<i64>,
    pub pages: Option<Vec<JobPage>>,
}

impl Job {
    pub fn is_done(&self) -> bool {
        matches!(self.status.as_str(), "completed" | "failed" | "cancelled")
    }
}

#[derive(Debug, Clone, Default)]
pub struct LogoOptions {
    pub theme: Option<String>,
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenshotOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_page: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenshotResult {
    pub url: String,
    pub final_url: Option<String>,
    pub status: Option<i64>,
    pub screenshot: String,
    pub screenshot_type: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub cached: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PageImage {
    pub src: String,
    pub alt: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagesResult {
    pub url: String,
    pub final_url: Option<String>,
    pub images: Vec<PageImage>,
}

#[derive(Debug, Clone)]
pub struct WaitOptions {
    pub poll_interval: std::time::Duration,
    pub timeout: std::time::Duration,
}

impl Default for WaitOptions {
    fn default() -> Self {
        Self {
            poll_interval: std::time::Duration::from_secs(2),
            timeout: std::time::Duration::from_secs(300),
        }
    }
}
