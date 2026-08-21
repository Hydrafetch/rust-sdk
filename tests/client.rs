use hydrafetch::{BatchOptions, Client, Format, ScrapeOptions, WaitOptions};
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
            json!({ "success": true, "data": { "url": "u", "markdown": "# hi" } }),
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
        .respond_with(ok(json!({ "success": true, "data": { "url": "u" } })))
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
async fn get_endpoints_use_query_parameters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/web/brand/logo"))
        .and(query_param("domain", "stripe.com"))
        .and(query_param("theme", "dark"))
        .respond_with(ok(
            json!({ "success": true, "data": { "url": "https://img" } }),
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
                .set_body_json(json!({ "batchId": "b_1", "status": "pending" })),
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

    let statuses = std::sync::Mutex::new(vec!["completed", "running", "pending"]);
    Mock::given(method("GET"))
        .and(path("/v1/web/batch/b_1"))
        .respond_with(move |_: &Request| {
            let next = statuses.lock().unwrap().pop().unwrap_or("completed");
            ok(json!({ "success": true, "data": { "id": "b_1", "status": next } }))
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

    assert_eq!(job.status, "completed");
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
            json!({ "success": true, "data": { "id": "b_1", "status": "running" } }),
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
