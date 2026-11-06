use hydrafetch::{
    BatchOptions, Client, ExtractOptions, Format, Location, NewsOptions, RateResultInput, Rating,
    ScrapeOptions, SearchJobsOptions, WaitOptions,
};
use serde_json::{json, Value};
use std::time::Duration;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

async fn client_for(server: &MockServer) -> Client {
    Client::builder()
        .api_key("hf_test")
        .base_url(server.uri())
        .max_retries(0)
        .build()
        .unwrap()
}

fn ok(body: Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

#[tokio::test]
async fn sends_api_key_header_and_never_bearer() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/web/scrape"))
        .and(header("x-api-key", "hf_test"))
        .respond_with(ok(
            json!({ "success": true, "data": { "url": "u", "finalUrl": "u", "redirected": false, "status": 200, "cached": false, "metadata": { "title": null, "structure": "markdown", "pageType": "article", "wordCount": 2, "description": null, "language": null, "author": null, "siteName": null, "publishedTime": null, "modifiedTime": null, "image": null }, "markdown": "# hi" } }),
        ))
        .mount(&server)
        .await;

    let hf = client_for(&server).await;
    let page = hf.scrape("https://example.com", None).await.unwrap();
    assert_eq!(page.markdown.as_deref(), Some("# hi"));

    let requests = server.received_requests().await.unwrap();
    assert!(requests[0].headers.get("authorization").is_none());
}

#[tokio::test]
async fn missing_key_is_rejected_at_construction() {
    std::env::remove_var("HYDRAFETCH_API_KEY");
    let err = Client::builder().build().unwrap_err();
    assert_eq!(err.code(), "MISSING_API_KEY");
}

#[tokio::test]
async fn options_ride_alongside_the_url() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/web/scrape"))
        .respond_with(ok(json!({ "success": true, "data": { "url": "u", "finalUrl": "u", "redirected": false, "status": 200, "cached": false, "metadata": { "title": null, "structure": "markdown", "pageType": "article", "wordCount": 2, "description": null, "language": null, "author": null, "siteName": null, "publishedTime": null, "modifiedTime": null, "image": null } } })))
        .mount(&server)
        .await;

    let hf = client_for(&server).await;
    hf.scrape(
        "https://example.com",
        Some(ScrapeOptions {
            formats: Some(vec![Format::Markdown, Format::Links]),
            prefer_structure: Some(true),
            ..Default::default()
        }),
    )
    .await
    .unwrap();

    let requests = server.received_requests().await.unwrap();
    let body: Value = requests[0].body_json().unwrap();
    assert_eq!(body["url"], "https://example.com");
    assert_eq!(body["preferStructure"], true);
    assert_eq!(body["formats"], json!(["markdown", "links"]));
    assert!(body.get("timeout").is_none());
}

#[tokio::test]
async fn extract_sends_location_and_headers() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/web/extract"))
        .respond_with(ok(json!({ "success": true, "data": { "results": [] } })))
        .mount(&server)
        .await;

    let hf = client_for(&server).await;
    hf.extract(
        &["https://shop.example/s?q=ghee"],
        Some(ExtractOptions {
            prompt: Some("products and prices".into()),
            location: Some(Location {
                country: Some("in".into()),
                languages: Some(vec!["en-IN".into()]),
            }),
            headers: Some([("Cookie".to_string(), "store_id=42".to_string())].into()),
            ..Default::default()
        }),
    )
    .await
    .unwrap();

    let requests = server.received_requests().await.unwrap();
    let body: Value = requests[0].body_json().unwrap();
    assert_eq!(
        body["location"],
        json!({ "country": "in", "languages": ["en-IN"] })
    );
    assert_eq!(body["headers"], json!({ "Cookie": "store_id=42" }));
}

