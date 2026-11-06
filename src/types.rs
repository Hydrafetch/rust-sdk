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
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
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
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_options: Option<Value>,
}

pub type Usage = crate::generated::WebUsageDto;

pub type ScrapeResult = crate::generated::WebScrapeDataDto;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sitemap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sitemap_include: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sitemap_exclude: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

pub type MapResult = crate::generated::MapResultDto;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<String>,
}

pub type SearchItem = crate::generated::SearchResultItemDto;

pub type SearchResult = crate::generated::SearchResultDto;

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
    pub show_confidence: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_entities: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
}

pub type ExtractItem = crate::generated::ExtractItemResultDto;

pub type ExtractResult = crate::generated::ExtractResultDto;

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
    pub webhook: Option<Webhook>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrape_options: Option<ScrapeOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_links: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_query_parameters: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sitemap: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrape_options: Option<ScrapeOptions>,
}

pub type JobPage = crate::generated::CrawlPageResultDto;

pub type Job = crate::generated::CrawlStatusResponseDto;

impl crate::generated::CrawlStatusResponseDto {
    pub fn is_done(&self) -> bool {
        use crate::generated::CrawlStatusResponseDtoStatus as S;
        matches!(self.status, S::Completed | S::Failed | S::Cancelled)
    }
}

#[derive(Debug, Clone, Default)]
pub struct LogoOptions {
    pub theme: Option<String>,
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct StyleguideOptions {
    pub max_age: Option<u64>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport_height: Option<u32>,
}

pub type ScreenshotResult = crate::generated::WebScreenshotDataDto;

pub type PageImage = crate::generated::WebImageDto;

pub type ImagesResult = crate::generated::WebImagesDataDto;

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

pub type Rating = crate::generated::ResultFeedbackDataDtoRating;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateResultInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub rating: Rating,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reasons: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl RateResultInput {
    pub fn for_request(request_id: impl Into<String>, rating: Rating) -> Self {
        Self {
            request_id: Some(request_id.into()),
            url: None,
            rating,
            reasons: Vec::new(),
            note: None,
        }
    }

    pub fn for_url(url: impl Into<String>, rating: Rating) -> Self {
        Self {
            request_id: None,
            url: Some(url.into()),
            rating,
            reasons: Vec::new(),
            note: None,
        }
    }
}

pub type RateResultResponse = crate::generated::ResultFeedbackDataDto;

/// Narrows a jobs search. Each 10 jobs returned cost 1 credit.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchJobsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_within_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_descriptions: Option<bool>,
}

pub type JobPay = crate::generated::JobsIndexPayDto;

pub type OpenJob = crate::generated::JobsIndexJobDto;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HiringChangesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_descriptions: Option<bool>,
}

pub type HiringCounts = crate::generated::JobsIndexCountsDto;

pub type HiringChangePosting = crate::generated::JobsIndexChangePostingDto;

pub type CompanyHiringChanges = crate::generated::JobsIndexCompanyChangesDto;

/// Narrows a news search. A search costs 1 credit, and one that finds nothing is free.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// One of "hour", "day", "week" or "month" (default "week").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrape_results: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrape_options: Option<ScrapeOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u64>,
}

pub type NewsArticle = crate::generated::NewsResultItemDto;

pub type NewsResult = crate::generated::NewsResultDto;

pub type MapLink = crate::generated::MapUrlEntryDto;

pub type Webhook = crate::generated::WebhookDto;

pub type BrandResult = crate::generated::WebBrandDataDto;

pub type LogoResult = crate::generated::LogoDescriptionDto;

pub type StyleguideResult = crate::generated::WebStyleguideDataDto;
