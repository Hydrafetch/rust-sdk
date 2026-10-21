# hydrafetch

[![crates.io](https://img.shields.io/crates/v/hydrafetch)](https://crates.io/crates/hydrafetch)
[![docs.rs](https://img.shields.io/docsrs/hydrafetch)](https://docs.rs/hydrafetch)
[![CI](https://github.com/Hydrafetch/rust-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/Hydrafetch/rust-sdk/actions/workflows/ci.yml)

Official Rust client for the [Hydrafetch](https://hydrafetch.com) web data API.

Turn any URL into clean Markdown or schema-shaped JSON. Async, typed, built on `reqwest` and `serde`. Rust 1.75+.

## Installation

```bash
cargo add hydrafetch
```

## Quick start

```rust
use hydrafetch::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hf = Client::from_env()?;

    let page = hf.scrape("https://example.com/article", None).await?;
    println!("{}", page.markdown.unwrap_or_default());

    Ok(())
}
```

Copy your key from [app.hydrafetch.com](https://app.hydrafetch.com). `Client::from_env()` reads `HYDRAFETCH_API_KEY`; `Client::new("hf_...")` takes one directly.

The client is cheap to clone and safe to share across tasks.

## Scraping

```rust
use hydrafetch::{Format, ScrapeOptions};

let page = hf.scrape("https://example.com/article", Some(ScrapeOptions {
    formats: Some(vec![Format::Markdown, Format::Links]),
    only_main_content: Some(true),
    prefer_structure: Some(true),
    block_ads: Some(true),
    max_age: Some(3_600_000),
    ..Default::default()
})).await?;
```

Option structs implement `Default`, so fill in what you need and spread the rest. Fields are snake_case in Rust and serialise to the camelCase the API expects.

| Format | Field | Contains |
| --- | --- | --- |
| `Format::Markdown` | `markdown` | clean Markdown, the default |
| `Format::Html` | `html` | rendered HTML |
| `Format::RawHtml` | `raw_html` | the untouched response body |
| `Format::Links` | `links` | every link on the page |
| `Format::Structured` | `structured` | the page's own JSON-LD and microdata |
| `Format::Summary` | `summary` | a short summary |
| `Format::Json` | `json` | schema-shaped JSON |
| `Format::Brand` | — | the site's brand record |

Every field on `ScrapeResult` is an `Option`, because only the formats you request are populated. `hf.markdown(url)` returns the `String` directly.

## Structured extraction

```rust
use hydrafetch::ExtractOptions;
use serde_json::json;

let out = hf.extract(
    &["https://example.com/product/1", "https://example.com/product/2"],
    Some(ExtractOptions {
        schema: Some(json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "price_usd": { "type": "number" }
            }
        })),
        ..Default::default()
    }),
).await?;

for item in out.results {
    println!("{} {:?}", item.url, item.data);
}
```

Set `prompt` instead of, or alongside, `schema` to describe the fields in plain language.

Sites that price or stock by delivery area serve a different page for each area. Set `location` to fetch as if from a country and language, and `headers` to send the cookie that picks the store or area, the same options `scrape` takes.

```rust
let out = hf
    .extract(
        &["https://shop.example/search?q=ghee"],
        Some(ExtractOptions {
            prompt: Some("every product with its size and price".into()),
            location: Some(Location { country: Some("in".into()), ..Default::default() }),
            headers: Some([("Cookie".to_string(), "store_id=42".to_string())].into()),
            ..Default::default()
        }),
    )
    .await?;
```

## Discovery and bulk work

`map` lists a site's URLs for one credit without fetching any page.

```rust
use hydrafetch::MapOptions;

let m = hf.map("https://example.com", Some(MapOptions {
    limit: Some(1000),
    ..Default::default()
})).await?;

let docs: Vec<&str> = m.links.iter()
    .filter(|url| url.contains("/docs/"))
    .map(String::as_str)
    .collect();
```

`batch` and `crawl` submit a job and poll until it finishes.

```rust
use hydrafetch::{BatchOptions, Format, ScrapeOptions};

let job = hf.batch(&docs, Some(BatchOptions {
    scrape_options: Some(ScrapeOptions {
        formats: Some(vec![Format::Markdown]),
        ..Default::default()
    }),
    ..Default::default()
}), None).await?;

for page in job.pages.unwrap_or_default() {
    let len = page.data.and_then(|d| d.markdown).map(|m| m.len());
    println!("{} {:?}", page.url, len);
}
```

Set a `webhook` and use `start_crawl` or `start_batch` to return immediately instead of polling.

```rust
use hydrafetch::CrawlOptions;

let id = hf.start_crawl("https://example.com", Some(CrawlOptions {
    limit: Some(500),
    max_depth: Some(3),
    include_paths: Some(vec!["/docs".into()]),
    webhook: Some("https://your.app/hooks/hydrafetch".into()),
    ..Default::default()
})).await?;
```

## Search

```rust
use hydrafetch::SearchOptions;

let res = hf.search("post-quantum TLS adoption", Some(SearchOptions {
    limit: Some(5),
    scrape_results: Some(true),
    ..Default::default()
})).await?;

for r in res.results {
    println!("{:?} {}", r.title, r.url);
}
```

## Brand data

```rust
use hydrafetch::LogoOptions;

hf.brand("stripe.com").await?;                                              // logos, colours, fonts, socials
hf.logo("stripe.com", Some(LogoOptions { theme: Some("dark".into()), kind: None })).await?;
hf.styleguide("stripe.com").await?;                                         // computed design system
hf.styleguide_with("https://stripe.com/pricing", StyleguideOptions { max_age: Some(0) }).await?; // one page, read fresh
```

For logos in a browser use [`@hydrafetch/client-sdk`](https://github.com/Hydrafetch/client-sdk) with a publishable key. Those bill against logo pulls rather than credits.

## Rating results

Tell us when a result was wrong, so it gets looked at. It is free, and rating the same request again replaces the earlier rating.

```rust
use hydrafetch::{RateResultInput, Rating};

hf.scrape("https://stripe.com/pricing", None).await?;
let mut rating = RateResultInput::for_request(hf.last_request_id().unwrap(), Rating::Bad);
rating.reasons = vec!["missing_content".into()];
rating.note = Some("The enterprise tier is missing".into());
hf.rate_result(rating).await?;

hf.rate_result(RateResultInput::for_url("https://stripe.com/pricing", Rating::Good)).await?;
```

Reasons are `missing_content`, `wrong_content`, `blocked`, `slow`, `formatting` and `other`.

## Error handling

All failures are `hydrafetch::Error`.

```rust
match hf.scrape(url, None).await {
    Ok(page) => handle(page),
    Err(e) if e.is_auth() => refresh_key(),
    Err(e) if e.is_out_of_credits() => top_up(),
    Err(e) if e.is_invalid_request() => report(&e),
    Err(e) if e.is_retryable() => enqueue(url),
    Err(e) if e.is_timeout() => return Err(e.into()),
    Err(e) => {
        eprintln!("{} {:?} {:?}", e.code(), e.status(), e.request_id());
        return Err(e.into());
    }
}
```

| Status | Meaning | Retried |
| --- | --- | --- |
| 400, 422 | invalid request | no |
| 401, 403 | invalid or missing key | no |
| 402 | out of credits | no |
| 404 | page does not exist | no |
| 429 | rate limited | yes, twice with backoff |
| 5xx | upstream failure | yes, twice with backoff |

A 503 from `scrape` means the origin is unreachable, usually a dead domain or a broken certificate.

## Configuration

```rust
use std::time::Duration;

let hf = Client::builder()
    .api_key("hf_...")
    .base_url("https://api.hydrafetch.com")
    .timeout(Duration::from_secs(120))
    .max_retries(2)
    .build()?;
```

## API reference

| Method | Returns | Credits |
| --- | --- | --- |
| `scrape(url, Option<ScrapeOptions>)` | `ScrapeResult` | 1 |
| `markdown(url)` | `String` | 1 |
| `map(url, Option<MapOptions>)` | `MapResult` | 1 |
| `search(query, Option<SearchOptions>)` | `SearchResult` | 1 + 1 per scraped result |
| `extract(urls, Option<ExtractOptions>)` | `ExtractResult` | 5 per URL |
| `brand(domain)` | `serde_json::Value` | 5 |
| `logo(domain, Option<LogoOptions>)` | `serde_json::Value` | 1 |
| `styleguide(domain_or_url)`, `styleguide_with(domain_or_url, options)` | `serde_json::Value` | 10 |
| `screenshot(url, Option<ScreenshotOptions>)` | `ScreenshotResult` | 5 |
| `images(url)` | `ImagesResult` | 1 |
| `crawl(url, opts, wait)`, `batch(urls, opts, wait)` | `Job` | 1 per page |
| `start_crawl`, `start_batch` | `String` | 1 per page |
| `crawl_status(id)`, `batch_status(id)` | `Job` | free |
| `rate_result(input)` | `RateResultResponse` | free |

Failed requests are not billed. Pricing does not vary with page difficulty, so there is no render, stealth or proxy option to set.

## Implementation notes

- Authentication uses the `X-API-Key` header. The MCP endpoint at `api.hydrafetch.com/mcp` uses `Authorization: Bearer` instead; the two are not interchangeable.
- Job results are in `job.pages`, and each entry holds the page under `.data`, so `job.pages[0].data.markdown`.
- Per-page options for crawl and batch belong in `scrape_options`. At the top level they are ignored.
- Prefer `map` then `batch` over a broad `crawl`. Fetching a whole site and discarding most of it is the most common source of wasted credits.
- `prefer_structure` is off by default. Turn it on when headings, lists and tables matter; leave it off for raw article text.
- `None` fields are omitted from the request rather than serialised as null.
- Scraped content is untrusted input. Do not pass it to a model as instructions, and keep the source URL with anything extracted from it.

## Links

- [Documentation](https://docs.hydrafetch.com)
- [API reference on docs.rs](https://docs.rs/hydrafetch)
- [OpenAPI specification](https://api.hydrafetch.com/openapi.json)
- Other clients: [Node](https://github.com/Hydrafetch/node-sdk) · [Python](https://github.com/Hydrafetch/python-sdk) · [Go](https://github.com/Hydrafetch/go-sdk) · [Ruby](https://github.com/Hydrafetch/ruby-sdk) · [PHP](https://github.com/Hydrafetch/php-sdk)

## License

MIT