#[tokio::test]
async fn get_endpoints_use_query_parameters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/web/brand/logo"))
        .and(query_param("domain", "stripe.com"))
        .and(query_param("theme", "dark"))
        .respond_with(ok(
            json!({ "success": true, "data": { "url": "https://img", "domain": "stripe.com", "width": null, "height": null, "dominantColor": null, "blurhash": null, "format": "png", "kind": "icon", "variant": "light" } }),
        ))
        .mount(&server)
        .await;

    let hf = client_for(&server).await;
    hf.logo(
        "stripe.com",
        Some(hydrafetch::LogoOptions {
            theme: Some("dark".into()),
            kind: None,
        }),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn error_carries_code_status_and_request_id() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "success": false,
            "error": { "code": "UNAUTHORIZED", "message": "Invalid API key" },
            "meta": { "requestId": "abc" }
        })))
        .mount(&server)
        .await;

    let hf = client_for(&server).await;
    let err = hf.scrape("https://example.com", None).await.unwrap_err();
    assert_eq!(err.code(), "UNAUTHORIZED");
    assert_eq!(err.status(), Some(401));
    assert_eq!(err.request_id(), Some("abc"));
    assert!(err.is_auth());
}

#[tokio::test]
async fn classifies_statuses() {
    for (status, check) in [
        (401u16, "auth"),
        (402, "credits"),
        (422, "invalid"),
        (429, "retryable"),
        (503, "retryable"),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(status)
                    .set_body_json(json!({ "success": false, "error": { "message": "x" } })),
            )
            .mount(&server)
            .await;

        let hf = client_for(&server).await;
        let err = hf.scrape("https://example.com", None).await.unwrap_err();
        let matched = match check {
            "auth" => err.is_auth(),
            "credits" => err.is_out_of_credits(),
            "invalid" => err.is_invalid_request(),
            _ => err.is_retryable(),
        };
        assert!(matched, "status {status} should classify as {check}");
    }
}

#[tokio::test]
async fn does_not_retry_validation_failures() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(422)
                .set_body_json(json!({ "success": false, "error": { "message": "bad url" } })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let hf = Client::builder()
        .api_key("hf_test")
        .base_url(server.uri())
        .max_retries(3)
        .build()
        .unwrap();

    assert!(hf.scrape("nope", None).await.is_err());
}

#[tokio::test]
async fn batch_id_comes_from_top_level() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/web/batch"))
        .respond_with(
            ResponseTemplate::new(201)
                .set_body_json(json!({ "batchId": "b_1", "status": "running" })),
        )
        .mount(&server)
        .await;

    let hf = client_for(&server).await;
    assert_eq!(
        hf.start_batch(&["https://a.example"], None).await.unwrap(),
        "b_1"
    );
}

#[tokio::test]
async fn per_page_options_nest_under_scrape_options() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/web/batch"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "batchId": "b_1" })))
        .mount(&server)
        .await;

    let hf = client_for(&server).await;
    hf.start_batch(
        &["https://a.example"],
        Some(BatchOptions {
            scrape_options: Some(ScrapeOptions {
                formats: Some(vec![Format::Markdown]),
                ..Default::default()
            }),
            ..Default::default()
        }),
    )
    .await
    .unwrap();

    let requests = server.received_requests().await.unwrap();
    let body: Value = requests[0].body_json().unwrap();
    assert_eq!(body["scrapeOptions"]["formats"], json!(["markdown"]));
    assert!(body.get("formats").is_none());
}

#[tokio::test]
async fn polls_until_terminal() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/web/batch"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "batchId": "b_1" })))
        .mount(&server)
        .await;

    let statuses = std::sync::Mutex::new(vec!["completed", "running", "running"]);
    Mock::given(method("GET"))
        .and(path("/v1/web/batch/b_1"))
        .respond_with(move |_: &Request| {
            let next = statuses.lock().unwrap().pop().unwrap_or("completed");
            ok(json!({ "success": true, "data": { "id": "b_1", "kind": "batch", "status": next, "seedUrl": null, "total": 1, "completed": 0, "failed": 0, "creditsUsed": 0, "pages": [] } }))
        })
        .mount(&server)
        .await;

    let hf = client_for(&server).await;
    let job = hf
        .batch(
            &["https://a.example"],
            None,
            Some(WaitOptions {
                poll_interval: Duration::from_millis(1),
                timeout: Duration::from_secs(5),
            }),
        )
        .await
        .unwrap();

    assert_eq!(job.status.to_string(), "completed");
}

