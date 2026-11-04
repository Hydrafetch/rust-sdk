mod error;
mod types;

pub use error::{Error, Result};
pub use types::*;

use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const DEFAULT_BASE_URL: &str = "https://api.hydrafetch.com";

#[derive(Deserialize)]
struct ApiError {
    code: Option<String>,
    message: Option<String>,
}

#[derive(Deserialize)]
struct Meta {
    #[serde(rename = "requestId")]
    request_id: Option<String>,
}

#[derive(Deserialize)]
struct Envelope {
    success: Option<bool>,
    data: Option<Value>,
    error: Option<ApiError>,
    meta: Option<Meta>,
}

pub struct ClientBuilder {
    api_key: Option<String>,
    base_url: String,
    timeout: Duration,
    max_retries: u32,
}

impl ClientBuilder {
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into().trim_end_matches('/').to_string();
        self
    }

    pub fn timeout(mut self, d: Duration) -> Self {
        self.timeout = d;
        self
    }

    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }

    pub fn build(self) -> Result<Client> {
        let api_key = self
            .api_key
            .or_else(|| std::env::var("HYDRAFETCH_API_KEY").ok())
            .filter(|k| !k.is_empty())
            .ok_or(Error::MissingApiKey)?;

        let http = reqwest::Client::builder()
            .timeout(self.timeout)
            .build()
            .map_err(Error::Transport)?;

        Ok(Client {
            api_key,
            base_url: self.base_url,
            http,
            max_retries: self.max_retries,
            last_request_id: Arc::new(Mutex::new(None)),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    api_key: String,
    base_url: String,
    http: reqwest::Client,
    max_retries: u32,
    last_request_id: Arc<Mutex<Option<String>>>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder {
            api_key: None,
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout: Duration::from_secs(120),
            max_retries: 2,
        }
    }

    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder().api_key(api_key).build()
    }

    pub fn from_env() -> Result<Self> {
        Self::builder().build()
    }

    async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<Value>,
        query: &[(&str, String)],
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let mut attempt = 0;

        loop {
            let mut req = self
                .http
                .request(method.clone(), &url)
                .header("X-API-Key", &self.api_key)
                .header("Accept", "application/json");

            if !query.is_empty() {
                req = req.query(query);
            }
            if let Some(ref payload) = body {
                req = req.json(payload);
            }

            let response = match req.send().await {
                Ok(r) => r,
                Err(e) if e.is_timeout() => {
                    return Err(Error::Timeout(format!(
                        "request to {path} timed out; raise the timeout, or use the crawl and batch job endpoints for long work"
                    )))
                }
                Err(e) => {
                    if attempt >= self.max_retries {
                        return Err(Error::Transport(e));
                    }
                    attempt += 1;
                    backoff(attempt).await;
                    continue;
                }
            };

            let status = response.status().as_u16();
            if let Some(id) = response
                .headers()
                .get("x-request-id")
                .and_then(|v| v.to_str().ok())
            {
                if let Ok(mut last) = self.last_request_id.lock() {
                    *last = Some(id.to_string());
                }
            }
            let text = response.text().await.map_err(Error::Transport)?;
            let envelope: Option<Envelope> = serde_json::from_str(&text).ok();

            let failed = status >= 400
                || envelope
                    .as_ref()
                    .and_then(|e| e.success)
                    .map(|ok| !ok)
                    .unwrap_or(false);

            if failed {
                let (code, message, request_id) = match envelope {
                    Some(env) => (
                        env.error.as_ref().and_then(|e| e.code.clone()),
                        env.error.as_ref().and_then(|e| e.message.clone()),
                        env.meta.and_then(|m| m.request_id),
                    ),
                    None => (None, None, None),
                };
                let err = Error::Api {
                    code: code.unwrap_or_else(|| format!("HTTP_{status}")),
                    message: message.unwrap_or_else(|| text.chars().take(300).collect()),
                    status,
                    request_id,
                };
                if !err.is_retryable() || attempt >= self.max_retries {
                    return Err(err);
                }
                attempt += 1;
                backoff(attempt).await;
                continue;
            }

            let payload = match envelope.and_then(|e| e.data) {
                Some(data) => data,
                None => serde_json::from_str(&text)?,
            };
            return Ok(serde_json::from_value(payload)?);
        }
    }

    fn merge(base: Value, extra: Option<impl serde::Serialize>) -> Value {
        let mut body = base;
        if let Some(options) = extra {
            if let Ok(Value::Object(fields)) = serde_json::to_value(options) {
                if let Value::Object(ref mut target) = body {
                    for (k, v) in fields {
                        target.insert(k, v);
                    }
                }
            }
        }
        body
    }

    pub fn last_request_id(&self) -> Option<String> {
        self.last_request_id.lock().ok().and_then(|id| id.clone())
    }

    pub async fn rate_result(&self, input: RateResultInput) -> Result<RateResultResponse> {
        if input.request_id.is_none() && input.url.is_none() {
            return Err(Error::Api {
                code: "VALIDATION_ERROR".to_string(),
                message: "pass request_id or url to say which result you are rating".to_string(),
                status: 400,
                request_id: None,
            });
        }
        let body = serde_json::to_value(input)?;
        self.request(reqwest::Method::POST, "/v1/web/feedback", Some(body), &[])
            .await
    }

    pub async fn scrape(&self, url: &str, options: Option<ScrapeOptions>) -> Result<ScrapeResult> {
        let body = Self::merge(json!({ "url": url }), options);
        self.request(reqwest::Method::POST, "/v1/web/scrape", Some(body), &[])
            .await
    }

    pub async fn markdown(&self, url: &str) -> Result<String> {
        let page = self
            .scrape(
                url,
                Some(ScrapeOptions {
                    formats: Some(vec![Format::Markdown]),
                    ..Default::default()
                }),
            )
            .await?;
        Ok(page.markdown.unwrap_or_default())
    }

    pub async fn map(&self, url: &str, options: Option<MapOptions>) -> Result<MapResult> {
        let body = Self::merge(json!({ "url": url }), options);
        self.request(reqwest::Method::POST, "/v1/web/map", Some(body), &[])
            .await
    }

    pub async fn search(
        &self,
        query: &str,
        options: Option<SearchOptions>,
    ) -> Result<SearchResult> {
        let body = Self::merge(json!({ "query": query }), options);
        self.request(reqwest::Method::POST, "/v1/web/search", Some(body), &[])
            .await
    }

    /// Finds open jobs by the words in the job title, newest first.
    pub async fn search_jobs(
        &self,
        query: &str,
        options: Option<SearchJobsOptions>,
    ) -> Result<Vec<OpenJob>> {
        let body = Self::merge(json!({ "query": query }), options);
        self.request(
            reqwest::Method::POST,
            "/v1/web/index/jobs/search",
            Some(body),
            &[],
        )
        .await
    }

    /// Reports what each company opened and closed since `since`, an ISO 8601 time.
    pub async fn hiring_changes(
        &self,
        domains: &[&str],
        since: &str,
        options: Option<HiringChangesOptions>,
    ) -> Result<Vec<CompanyHiringChanges>> {
        let body = Self::merge(json!({ "domains": domains, "since": since }), options);
        self.request(
            reqwest::Method::POST,
            "/v1/web/index/jobs/changes",
            Some(body),
            &[],
        )
        .await
    }

    pub async fn extract(
        &self,
        urls: &[&str],
        options: Option<ExtractOptions>,
    ) -> Result<ExtractResult> {
        let body = Self::merge(json!({ "urls": urls }), options);
        self.request(reqwest::Method::POST, "/v1/web/extract", Some(body), &[])
            .await
    }

    pub async fn brand(&self, domain: &str) -> Result<Value> {
        self.request(
            reqwest::Method::GET,
            "/v1/web/brand",
            None,
            &[("domain", domain.to_string())],
        )
        .await
    }

    pub async fn logo(&self, domain: &str, options: Option<LogoOptions>) -> Result<Value> {
        let mut query = vec![("domain", domain.to_string())];
        if let Some(opts) = options {
            if let Some(theme) = opts.theme {
                query.push(("theme", theme));
            }
            if let Some(kind) = opts.kind {
                query.push(("type", kind));
            }
        }
        self.request(reqwest::Method::GET, "/v1/web/brand/logo", None, &query)
            .await
    }

    pub async fn styleguide(&self, domain_or_url: &str) -> Result<Value> {
        self.styleguide_with(domain_or_url, StyleguideOptions::default())
            .await
    }

    pub async fn styleguide_with(
        &self,
        domain_or_url: &str,
        options: StyleguideOptions,
    ) -> Result<Value> {
        let lower = domain_or_url.to_ascii_lowercase();
        let key = if lower.starts_with("http://") || lower.starts_with("https://") {
            "url"
        } else {
            "domain"
        };
        let mut query = vec![(key, domain_or_url.to_string())];
        if let Some(max_age) = options.max_age {
            query.push(("maxAge", max_age.to_string()));
        }
        self.request(reqwest::Method::GET, "/v1/web/styleguide", None, &query)
            .await
    }

    pub async fn screenshot(
        &self,
        url: &str,
        options: Option<ScreenshotOptions>,
    ) -> Result<ScreenshotResult> {
        let body = Self::merge(json!({ "url": url }), options);
        self.request(reqwest::Method::POST, "/v1/web/screenshot", Some(body), &[])
            .await
    }

    pub async fn images(&self, url: &str) -> Result<ImagesResult> {
        self.request(
            reqwest::Method::POST,
            "/v1/web/images",
            Some(json!({ "url": url })),
            &[],
        )
        .await
    }

    pub async fn start_crawl(&self, url: &str, options: Option<CrawlOptions>) -> Result<String> {
        let body = Self::merge(json!({ "url": url }), options);
        let payload: Value = self
            .request(reqwest::Method::POST, "/v1/web/crawl", Some(body), &[])
            .await?;
        job_id(&payload, "crawlId")
    }

    pub async fn start_batch(
        &self,
        urls: &[&str],
        options: Option<BatchOptions>,
    ) -> Result<String> {
        let body = Self::merge(json!({ "urls": urls }), options);
        let payload: Value = self
            .request(reqwest::Method::POST, "/v1/web/batch", Some(body), &[])
            .await?;
        job_id(&payload, "batchId")
    }

    pub async fn crawl_status(&self, id: &str) -> Result<Job> {
        self.request(
            reqwest::Method::GET,
            &format!("/v1/web/crawl/{id}"),
            None,
            &[],
        )
        .await
    }

    pub async fn batch_status(&self, id: &str) -> Result<Job> {
        self.request(
            reqwest::Method::GET,
            &format!("/v1/web/batch/{id}"),
            None,
            &[],
        )
        .await
    }

    pub async fn crawl(
        &self,
        url: &str,
        options: Option<CrawlOptions>,
        wait: Option<WaitOptions>,
    ) -> Result<Job> {
        let id = self.start_crawl(url, options).await?;
        self.poll(JobKind::Crawl, &id, wait.unwrap_or_default())
            .await
    }

    pub async fn batch(
        &self,
        urls: &[&str],
        options: Option<BatchOptions>,
        wait: Option<WaitOptions>,
    ) -> Result<Job> {
        let id = self.start_batch(urls, options).await?;
        self.poll(JobKind::Batch, &id, wait.unwrap_or_default())
            .await
    }

    async fn poll(&self, kind: JobKind, id: &str, wait: WaitOptions) -> Result<Job> {
        let deadline = Instant::now() + wait.timeout;
        loop {
            let job = match kind {
                JobKind::Crawl => self.crawl_status(id).await?,
                JobKind::Batch => self.batch_status(id).await?,
            };
            if job.is_done() {
                return Ok(job);
            }
            if Instant::now() + wait.poll_interval > deadline {
                return Err(Error::Timeout(format!(
                    "job did not finish within {:?}; it is still running server-side, so poll its status or pass a webhook",
                    wait.timeout
                )));
            }
            tokio::time::sleep(wait.poll_interval).await;
        }
    }
}

#[derive(Clone, Copy)]
enum JobKind {
    Crawl,
    Batch,
}

fn job_id(payload: &Value, key: &str) -> Result<String> {
    for candidate in [key, "id"] {
        if let Some(value) = payload.get(candidate).and_then(|v| v.as_str()) {
            if !value.is_empty() {
                return Ok(value.to_string());
            }
        }
    }
    Err(Error::Api {
        code: "NO_JOB_ID".into(),
        message: "job accepted but returned no id".into(),
        status: 502,
        request_id: None,
    })
}

async fn backoff(attempt: u32) {
    tokio::time::sleep(Duration::from_millis(250 * 2u64.pow(attempt.min(5)))).await;
}