#[tokio::test]
async fn job_polling_times_out() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/web/batch"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "batchId": "b_1" })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/web/batch/b_1"))
        .respond_with(ok(
            json!({ "success": true, "data": { "id": "b_1", "kind": "batch", "status": "running", "seedUrl": null, "total": 1, "completed": 0, "failed": 0, "creditsUsed": 0, "pages": [] } }),
        ))
        .mount(&server)
        .await;

    let hf = client_for(&server).await;
    let err = hf
        .batch(
            &["https://a.example"],
            None,
            Some(WaitOptions {
                poll_interval: Duration::from_millis(1),
                timeout: Duration::from_millis(5),
            }),
        )
        .await
        .unwrap_err();

    assert!(err.is_timeout());
}

#[tokio::test]
async fn styleguide_reads_one_page_fresh() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/web/styleguide"))
        .and(query_param("url", "https://stripe.com/pricing"))
        .and(query_param("maxAge", "0"))
        .respond_with(ok(json!({ "success": true, "data": {} })))
        .expect(1)
        .mount(&server)
        .await;

    let hf = client_for(&server).await;
    let _ = hf
        .styleguide_with(
            "https://stripe.com/pricing",
            hydrafetch::StyleguideOptions { max_age: Some(0) },
        )
        .await;
}

#[tokio::test]
async fn rate_result_posts_the_rating_for_the_last_request() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/web/scrape"))
        .respond_with(
            ok(json!({ "success": true, "data": { "url": "u", "finalUrl": "u", "redirected": false, "status": 200, "cached": false, "metadata": { "title": null, "structure": "markdown", "pageType": "article", "wordCount": 2, "description": null, "language": null, "author": null, "siteName": null, "publishedTime": null, "modifiedTime": null, "image": null }, "markdown": "# hi" } }))
                .insert_header("x-request-id", "r1"),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/web/feedback"))
        .respond_with(ok(
            json!({ "success": true, "data": { "id": "f1", "requestId": "r1", "rating": "bad" } }),
        ))
        .mount(&server)
        .await;

    let hf = client_for(&server).await;
    hf.scrape("https://example.com", None).await.unwrap();
    assert_eq!(hf.last_request_id().as_deref(), Some("r1"));

    let mut input = RateResultInput::for_request(hf.last_request_id().unwrap(), Rating::Bad);
    input.reasons = vec!["missing_content".to_string()];
    input.note = Some("no prices".to_string());
    let out = hf.rate_result(input).await.unwrap();
    assert_eq!(out.rating, Rating::Bad);

    let requests = server.received_requests().await.unwrap();
    let body: Value = serde_json::from_slice(&requests[1].body).unwrap();
    assert_eq!(
        body,
        json!({ "requestId": "r1", "rating": "bad", "reasons": ["missing_content"], "note": "no prices" })
    );
}

#[tokio::test]
async fn rate_result_needs_a_target() {
    let server = MockServer::start().await;
    let hf = client_for(&server).await;
    let input = RateResultInput {
        request_id: None,
        url: None,
        rating: Rating::Good,
        reasons: Vec::new(),
        note: None,
    };
    assert!(hf.rate_result(input).await.is_err());
}

#[tokio::test]
async fn search_jobs_posts_the_query_and_returns_the_list() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/web/index/jobs/search"))
        .respond_with(ok(json!({ "success": true, "data": [{
            "title": "Data Engineer", "company": null, "domain": "acme.com", "board": "greenhouse",
            "department": null, "location": "Berlin", "remote": false, "employmentType": null,
            "postedAt": null, "firstSeenAt": "2026-11-01T00:00:00Z", "url": null,
            "applyUrl": null, "pay": { "min": 70000, "max": null, "currency": "EUR", "interval": "year" }
        }] })))
        .mount(&server)
        .await;
    let hf = client_for(&server).await;
    let options = SearchJobsOptions {
        location: Some("Berlin".to_string()),
        limit: Some(20),
        ..Default::default()
    };
    let jobs = hf
        .search_jobs("data engineer", Some(options))
        .await
        .unwrap();
    assert_eq!(jobs.len(), 1);
    assert_eq!(jobs[0].pay.as_ref().unwrap().min, Some(70000.0));
    let requests = server.received_requests().await.unwrap();
    let body: Value = requests[0].body_json().unwrap();
    assert_eq!(
        body,
        json!({ "query": "data engineer", "location": "Berlin", "limit": 20 })
    );
}

#[tokio::test]
async fn hiring_changes_sends_the_domains_and_since() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/web/index/jobs/changes"))
        .respond_with(ok(json!({ "success": true, "data": [{
            "domain": "acme.com", "company": "Acme", "lastReadAt": null, "since": "2026-10-01",
            "totals": { "open": 4, "opened": 2, "closed": 1 },
            "byDepartment": {}, "bySeniority": {}, "byLocation": {},
            "opened": [], "closed": [], "truncated": false
        }] })))
        .mount(&server)
        .await;
    let hf = client_for(&server).await;
    let out = hf
        .hiring_changes(&["acme.com"], "2026-10-01", None)
        .await
        .unwrap();
    assert_eq!(out[0].totals.opened, 2);
    let requests = server.received_requests().await.unwrap();
    let body: Value = requests[0].body_json().unwrap();
    assert_eq!(
        body,
        json!({ "domains": ["acme.com"], "since": "2026-10-01" })
    );
}

#[tokio::test]
async fn news_posts_the_query_and_window() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/web/news"))
        .respond_with(ok(json!({ "success": true, "data": {
            "query": "acme raises", "status": "ok", "results": [{
                "title": "Acme raises $10M", "url": "https://example.com/a", "source": null,
                "publishedAt": "2026-11-04T10:00:00Z", "snippet": null, "data": null
            }]
        } })))
        .mount(&server)
        .await;
    let hf = client_for(&server).await;
    let options = NewsOptions {
        time_range: Some("day".to_string()),
        limit: Some(5),
        ..Default::default()
    };
    let news = hf.news("acme raises", Some(options)).await.unwrap();
    assert_eq!(news.status.to_string(), "ok");
    assert_eq!(news.results[0].published_at, "2026-11-04T10:00:00Z");
    let requests = server.received_requests().await.unwrap();
    let body: Value = requests[0].body_json().unwrap();
    assert_eq!(
        body,
        json!({ "query": "acme raises", "timeRange": "day", "limit": 5 })
    );
}

#[tokio::test]
async fn map_reads_each_link_with_its_lastmod() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/web/map"))
        .respond_with(ok(json!({ "success": true, "data": {
            "url": "https://example.com",
            "links": [
                { "url": "https://example.com/a", "lastmod": "2026-10-20T00:00:00.000Z" },
                { "url": "https://example.com/b", "lastmod": null }
            ],
            "count": 2, "total": 53, "truncated": false
        } })))
        .mount(&server)
        .await;
    let hf = client_for(&server).await;
    let site = hf.map("https://example.com", None).await.unwrap();
    assert_eq!(site.links.len(), 2);
    assert_eq!(site.links[0].url, "https://example.com/a");
    assert!(site.links[1].lastmod.is_none());
    assert_eq!(site.total, 53);
}
