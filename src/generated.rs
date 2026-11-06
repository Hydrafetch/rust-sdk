#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = "`BatchAcceptedDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BatchAcceptedDto {
    #[doc = "Poll this batch id for progress and results."]
    #[serde(rename = "batchId")]
    pub batch_id: ::std::string::String,
    #[doc = "The job has been accepted."]
    pub status: BatchAcceptedDtoStatus,
}
#[doc = "The job has been accepted."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum BatchAcceptedDtoStatus {
    #[serde(rename = "queued")]
    Queued,
}
impl ::std::fmt::Display for BatchAcceptedDtoStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Queued => f.write_str("queued"),
        }
    }
}
impl ::std::str::FromStr for BatchAcceptedDtoStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "queued" => Ok(Self::Queued),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BatchAcceptedDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BatchAcceptedDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`BatchRequestDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BatchRequestDto {
    #[doc = "How to scrape each URL. Same options as a single scrape."]
    #[serde(
        rename = "scrapeOptions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrape_options: ::std::option::Option<ScrapeOptionsDto>,
    #[doc = "The explicit list of URLs to scrape. Each must be http(s)."]
    pub urls: ::std::vec::Vec<::std::string::String>,
    #[doc = "Register a callback to be notified as the batch progresses instead of polling."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub webhook: ::std::option::Option<WebhookDto>,
}
#[doc = "`BrandAssetDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BrandAssetDto {
    #[doc = "Tiny placeholder hash, so a client can render something before the image lands."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub blurhash: ::std::option::Option<::std::string::String>,
    #[doc = "The colours in the asset itself. A logo's own palette is the brand's, by definition."]
    pub colors: ::std::vec::Vec<BrandColorDto>,
    pub format: ::std::string::String,
    #[doc = "What kind of mark this is."]
    pub kind: BrandAssetDtoKind,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub resolution: ::std::option::Option<BrandResolutionDto>,
    #[doc = "Public and immutable, so it can be embedded anywhere."]
    pub url: ::std::string::String,
    #[doc = "Which background the asset is legible on. `mono` is a single-colour mark that works on both."]
    pub variant: BrandAssetDtoVariant,
}
#[doc = "What kind of mark this is."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum BrandAssetDtoKind {
    #[serde(rename = "wordmark")]
    Wordmark,
    #[serde(rename = "symbol")]
    Symbol,
    #[serde(rename = "favicon")]
    Favicon,
    #[serde(rename = "backdrop")]
    Backdrop,
}
impl ::std::fmt::Display for BrandAssetDtoKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Wordmark => f.write_str("wordmark"),
            Self::Symbol => f.write_str("symbol"),
            Self::Favicon => f.write_str("favicon"),
            Self::Backdrop => f.write_str("backdrop"),
        }
    }
}
impl ::std::str::FromStr for BrandAssetDtoKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "wordmark" => Ok(Self::Wordmark),
            "symbol" => Ok(Self::Symbol),
            "favicon" => Ok(Self::Favicon),
            "backdrop" => Ok(Self::Backdrop),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BrandAssetDtoKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BrandAssetDtoKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Which background the asset is legible on. `mono` is a single-colour mark that works on both."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum BrandAssetDtoVariant {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "mono")]
    Mono,
}
impl ::std::fmt::Display for BrandAssetDtoVariant {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Light => f.write_str("light"),
            Self::Dark => f.write_str("dark"),
            Self::Mono => f.write_str("mono"),
        }
    }
}
impl ::std::str::FromStr for BrandAssetDtoVariant {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            "mono" => Ok(Self::Mono),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BrandAssetDtoVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BrandAssetDtoVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`BrandBatchDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct BrandBatchDto {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub domains: ::std::vec::Vec<::std::string::String>,
}
#[doc = "`BrandBatchResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BrandBatchResponseDto {
    #[doc = "One entry per requested domain, in the order you sent them. Null where unresolved."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub data: ::std::option::Option<::std::vec::Vec<WebBrandDataDto>>,
}
#[doc = "`BrandColorDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BrandColorDto {
    pub hex: ::std::string::String,
    #[doc = "Nearest CSS colour name, so a swatch reads as something and not only as six digits."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<BrandColorDtoRole>,
    #[doc = "How much the site itself uses it — the ranking that makes this the brand's colour."]
    pub weight: f64,
}
#[doc = "`BrandColorDtoRole`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum BrandColorDtoRole {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "accent")]
    Accent,
    #[serde(rename = "background")]
    Background,
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for BrandColorDtoRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Primary => f.write_str("primary"),
            Self::Accent => f.write_str("accent"),
            Self::Background => f.write_str("background"),
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for BrandColorDtoRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "primary" => Ok(Self::Primary),
            "accent" => Ok(Self::Accent),
            "background" => Ok(Self::Background),
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BrandColorDtoRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BrandColorDtoRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`BrandFieldProvenanceDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BrandFieldProvenanceDto {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub confidence: ::std::option::Option<f64>,
    #[doc = "`declared` the site said it in its own markup. `harvested` we processed an asset or read its CSS. `inferred` a model produced it from the page. `external` an enrichment source rather than the site."]
    pub source: BrandFieldProvenanceDtoSource,
}
#[doc = "`declared` the site said it in its own markup. `harvested` we processed an asset or read its CSS. `inferred` a model produced it from the page. `external` an enrichment source rather than the site."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum BrandFieldProvenanceDtoSource {
    #[serde(rename = "declared")]
    Declared,
    #[serde(rename = "harvested")]
    Harvested,
    #[serde(rename = "inferred")]
    Inferred,
    #[serde(rename = "external")]
    External,
}
impl ::std::fmt::Display for BrandFieldProvenanceDtoSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Declared => f.write_str("declared"),
            Self::Harvested => f.write_str("harvested"),
            Self::Inferred => f.write_str("inferred"),
            Self::External => f.write_str("external"),
        }
    }
}
impl ::std::str::FromStr for BrandFieldProvenanceDtoSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "declared" => Ok(Self::Declared),
            "harvested" => Ok(Self::Harvested),
            "inferred" => Ok(Self::Inferred),
            "external" => Ok(Self::External),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BrandFieldProvenanceDtoSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BrandFieldProvenanceDtoSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`BrandFontDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BrandFontDto {
    pub family: ::std::string::String,
    #[doc = "Absent when the page does not say which the face is."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<BrandFontDtoRole>,
    #[doc = "How much of the site's type this face sets, 1 for the most-used one."]
    pub weight: f64,
}
#[doc = "Absent when the page does not say which the face is."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum BrandFontDtoRole {
    #[serde(rename = "heading")]
    Heading,
    #[serde(rename = "body")]
    Body,
}
impl ::std::fmt::Display for BrandFontDtoRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Heading => f.write_str("heading"),
            Self::Body => f.write_str("body"),
        }
    }
}
impl ::std::str::FromStr for BrandFontDtoRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "heading" => Ok(Self::Heading),
            "body" => Ok(Self::Body),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BrandFontDtoRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BrandFontDtoRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`BrandLinksDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct BrandLinksDto {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub blog: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub careers: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub contact: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub docs: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pricing: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub privacy: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub terms: ::std::option::Option<::std::string::String>,
}
#[doc = "`BrandLogoResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BrandLogoResponseDto {
    #[doc = "Null when we hold no mark for the domain. A domain with no logo is free."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub data: ::std::option::Option<LogoDescriptionDto>,
}
#[doc = "`BrandNaicsDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BrandNaicsDto {
    pub code: ::std::string::String,
    #[doc = "How much evidence the classification found, not a model's self-report."]
    pub confidence: f64,
    #[doc = "How specific the answer is. `industry_group` is the 4-digit level, `national_industry` the 6-digit one."]
    pub level: BrandNaicsDtoLevel,
    pub sector: BrandNaicsSectorDto,
    pub title: ::std::string::String,
}
#[doc = "How specific the answer is. `industry_group` is the 4-digit level, `national_industry` the 6-digit one."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum BrandNaicsDtoLevel {
    #[serde(rename = "sector")]
    Sector,
    #[serde(rename = "industry_group")]
    IndustryGroup,
    #[serde(rename = "national_industry")]
    NationalIndustry,
}
impl ::std::fmt::Display for BrandNaicsDtoLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Sector => f.write_str("sector"),
            Self::IndustryGroup => f.write_str("industry_group"),
            Self::NationalIndustry => f.write_str("national_industry"),
        }
    }
}
impl ::std::str::FromStr for BrandNaicsDtoLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "sector" => Ok(Self::Sector),
            "industry_group" => Ok(Self::IndustryGroup),
            "national_industry" => Ok(Self::NationalIndustry),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BrandNaicsDtoLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BrandNaicsDtoLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`BrandNaicsSectorDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BrandNaicsSectorDto {
    pub code: ::std::string::String,
    pub title: ::std::string::String,
}
#[doc = "`BrandResolutionDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BrandResolutionDto {
    #[doc = "Width over height — what a caller reserves layout space with."]
    #[serde(rename = "aspectRatio")]
    pub aspect_ratio: f64,
    pub height: i64,
    pub width: i64,
}
#[doc = "`BrandResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BrandResponseDto {
    #[doc = "Null when the domain could not be resolved at all. Unresolved domains are free."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub data: ::std::option::Option<WebBrandDataDto>,
}
#[doc = "`BrandSearchResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BrandSearchResponseDto {
    pub data: WebBrandSearchDataDto,
}
#[doc = "`BrandSearchResultDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BrandSearchResultDto {
    #[doc = "The brand's primary colour."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<::std::string::String>,
    pub domain: ::std::string::String,
    #[doc = "The primary logo, so a picker can render a recognisable row."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logo: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub name: ::std::option::Option<::std::string::String>,
}
#[doc = "`BrandSocialDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BrandSocialDto {
    #[doc = "The account name, without the `@`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub handle: ::std::option::Option<::std::string::String>,
    pub platform: ::std::string::String,
    pub url: ::std::string::String,
}
#[doc = "`CrawlAcceptedDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CrawlAcceptedDto {
    #[doc = "Poll this crawl id for progress and results."]
    #[serde(rename = "crawlId")]
    pub crawl_id: ::std::string::String,
    #[doc = "The job has been accepted."]
    pub status: CrawlAcceptedDtoStatus,
}
#[doc = "The job has been accepted."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CrawlAcceptedDtoStatus {
    #[serde(rename = "queued")]
    Queued,
}
impl ::std::fmt::Display for CrawlAcceptedDtoStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Queued => f.write_str("queued"),
        }
    }
}
impl ::std::str::FromStr for CrawlAcceptedDtoStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "queued" => Ok(Self::Queued),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CrawlAcceptedDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrawlAcceptedDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CrawlPageResultDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CrawlPageResultDto {
    #[doc = "The scraped page, once completed."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub data: ::std::option::Option<WebScrapeDataDto>,
    #[doc = "How many links deep this page was from the start."]
    pub depth: i64,
    #[doc = "How this URL entered the job. `requested` is a URL you gave us, meaning the crawl seed or a batch entry. `sitemap` came from the site's sitemap, `link` from following a link on a page we had already fetched."]
    #[serde(rename = "discoveredVia")]
    pub discovered_via: CrawlPageResultDtoDiscoveredVia,
    #[doc = "Human-readable failure message. Set when this page failed."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub error: ::std::option::Option<::std::string::String>,
    #[doc = "Stable machine-readable failure code. Set when this page failed."]
    #[serde(
        rename = "errorCode",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub error_code: ::std::option::Option<::std::string::String>,
    #[doc = "For a batch, the URL exactly as you sent it, present only when it differed from `url` after normalisation (a trailing slash or #fragment). A batch always returns one entry per URL you submitted, in order, including duplicates — so you can match results back to your input list positionally."]
    #[serde(
        rename = "requestedUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub requested_url: ::std::option::Option<::std::string::String>,
    #[doc = "When failed: true if it gave up after real retries (transient — safe to re-submit), false if a terminal failure."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub retryable: ::std::option::Option<bool>,
    #[doc = "State of this page."]
    pub status: CrawlPageResultDtoStatus,
    #[doc = "The page URL."]
    pub url: ::std::string::String,
}
#[doc = "How this URL entered the job. `requested` is a URL you gave us, meaning the crawl seed or a batch entry. `sitemap` came from the site's sitemap, `link` from following a link on a page we had already fetched."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CrawlPageResultDtoDiscoveredVia {
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "sitemap")]
    Sitemap,
    #[serde(rename = "link")]
    Link,
}
impl ::std::fmt::Display for CrawlPageResultDtoDiscoveredVia {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Requested => f.write_str("requested"),
            Self::Sitemap => f.write_str("sitemap"),
            Self::Link => f.write_str("link"),
        }
    }
}
impl ::std::str::FromStr for CrawlPageResultDtoDiscoveredVia {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "requested" => Ok(Self::Requested),
            "sitemap" => Ok(Self::Sitemap),
            "link" => Ok(Self::Link),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CrawlPageResultDtoDiscoveredVia {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrawlPageResultDtoDiscoveredVia {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "State of this page."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CrawlPageResultDtoStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for CrawlPageResultDtoStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Queued => f.write_str("queued"),
            Self::Running => f.write_str("running"),
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for CrawlPageResultDtoStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "queued" => Ok(Self::Queued),
            "running" => Ok(Self::Running),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CrawlPageResultDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrawlPageResultDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CrawlRequestDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CrawlRequestDto {
    #[doc = "Also follow links that lead off the starting site. Default off."]
    #[serde(
        rename = "allowExternalLinks",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allow_external_links: ::std::option::Option<bool>,
    #[doc = "Also follow links into subdomains of the starting site. Default off."]
    #[serde(
        rename = "allowSubdomains",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allow_subdomains: ::std::option::Option<bool>,
    #[doc = "Skip URLs whose path matches any of these patterns."]
    #[serde(
        rename = "excludePaths",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exclude_paths: ::std::vec::Vec<::std::string::String>,
    #[doc = "Treat URLs that differ only by query string as the same page. Default off."]
    #[serde(
        rename = "ignoreQueryParameters",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ignore_query_parameters: ::std::option::Option<bool>,
    #[doc = "Only follow URLs whose path matches every one of these patterns."]
    #[serde(
        rename = "includePaths",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub include_paths: ::std::vec::Vec<::std::string::String>,
    #[doc = "Maximum number of pages to scrape."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "How many links deep from the starting page to follow."]
    #[serde(
        rename = "maxDepth",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_depth: ::std::option::Option<i64>,
    #[doc = "How to scrape each page. Same options as a single scrape."]
    #[serde(
        rename = "scrapeOptions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrape_options: ::std::option::Option<ScrapeOptionsDto>,
    #[doc = "Whether to seed discovery from the site's published page list. Default includes it."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sitemap: ::std::option::Option<CrawlRequestDtoSitemap>,
    #[doc = "The site to start from. Must be http(s)."]
    pub url: ::std::string::String,
    #[doc = "Register a callback to be notified as the crawl progresses instead of polling."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub webhook: ::std::option::Option<WebhookDto>,
}
#[doc = "Whether to seed discovery from the site's published page list. Default includes it."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CrawlRequestDtoSitemap {
    #[serde(rename = "skip")]
    Skip,
    #[serde(rename = "include")]
    Include,
}
impl ::std::fmt::Display for CrawlRequestDtoSitemap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Skip => f.write_str("skip"),
            Self::Include => f.write_str("include"),
        }
    }
}
impl ::std::str::FromStr for CrawlRequestDtoSitemap {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "skip" => Ok(Self::Skip),
            "include" => Ok(Self::Include),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CrawlRequestDtoSitemap {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrawlRequestDtoSitemap {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CrawlStatusEnvelopeDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CrawlStatusEnvelopeDto {
    pub data: CrawlStatusResponseDto,
}
#[doc = "`CrawlStatusResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CrawlStatusResponseDto {
    #[doc = "Pages scraped so far."]
    pub completed: i64,
    #[doc = "Credits consumed so far. One per scraped page."]
    #[serde(rename = "creditsUsed")]
    pub credits_used: i64,
    #[doc = "Pages that failed."]
    pub failed: i64,
    #[doc = "The crawl or batch id."]
    pub id: ::std::string::String,
    #[doc = "Which kind of job this is."]
    pub kind: CrawlStatusResponseDtoKind,
    #[doc = "Per-page status and results."]
    pub pages: ::std::vec::Vec<CrawlPageResultDto>,
    #[doc = "The starting URL, for a crawl."]
    #[serde(
        rename = "seedUrl",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub seed_url: ::std::option::Option<::std::string::String>,
    #[doc = "Overall job state."]
    pub status: CrawlStatusResponseDtoStatus,
    #[doc = "Total pages in this job."]
    pub total: i64,
}
#[doc = "Which kind of job this is."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CrawlStatusResponseDtoKind {
    #[serde(rename = "crawl")]
    Crawl,
    #[serde(rename = "batch")]
    Batch,
}
impl ::std::fmt::Display for CrawlStatusResponseDtoKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Crawl => f.write_str("crawl"),
            Self::Batch => f.write_str("batch"),
        }
    }
}
impl ::std::str::FromStr for CrawlStatusResponseDtoKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "crawl" => Ok(Self::Crawl),
            "batch" => Ok(Self::Batch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CrawlStatusResponseDtoKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrawlStatusResponseDtoKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Overall job state."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CrawlStatusResponseDtoStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}
impl ::std::fmt::Display for CrawlStatusResponseDtoStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Running => f.write_str("running"),
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
            Self::Cancelled => f.write_str("cancelled"),
        }
    }
}
impl ::std::str::FromStr for CrawlStatusResponseDtoStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "running" => Ok(Self::Running),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CrawlStatusResponseDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrawlStatusResponseDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ExtractItemResultDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ExtractItemResultDto {
    #[doc = "The extracted data, shaped by your schema and/or prompt. Null when nothing matched."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub data: ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "Set when this page could not be extracted; null on success."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub error: ::std::option::Option<::std::string::String>,
    #[doc = "Per-field confidence and source passage, keyed by field name. Present only when `showConfidence` is set."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub fields: ::std::collections::HashMap<::std::string::String, FieldProvenanceDto>,
    #[doc = "The page this result came from."]
    pub url: ::std::string::String,
}
#[doc = "`ExtractRequestDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ExtractRequestDto {
    #[doc = "Pull in extra source pages by web-searching your prompt, to fill fields your URLs do not cover. Requires a `prompt`."]
    #[serde(
        rename = "enableWebSearch",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub enable_web_search: ::std::option::Option<bool>,
    #[doc = "Extra request headers to send when fetching each page, such as a cookie that picks the store or delivery area a site prices by."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub headers: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "Fetch each page as if from this country and with these preferred languages, for sites that change prices or content by region."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationDto>,
    #[doc = "Reuse a recent capture of each page if it is younger than this many milliseconds. Omit or 0 to always fetch fresh. Capped at 7 days."]
    #[serde(
        rename = "maxAge",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_age: ::std::option::Option<i64>,
    #[doc = "Merge the per-page results into one deduplicated collection — one row per entity, with its contributing source URLs — instead of a separate result per page. Default off."]
    #[serde(
        rename = "mergeEntities",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub merge_entities: ::std::option::Option<bool>,
    #[doc = "Preserve document structure (headings, lists, tables) over prose density when reading the page — good for listing and catalog pages. Default off."]
    #[serde(
        rename = "preferStructure",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub prefer_structure: ::std::option::Option<bool>,
    #[doc = "Natural-language instruction for what to extract. Use with or instead of a schema."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt: ::std::option::Option<ExtractRequestDtoPrompt>,
    #[doc = "JSON Schema describing the shape you want back. Optional if `prompt` is given; when both are present the schema fixes the field names and types while the prompt guides what to pull."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub schema: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "For each field, return a confidence score and the exact source passage the value was drawn from. Default off."]
    #[serde(
        rename = "showConfidence",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub show_confidence: ::std::option::Option<bool>,
    #[doc = "Return the concrete list of URLs that were actually extracted, after any wildcard and web-search expansion. Default off."]
    #[serde(
        rename = "showSources",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub show_sources: ::std::option::Option<bool>,
    #[doc = "The pages to extract from. Each must be an http(s) URL. A trailing `/*` marks a crawl scope: every page discovered under that path is extracted and merged into the result."]
    pub urls: ::std::vec::Vec<::std::string::String>,
}
#[doc = "Natural-language instruction for what to extract. Use with or instead of a schema."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExtractRequestDtoPrompt(::std::string::String);
impl ::std::ops::Deref for ExtractRequestDtoPrompt {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExtractRequestDtoPrompt> for ::std::string::String {
    fn from(value: ExtractRequestDtoPrompt) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExtractRequestDtoPrompt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 2000usize {
            return Err("longer than 2000 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExtractRequestDtoPrompt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExtractRequestDtoPrompt {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExtractRequestDtoPrompt {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`ExtractResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ExtractResponseDto {
    pub data: ExtractResultDto,
}
#[doc = "`ExtractResultDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ExtractResultDto {
    #[doc = "The deduplicated collection, one row per entity. Present only when `mergeEntities` is set."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub collection: ::std::vec::Vec<MergedEntityDto>,
    #[doc = "One result per extracted page."]
    pub results: ::std::vec::Vec<ExtractItemResultDto>,
    #[doc = "The concrete URLs actually extracted, after wildcard and web-search expansion. Present only when `showSources` is set."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub sources: ::std::vec::Vec<::std::string::String>,
}
#[doc = "`FieldProvenanceDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FieldProvenanceDto {
    #[doc = "How certain the value is correct given the page, from 0 to 1."]
    pub confidence: f64,
    #[doc = "The exact short passage the value was drawn from, or an empty string if none."]
    pub evidence: ::std::string::String,
}
#[doc = "`ImagesRequestDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ImagesRequestDto {
    #[doc = "Serve from cache if a capture of this URL is younger than this many milliseconds. Omit for the default window; 0 always fetches fresh. Capped at 7 days."]
    #[serde(
        rename = "maxAge",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_age: ::std::option::Option<i64>,
    #[doc = "The URL to harvest images from. Must be http(s)."]
    pub url: ::std::string::String,
}
#[doc = "`ImagesResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ImagesResponseDto {
    pub data: WebImagesDataDto,
}
#[doc = "`JobsIndexChangePostingDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JobsIndexChangePostingDto {
    pub board: ::std::string::String,
    #[serde(
        rename = "closedAt",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub closed_at: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub department: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "firstSeenAt")]
    pub first_seen_at: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "postedAt",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub posted_at: ::std::option::Option<::std::string::String>,
    #[doc = "intern, junior, mid, senior, lead, manager, director, vp or c-level, from the title."]
    pub seniority: ::std::string::String,
    pub title: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub url: ::std::option::Option<::std::string::String>,
}
#[doc = "`JobsIndexChangesRequestDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JobsIndexChangesRequestDto {
    #[doc = "Company websites to report on."]
    pub domains: ::std::vec::Vec<::std::string::String>,
    #[doc = "Include the description of each newly opened job when we have it."]
    #[serde(
        rename = "includeDescriptions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_descriptions: ::std::option::Option<bool>,
    #[doc = "Report changes since this time, at most about 400 days back."]
    pub since: ::std::string::String,
}
#[doc = "`JobsIndexChangesResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JobsIndexChangesResponseDto {
    pub data: ::std::vec::Vec<JobsIndexCompanyChangesDto>,
}
#[doc = "`JobsIndexCompanyChangesDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JobsIndexCompanyChangesDto {
    #[serde(rename = "byDepartment")]
    pub by_department: ::std::collections::HashMap<::std::string::String, JobsIndexCountsDto>,
    #[doc = "The top locations by open jobs."]
    #[serde(rename = "byLocation")]
    pub by_location: ::std::collections::HashMap<::std::string::String, JobsIndexCountsDto>,
    #[serde(rename = "bySeniority")]
    pub by_seniority: ::std::collections::HashMap<::std::string::String, JobsIndexCountsDto>,
    #[doc = "Newest first, up to 200."]
    pub closed: ::std::vec::Vec<JobsIndexChangePostingDto>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub company: ::std::option::Option<::std::string::String>,
    pub domain: ::std::string::String,
    #[doc = "When we last read the company."]
    #[serde(
        rename = "lastReadAt",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_read_at: ::std::option::Option<::std::string::String>,
    #[doc = "Newest first, up to 200."]
    pub opened: ::std::vec::Vec<JobsIndexChangePostingDto>,
    pub since: ::std::string::String,
    pub totals: JobsIndexCountsDto,
    #[doc = "True when the company had more postings than one call reports."]
    pub truncated: bool,
}
#[doc = "`JobsIndexCountsDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JobsIndexCountsDto {
    #[doc = "Closed since the date."]
    pub closed: i64,
    #[doc = "Open now."]
    pub open: i64,
    #[doc = "Opened since the date."]
    pub opened: i64,
}
#[doc = "`JobsIndexJobDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JobsIndexJobDto {
    #[doc = "Direct apply link, when different."]
    #[serde(
        rename = "applyUrl",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub apply_url: ::std::option::Option<::std::string::String>,
    #[doc = "The hiring platform the job is posted on."]
    pub board: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub company: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub department: ::std::option::Option<::std::string::String>,
    #[doc = "Present when includeDescriptions is true."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The company's website."]
    pub domain: ::std::string::String,
    #[serde(
        rename = "employmentType",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub employment_type: ::std::option::Option<::std::string::String>,
    #[doc = "When we first saw it open."]
    #[serde(rename = "firstSeenAt")]
    pub first_seen_at: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pay: ::std::option::Option<JobsIndexPayDto>,
    #[serde(
        rename = "postedAt",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub posted_at: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub remote: ::std::option::Option<bool>,
    pub title: ::std::string::String,
    #[doc = "Link to the employer's posting."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub url: ::std::option::Option<::std::string::String>,
}
#[doc = "`JobsIndexPayDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JobsIndexPayDto {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub currency: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub interval: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub max: ::std::option::Option<f64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub min: ::std::option::Option<f64>,
}
#[doc = "`JobsIndexSearchRequestDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JobsIndexSearchRequestDto {
    #[doc = "Include each job description when we have it."]
    #[serde(
        rename = "includeDescriptions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_descriptions: ::std::option::Option<bool>,
    #[doc = "How many jobs to return, newest first. Defaults to 10."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "A city, region or country the job location must mention."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<JobsIndexSearchRequestDtoLocation>,
    #[doc = "Only jobs posted in the last N days."]
    #[serde(
        rename = "postedWithinDays",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub posted_within_days: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Words to find in the job title. Every word must appear."]
    pub query: JobsIndexSearchRequestDtoQuery,
    #[doc = "Only jobs marked remote."]
    #[serde(
        rename = "remoteOnly",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub remote_only: ::std::option::Option<bool>,
}
#[doc = "A city, region or country the job location must mention."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct JobsIndexSearchRequestDtoLocation(::std::string::String);
impl ::std::ops::Deref for JobsIndexSearchRequestDtoLocation {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<JobsIndexSearchRequestDtoLocation> for ::std::string::String {
    fn from(value: JobsIndexSearchRequestDtoLocation) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for JobsIndexSearchRequestDtoLocation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 200usize {
            return Err("longer than 200 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for JobsIndexSearchRequestDtoLocation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JobsIndexSearchRequestDtoLocation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for JobsIndexSearchRequestDtoLocation {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Words to find in the job title. Every word must appear."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct JobsIndexSearchRequestDtoQuery(::std::string::String);
impl ::std::ops::Deref for JobsIndexSearchRequestDtoQuery {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<JobsIndexSearchRequestDtoQuery> for ::std::string::String {
    fn from(value: JobsIndexSearchRequestDtoQuery) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for JobsIndexSearchRequestDtoQuery {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 300usize {
            return Err("longer than 300 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for JobsIndexSearchRequestDtoQuery {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JobsIndexSearchRequestDtoQuery {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for JobsIndexSearchRequestDtoQuery {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`JobsIndexSearchResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JobsIndexSearchResponseDto {
    pub data: ::std::vec::Vec<JobsIndexJobDto>,
}
#[doc = "`JsonOptionsDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct JsonOptionsDto {
    #[doc = "Natural-language instruction for the `json` format. Use with or instead of a schema."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt: ::std::option::Option<JsonOptionsDtoPrompt>,
    #[doc = "JSON Schema describing the shape you want the `json` format to return."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub schema: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
#[doc = "Natural-language instruction for the `json` format. Use with or instead of a schema."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct JsonOptionsDtoPrompt(::std::string::String);
impl ::std::ops::Deref for JsonOptionsDtoPrompt {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<JsonOptionsDtoPrompt> for ::std::string::String {
    fn from(value: JsonOptionsDtoPrompt) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for JsonOptionsDtoPrompt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 2000usize {
            return Err("longer than 2000 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for JsonOptionsDtoPrompt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonOptionsDtoPrompt {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for JsonOptionsDtoPrompt {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`LocationDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct LocationDto {
    #[doc = "ISO 3166 alpha-2 country to fetch the page as if from."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub country: ::std::option::Option<::std::string::String>,
    #[doc = "Preferred content languages, most-preferred first."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub languages: ::std::vec::Vec<::std::string::String>,
}
#[doc = "`LogoDescriptionDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LogoDescriptionDto {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub blurhash: ::std::option::Option<::std::string::String>,
    pub domain: ::std::string::String,
    #[serde(
        rename = "dominantColor",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub dominant_color: ::std::option::Option<::std::string::String>,
    pub format: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub height: ::std::option::Option<i64>,
    pub kind: ::std::string::String,
    #[doc = "Public and immutable, so it can be embedded anywhere."]
    pub url: ::std::string::String,
    pub variant: LogoDescriptionDtoVariant,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub width: ::std::option::Option<i64>,
}
#[doc = "`LogoDescriptionDtoVariant`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LogoDescriptionDtoVariant {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "mono")]
    Mono,
}
impl ::std::fmt::Display for LogoDescriptionDtoVariant {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Light => f.write_str("light"),
            Self::Dark => f.write_str("dark"),
            Self::Mono => f.write_str("mono"),
        }
    }
}
impl ::std::str::FromStr for LogoDescriptionDtoVariant {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            "mono" => Ok(Self::Mono),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LogoDescriptionDtoVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LogoDescriptionDtoVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`MapRequestDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MapRequestDto {
    #[doc = "Cursor for paging a large archive newest→oldest: return only URLs with lastmod at or before this ISO timestamp. Page by passing the oldest lastmod from the previous page; dedupe by url across pages."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub before: ::std::option::Option<::std::string::String>,
    #[doc = "Treat URLs that differ only by query string as one. Default off."]
    #[serde(
        rename = "ignoreQueryParameters",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ignore_query_parameters: ::std::option::Option<bool>,
    #[doc = "Also include same-site links found on the starting page, not just the site's page list."]
    #[serde(
        rename = "includeLinks",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_links: ::std::option::Option<bool>,
    #[doc = "Also include URLs on subdomains of the site. Default off."]
    #[serde(
        rename = "includeSubdomains",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_subdomains: ::std::option::Option<bool>,
    #[doc = "Maximum number of URLs to return."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Skip the first N results (in the sorted order) then return the next `limit`. Pagination for sources with no lastmod (WordPress-core), where the date cursor can't apply. Page sequentially (0, limit, 2·limit, …) so the sub-sitemap cache keeps deep pages fast."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub offset: ::std::option::Option<i64>,
    #[doc = "Sort order by sitemap lastmod. \"newest\" (default) returns the most recent URLs first — the wanted end of a large archive under the limit. URLs without a lastmod sort last."]
    #[serde(default = "defaults::map_request_dto_order")]
    pub order: MapRequestDtoOrder,
    #[doc = "Keep only discovered URLs containing this term."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub search: ::std::option::Option<MapRequestDtoSearch>,
    #[doc = "Whether to use the site's published page list: skip it, include it alongside discovered links, or use it only."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sitemap: ::std::option::Option<MapRequestDtoSitemap>,
    #[doc = "Skip sub-sitemaps whose URL contains any of these comma-separated terms (case-insensitive). Compose with sitemapInclude — e.g. include \"post\" + exclude \"tag,category,author\" avoids WordPress taxonomy sitemaps (post_tag-sitemap, …-taxonomies-post_tag-*) that also contain \"post\"."]
    #[serde(
        rename = "sitemapExclude",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sitemap_exclude: ::std::option::Option<MapRequestDtoSitemapExclude>,
    #[doc = "On sitemap-index sites, only descend into sub-sitemaps whose URL contains one of these comma-separated terms (case-insensitive). E.g. \"post\" to target article sitemaps."]
    #[serde(
        rename = "sitemapInclude",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sitemap_include: ::std::option::Option<MapRequestDtoSitemapInclude>,
    #[doc = "The site whose URLs you want to enumerate. Must be http(s)."]
    pub url: ::std::string::String,
}
#[doc = "Sort order by sitemap lastmod. \"newest\" (default) returns the most recent URLs first — the wanted end of a large archive under the limit. URLs without a lastmod sort last."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MapRequestDtoOrder {
    #[serde(rename = "newest")]
    Newest,
    #[serde(rename = "oldest")]
    Oldest,
    #[serde(rename = "sitemap")]
    Sitemap,
}
impl ::std::fmt::Display for MapRequestDtoOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Newest => f.write_str("newest"),
            Self::Oldest => f.write_str("oldest"),
            Self::Sitemap => f.write_str("sitemap"),
        }
    }
}
impl ::std::str::FromStr for MapRequestDtoOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "newest" => Ok(Self::Newest),
            "oldest" => Ok(Self::Oldest),
            "sitemap" => Ok(Self::Sitemap),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MapRequestDtoOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MapRequestDtoOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for MapRequestDtoOrder {
    fn default() -> Self {
        MapRequestDtoOrder::Newest
    }
}
#[doc = "Keep only discovered URLs containing this term."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct MapRequestDtoSearch(::std::string::String);
impl ::std::ops::Deref for MapRequestDtoSearch {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MapRequestDtoSearch> for ::std::string::String {
    fn from(value: MapRequestDtoSearch) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for MapRequestDtoSearch {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 200usize {
            return Err("longer than 200 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MapRequestDtoSearch {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MapRequestDtoSearch {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MapRequestDtoSearch {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Whether to use the site's published page list: skip it, include it alongside discovered links, or use it only."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MapRequestDtoSitemap {
    #[serde(rename = "skip")]
    Skip,
    #[serde(rename = "include")]
    Include,
    #[serde(rename = "only")]
    Only,
}
impl ::std::fmt::Display for MapRequestDtoSitemap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Skip => f.write_str("skip"),
            Self::Include => f.write_str("include"),
            Self::Only => f.write_str("only"),
        }
    }
}
impl ::std::str::FromStr for MapRequestDtoSitemap {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "skip" => Ok(Self::Skip),
            "include" => Ok(Self::Include),
            "only" => Ok(Self::Only),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MapRequestDtoSitemap {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MapRequestDtoSitemap {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Skip sub-sitemaps whose URL contains any of these comma-separated terms (case-insensitive). Compose with sitemapInclude — e.g. include \"post\" + exclude \"tag,category,author\" avoids WordPress taxonomy sitemaps (post_tag-sitemap, …-taxonomies-post_tag-*) that also contain \"post\"."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct MapRequestDtoSitemapExclude(::std::string::String);
impl ::std::ops::Deref for MapRequestDtoSitemapExclude {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MapRequestDtoSitemapExclude> for ::std::string::String {
    fn from(value: MapRequestDtoSitemapExclude) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for MapRequestDtoSitemapExclude {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 200usize {
            return Err("longer than 200 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MapRequestDtoSitemapExclude {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MapRequestDtoSitemapExclude {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MapRequestDtoSitemapExclude {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "On sitemap-index sites, only descend into sub-sitemaps whose URL contains one of these comma-separated terms (case-insensitive). E.g. \"post\" to target article sitemaps."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct MapRequestDtoSitemapInclude(::std::string::String);
impl ::std::ops::Deref for MapRequestDtoSitemapInclude {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MapRequestDtoSitemapInclude> for ::std::string::String {
    fn from(value: MapRequestDtoSitemapInclude) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for MapRequestDtoSitemapInclude {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 200usize {
            return Err("longer than 200 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MapRequestDtoSitemapInclude {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MapRequestDtoSitemapInclude {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MapRequestDtoSitemapInclude {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`MapResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MapResponseDto {
    pub data: MapResultDto,
}
#[doc = "`MapResultDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MapResultDto {
    #[doc = "How many URLs were returned."]
    pub count: i64,
    #[doc = "The site's discovered URLs, each with its sitemap lastmod (null when unknown)."]
    pub links: ::std::vec::Vec<MapUrlEntryDto>,
    #[doc = "Total URLs in the enumeration this page is a slice of. Page with `offset` until offset+count reaches this — do not infer the end from a short page."]
    pub total: i64,
    #[doc = "Whether enumeration hit its ceiling or time budget. When true, `total` is a floor rather than the real size of the site."]
    pub truncated: bool,
    #[doc = "The site you requested."]
    pub url: ::std::string::String,
}
#[doc = "`MapUrlEntryDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MapUrlEntryDto {
    #[doc = "The <lastmod> from the sitemap, if present. null for links found on the page."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub lastmod: ::std::option::Option<::std::string::String>,
    pub url: ::std::string::String,
}
#[doc = "`MergedEntityDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MergedEntityDto {
    #[doc = "The unioned data for one entity, filled across every contributing page."]
    pub data: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Every source URL that contributed to this entity."]
    pub sources: ::std::vec::Vec<::std::string::String>,
}
#[doc = "`NewsRequestDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NewsRequestDto {
    #[doc = "ISO 3166 alpha-2 country code to lean results toward that edition."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub country: ::std::option::Option<::std::string::String>,
    #[doc = "Drop articles from these sites."]
    #[serde(
        rename = "excludeDomains",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exclude_domains: ::std::vec::Vec<::std::string::String>,
    #[doc = "Only return articles from these sites."]
    #[serde(
        rename = "includeDomains",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub include_domains: ::std::vec::Vec<::std::string::String>,
    #[doc = "How many articles to return, newest first."]
    #[serde(default = "defaults::default_nzu64::<::std::num::NonZeroU64, 10>")]
    pub limit: ::std::num::NonZeroU64,
    #[doc = "Accept a recent answer to the same search no older than this many milliseconds. Leave it out to accept answers up to 5 minutes old for `hour`, 15 for `day` and 30 for `week` or `month`. `0` always searches fresh."]
    #[serde(
        rename = "maxAge",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_age: ::std::option::Option<u64>,
    #[doc = "What to find news about."]
    pub query: NewsRequestDtoQuery,
    #[serde(
        rename = "scrapeOptions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrape_options: ::std::option::Option<ScrapeOptionsDto>,
    #[doc = "When true, each article also comes back with its page read and returned as clean data, at 1 additional credit per article read."]
    #[serde(
        rename = "scrapeResults",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrape_results: ::std::option::Option<bool>,
    #[doc = "Only articles published within this window before now."]
    #[serde(
        rename = "timeRange",
        default = "defaults::news_request_dto_time_range"
    )]
    pub time_range: NewsRequestDtoTimeRange,
}
#[doc = "What to find news about."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NewsRequestDtoQuery(::std::string::String);
impl ::std::ops::Deref for NewsRequestDtoQuery {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<NewsRequestDtoQuery> for ::std::string::String {
    fn from(value: NewsRequestDtoQuery) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for NewsRequestDtoQuery {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 500usize {
            return Err("longer than 500 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for NewsRequestDtoQuery {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NewsRequestDtoQuery {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for NewsRequestDtoQuery {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Only articles published within this window before now."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NewsRequestDtoTimeRange {
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
}
impl ::std::fmt::Display for NewsRequestDtoTimeRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Hour => f.write_str("hour"),
            Self::Day => f.write_str("day"),
            Self::Week => f.write_str("week"),
            Self::Month => f.write_str("month"),
        }
    }
}
impl ::std::str::FromStr for NewsRequestDtoTimeRange {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "hour" => Ok(Self::Hour),
            "day" => Ok(Self::Day),
            "week" => Ok(Self::Week),
            "month" => Ok(Self::Month),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NewsRequestDtoTimeRange {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NewsRequestDtoTimeRange {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for NewsRequestDtoTimeRange {
    fn default() -> Self {
        NewsRequestDtoTimeRange::Week
    }
}
#[doc = "`NewsResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NewsResponseDto {
    pub data: NewsResultDto,
}
#[doc = "`NewsResultDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NewsResultDto {
    #[doc = "The query you searched."]
    pub query: ::std::string::String,
    #[doc = "Articles, newest first."]
    pub results: ::std::vec::Vec<NewsResultItemDto>,
    #[doc = "`ok` when articles were found, `no_results` when none matched in the window. If news cannot be searched at all, the request fails with 503 rather than returning an empty list."]
    pub status: NewsResultDtoStatus,
}
#[doc = "`ok` when articles were found, `no_results` when none matched in the window. If news cannot be searched at all, the request fails with 503 rather than returning an empty list."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NewsResultDtoStatus {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "no_results")]
    NoResults,
}
impl ::std::fmt::Display for NewsResultDtoStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ok => f.write_str("ok"),
            Self::NoResults => f.write_str("no_results"),
        }
    }
}
impl ::std::str::FromStr for NewsResultDtoStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ok" => Ok(Self::Ok),
            "no_results" => Ok(Self::NoResults),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NewsResultDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NewsResultDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`NewsResultItemDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NewsResultItemDto {
    #[doc = "The article read as clean data, or null when `scrapeResults` is off or the page could not be read."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub data: ::std::option::Option<WebScrapeDataDto>,
    #[doc = "When it was published. Exact when the listing gives a time, otherwise worked out from its stated age, such as 2 days ago."]
    #[serde(rename = "publishedAt")]
    pub published_at: ::std::string::String,
    #[doc = "A short summary of the article."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub snippet: ::std::option::Option<::std::string::String>,
    #[doc = "The outlet that published it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub source: ::std::option::Option<::std::string::String>,
    #[doc = "The headline."]
    pub title: ::std::string::String,
    #[doc = "The article's address on the publisher's own site."]
    pub url: ::std::string::String,
}
#[doc = "`OptLocationDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct OptLocationDto {
    #[doc = "ISO 3166 alpha-2 country to fetch as if from."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub country: ::std::option::Option<::std::string::String>,
    #[doc = "Preferred content languages, most-preferred first."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub languages: ::std::vec::Vec<::std::string::String>,
}
#[doc = "`ResultFeedbackDataDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResultFeedbackDataDto {
    pub id: ::std::string::String,
    pub rating: ResultFeedbackDataDtoRating,
    #[serde(rename = "requestId")]
    pub request_id: ::std::string::String,
}
#[doc = "`ResultFeedbackDataDtoRating`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResultFeedbackDataDtoRating {
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "bad")]
    Bad,
}
impl ::std::fmt::Display for ResultFeedbackDataDtoRating {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Good => f.write_str("good"),
            Self::Bad => f.write_str("bad"),
        }
    }
}
impl ::std::str::FromStr for ResultFeedbackDataDtoRating {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "good" => Ok(Self::Good),
            "bad" => Ok(Self::Bad),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResultFeedbackDataDtoRating {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResultFeedbackDataDtoRating {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ResultFeedbackDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResultFeedbackDto {
    #[doc = "More detail, in your own words."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub note: ::std::option::Option<ResultFeedbackDtoNote>,
    #[doc = "Whether the result was accurate."]
    pub rating: ResultFeedbackDtoRating,
    #[doc = "What was wrong, for a bad rating."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub reasons: ::std::vec::Vec<ResultFeedbackDtoReasonsItem>,
    #[doc = "The request being rated: the X-Request-Id response header, or usage.requestId from an MCP tool. Give this or url."]
    #[serde(
        rename = "requestId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub request_id: ::std::option::Option<::std::string::String>,
    #[doc = "The URL that was read, when the request id is not to hand. Rates the latest request for it in the last 24 hours."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
#[doc = "More detail, in your own words."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ResultFeedbackDtoNote(::std::string::String);
impl ::std::ops::Deref for ResultFeedbackDtoNote {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ResultFeedbackDtoNote> for ::std::string::String {
    fn from(value: ResultFeedbackDtoNote) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ResultFeedbackDtoNote {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 500usize {
            return Err("longer than 500 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ResultFeedbackDtoNote {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResultFeedbackDtoNote {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ResultFeedbackDtoNote {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Whether the result was accurate."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResultFeedbackDtoRating {
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "bad")]
    Bad,
}
impl ::std::fmt::Display for ResultFeedbackDtoRating {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Good => f.write_str("good"),
            Self::Bad => f.write_str("bad"),
        }
    }
}
impl ::std::str::FromStr for ResultFeedbackDtoRating {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "good" => Ok(Self::Good),
            "bad" => Ok(Self::Bad),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResultFeedbackDtoRating {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResultFeedbackDtoRating {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ResultFeedbackDtoReasonsItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResultFeedbackDtoReasonsItem {
    #[serde(rename = "missing_content")]
    MissingContent,
    #[serde(rename = "wrong_content")]
    WrongContent,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "slow")]
    Slow,
    #[serde(rename = "formatting")]
    Formatting,
    #[serde(rename = "other")]
    Other,
}
impl ::std::fmt::Display for ResultFeedbackDtoReasonsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MissingContent => f.write_str("missing_content"),
            Self::WrongContent => f.write_str("wrong_content"),
            Self::Blocked => f.write_str("blocked"),
            Self::Slow => f.write_str("slow"),
            Self::Formatting => f.write_str("formatting"),
            Self::Other => f.write_str("other"),
        }
    }
}
impl ::std::str::FromStr for ResultFeedbackDtoReasonsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "missing_content" => Ok(Self::MissingContent),
            "wrong_content" => Ok(Self::WrongContent),
            "blocked" => Ok(Self::Blocked),
            "slow" => Ok(Self::Slow),
            "formatting" => Ok(Self::Formatting),
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResultFeedbackDtoReasonsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResultFeedbackDtoReasonsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ResultFeedbackResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResultFeedbackResponseDto {
    pub data: ResultFeedbackDataDto,
}
#[doc = "`ScrapeAcceptedDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ScrapeAcceptedDto {
    #[doc = "Poll this job id at GET /v1/web/scrape/{id}."]
    #[serde(rename = "jobId")]
    pub job_id: ::std::string::String,
    #[doc = "Only returned when the request asked for `async: true`."]
    pub status: ScrapeAcceptedDtoStatus,
}
#[doc = "Only returned when the request asked for `async: true`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ScrapeAcceptedDtoStatus {
    #[serde(rename = "queued")]
    Queued,
}
impl ::std::fmt::Display for ScrapeAcceptedDtoStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Queued => f.write_str("queued"),
        }
    }
}
impl ::std::str::FromStr for ScrapeAcceptedDtoStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "queued" => Ok(Self::Queued),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ScrapeAcceptedDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScrapeAcceptedDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ScrapeFormatRequestDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ScrapeFormatRequestDto {
    #[doc = "Return a job id immediately instead of waiting for the result. Poll GET /v1/web/scrape/{id}. Opt in only: a synchronous call never degrades to a job id on its own, so you get a page or an error, not two response shapes."]
    #[serde(
        rename = "async",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub async_: ::std::option::Option<bool>,
    #[doc = "Remove common ad and tracking elements. Default true."]
    #[serde(
        rename = "blockAds",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub block_ads: ::std::option::Option<bool>,
    #[doc = "Request body, sent verbatim. Only used when `method` is POST. Set the content type yourself via `headers`; nothing is assumed about the format."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub body: ::std::option::Option<ScrapeFormatRequestDtoBody>,
    #[doc = "Only serve from cache. If there is no fresh cached copy, return 404 instead of fetching."]
    #[serde(
        rename = "cacheOnly",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cache_only: ::std::option::Option<bool>,
    #[doc = "CSS selectors to strip before extraction."]
    #[serde(
        rename = "excludeTags",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exclude_tags: ::std::vec::Vec<::std::string::String>,
    #[doc = "Extra request headers to send when fetching the page."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub headers: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "Keep inline links in the markdown. By default links are kept wherever the page text carries them. Set true to keep them on every page, or false for the densest prose."]
    #[serde(
        rename = "includeLinks",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_links: ::std::option::Option<bool>,
    #[doc = "CSS selectors to keep. When set, only matching elements are considered."]
    #[serde(
        rename = "includeTags",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub include_tags: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationDto>,
    #[doc = "Serve from cache if a capture of this URL is younger than this many milliseconds. Omit for the default window, two days for a page and one hour for a feed; 0 always fetches fresh. Capped at 7 days."]
    #[serde(
        rename = "maxAge",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_age: ::std::option::Option<i64>,
    #[doc = "HTTP method to fetch the URL with. Use POST for endpoints that only answer a POST, such as JSON and GraphQL APIs. Defaults to GET."]
    #[serde(default = "defaults::scrape_format_request_dto_method")]
    pub method: ScrapeFormatRequestDtoMethod,
    #[doc = "Return only the main content, dropping nav/boilerplate. Default true."]
    #[serde(
        rename = "onlyMainContent",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub only_main_content: ::std::option::Option<bool>,
    #[doc = "Preserve document structure (headings, lists, tables) over prose density — good for marketing and service pages. Default off."]
    #[serde(
        rename = "preferStructure",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub prefer_structure: ::std::option::Option<bool>,
    #[doc = "Strip inline base64 images from the output. Default true."]
    #[serde(
        rename = "removeBase64Images",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub remove_base64_images: ::std::option::Option<bool>,
    #[doc = "Persist the capture for later re-extraction. Default true."]
    #[serde(
        rename = "storeInCache",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub store_in_cache: ::std::option::Option<bool>,
    #[doc = "Overall time budget for the request, in milliseconds."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout: ::std::option::Option<i64>,
    #[doc = "The URL to scrape. Must be http(s); a bare domain such as example.com is read over https."]
    pub url: ::std::string::String,
    #[doc = "Extra milliseconds to let the page settle before capture."]
    #[serde(
        rename = "waitFor",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub wait_for: ::std::option::Option<i64>,
}
#[doc = "Request body, sent verbatim. Only used when `method` is POST. Set the content type yourself via `headers`; nothing is assumed about the format."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ScrapeFormatRequestDtoBody(::std::string::String);
impl ::std::ops::Deref for ScrapeFormatRequestDtoBody {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ScrapeFormatRequestDtoBody> for ::std::string::String {
    fn from(value: ScrapeFormatRequestDtoBody) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ScrapeFormatRequestDtoBody {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 65536usize {
            return Err("longer than 65536 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ScrapeFormatRequestDtoBody {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScrapeFormatRequestDtoBody {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ScrapeFormatRequestDtoBody {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "HTTP method to fetch the URL with. Use POST for endpoints that only answer a POST, such as JSON and GraphQL APIs. Defaults to GET."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ScrapeFormatRequestDtoMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
}
impl ::std::fmt::Display for ScrapeFormatRequestDtoMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Get => f.write_str("GET"),
            Self::Post => f.write_str("POST"),
        }
    }
}
impl ::std::str::FromStr for ScrapeFormatRequestDtoMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ScrapeFormatRequestDtoMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScrapeFormatRequestDtoMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for ScrapeFormatRequestDtoMethod {
    fn default() -> Self {
        ScrapeFormatRequestDtoMethod::Get
    }
}
#[doc = "`ScrapeOptionsDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ScrapeOptionsDto {
    #[doc = "Remove common ad and tracking elements. Default true."]
    #[serde(
        rename = "blockAds",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub block_ads: ::std::option::Option<bool>,
    #[doc = "CSS selectors to strip before extraction."]
    #[serde(
        rename = "excludeTags",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exclude_tags: ::std::vec::Vec<::std::string::String>,
    #[doc = "What to return per page. Omit for Markdown only."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub formats: ::std::vec::Vec<ScrapeOptionsDtoFormatsItem>,
    #[doc = "Extra request headers to send when fetching each page."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub headers: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "Keep inline links in the markdown. By default links are kept wherever the page text carries them. Set true to keep them on every page, or false for the densest prose."]
    #[serde(
        rename = "includeLinks",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_links: ::std::option::Option<bool>,
    #[doc = "CSS selectors to keep."]
    #[serde(
        rename = "includeTags",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub include_tags: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<OptLocationDto>,
    #[doc = "Serve a page from cache if younger than this many milliseconds."]
    #[serde(
        rename = "maxAge",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_age: ::std::option::Option<i64>,
    #[doc = "Return only the main content, dropping boilerplate. Default true."]
    #[serde(
        rename = "onlyMainContent",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub only_main_content: ::std::option::Option<bool>,
    #[doc = "Preserve document structure over prose density. Default off."]
    #[serde(
        rename = "preferStructure",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub prefer_structure: ::std::option::Option<bool>,
    #[doc = "Strip inline base64 images. Default true."]
    #[serde(
        rename = "removeBase64Images",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub remove_base64_images: ::std::option::Option<bool>,
    #[doc = "Per-page time budget, in milliseconds."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout: ::std::option::Option<i64>,
    #[doc = "Extra milliseconds to let the page settle before capture."]
    #[serde(
        rename = "waitFor",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub wait_for: ::std::option::Option<i64>,
}
#[doc = "`ScrapeOptionsDtoFormatsItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ScrapeOptionsDtoFormatsItem {
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "rawHtml")]
    RawHtml,
    #[serde(rename = "links")]
    Links,
    #[serde(rename = "structured")]
    Structured,
}
impl ::std::fmt::Display for ScrapeOptionsDtoFormatsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Markdown => f.write_str("markdown"),
            Self::Html => f.write_str("html"),
            Self::RawHtml => f.write_str("rawHtml"),
            Self::Links => f.write_str("links"),
            Self::Structured => f.write_str("structured"),
        }
    }
}
impl ::std::str::FromStr for ScrapeOptionsDtoFormatsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "markdown" => Ok(Self::Markdown),
            "html" => Ok(Self::Html),
            "rawHtml" => Ok(Self::RawHtml),
            "links" => Ok(Self::Links),
            "structured" => Ok(Self::Structured),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ScrapeOptionsDtoFormatsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScrapeOptionsDtoFormatsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ScrapeRequestDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ScrapeRequestDto {
    #[doc = "Return a job id immediately instead of waiting for the result. Poll GET /v1/web/scrape/{id}. Opt in only: a synchronous call never degrades to a job id on its own, so you get a page or an error, not two response shapes."]
    #[serde(
        rename = "async",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub async_: ::std::option::Option<bool>,
    #[doc = "Remove common ad and tracking elements. Default true."]
    #[serde(
        rename = "blockAds",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub block_ads: ::std::option::Option<bool>,
    #[doc = "Request body, sent verbatim. Only used when `method` is POST. Set the content type yourself via `headers`; nothing is assumed about the format."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub body: ::std::option::Option<ScrapeRequestDtoBody>,
    #[doc = "Only serve from cache. If there is no fresh cached copy, return 404 instead of fetching."]
    #[serde(
        rename = "cacheOnly",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cache_only: ::std::option::Option<bool>,
    #[doc = "CSS selectors to strip before extraction."]
    #[serde(
        rename = "excludeTags",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exclude_tags: ::std::vec::Vec<::std::string::String>,
    #[doc = "What to return. Omit for Markdown only. `structured` is the page's own data (no LLM, cheapest); `summary` and `json` are LLM-backed and cost more."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub formats: ::std::vec::Vec<ScrapeRequestDtoFormatsItem>,
    #[doc = "Extra request headers to send when fetching the page."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub headers: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "Keep inline links in the markdown. By default links are kept wherever the page text carries them. Set true to keep them on every page, or false for the densest prose."]
    #[serde(
        rename = "includeLinks",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_links: ::std::option::Option<bool>,
    #[doc = "CSS selectors to keep. When set, only matching elements are considered."]
    #[serde(
        rename = "includeTags",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub include_tags: ::std::vec::Vec<::std::string::String>,
    #[doc = "Schema and/or prompt for the `json` format. One is required when `json` is requested."]
    #[serde(
        rename = "jsonOptions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub json_options: ::std::option::Option<JsonOptionsDto>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationDto>,
    #[doc = "Serve from cache if a capture of this URL is younger than this many milliseconds. Omit for the default window, two days for a page and one hour for a feed; 0 always fetches fresh. Capped at 7 days."]
    #[serde(
        rename = "maxAge",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_age: ::std::option::Option<f64>,
    #[doc = "HTTP method to fetch the URL with. Use POST for endpoints that only answer a POST, such as JSON and GraphQL APIs. Defaults to GET."]
    #[serde(default = "defaults::scrape_request_dto_method")]
    pub method: ScrapeRequestDtoMethod,
    #[doc = "Return only the main content, dropping nav/boilerplate. Default true."]
    #[serde(
        rename = "onlyMainContent",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub only_main_content: ::std::option::Option<bool>,
    #[doc = "Preserve document structure (headings, lists, tables) over prose density — good for marketing and service pages. Default off."]
    #[serde(
        rename = "preferStructure",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub prefer_structure: ::std::option::Option<bool>,
    #[doc = "Strip inline base64 images from the output. Default true."]
    #[serde(
        rename = "removeBase64Images",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub remove_base64_images: ::std::option::Option<bool>,
    #[doc = "Persist the capture for later re-extraction. Default true."]
    #[serde(
        rename = "storeInCache",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub store_in_cache: ::std::option::Option<bool>,
    #[doc = "Overall time budget for the request, in milliseconds."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout: ::std::option::Option<f64>,
    #[doc = "The URL to scrape. Must be http(s); a bare domain such as example.com is read over https."]
    pub url: ::std::string::String,
    #[doc = "Extra milliseconds to let the page settle before capture."]
    #[serde(
        rename = "waitFor",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub wait_for: ::std::option::Option<f64>,
}
#[doc = "Request body, sent verbatim. Only used when `method` is POST. Set the content type yourself via `headers`; nothing is assumed about the format."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ScrapeRequestDtoBody(::std::string::String);
impl ::std::ops::Deref for ScrapeRequestDtoBody {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ScrapeRequestDtoBody> for ::std::string::String {
    fn from(value: ScrapeRequestDtoBody) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ScrapeRequestDtoBody {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 65536usize {
            return Err("longer than 65536 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ScrapeRequestDtoBody {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScrapeRequestDtoBody {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ScrapeRequestDtoBody {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`ScrapeRequestDtoFormatsItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ScrapeRequestDtoFormatsItem {
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "rawHtml")]
    RawHtml,
    #[serde(rename = "links")]
    Links,
    #[serde(rename = "structured")]
    Structured,
    #[serde(rename = "summary")]
    Summary,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "brand")]
    Brand,
}
impl ::std::fmt::Display for ScrapeRequestDtoFormatsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Markdown => f.write_str("markdown"),
            Self::Html => f.write_str("html"),
            Self::RawHtml => f.write_str("rawHtml"),
            Self::Links => f.write_str("links"),
            Self::Structured => f.write_str("structured"),
            Self::Summary => f.write_str("summary"),
            Self::Json => f.write_str("json"),
            Self::Brand => f.write_str("brand"),
        }
    }
}
impl ::std::str::FromStr for ScrapeRequestDtoFormatsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "markdown" => Ok(Self::Markdown),
            "html" => Ok(Self::Html),
            "rawHtml" => Ok(Self::RawHtml),
            "links" => Ok(Self::Links),
            "structured" => Ok(Self::Structured),
            "summary" => Ok(Self::Summary),
            "json" => Ok(Self::Json),
            "brand" => Ok(Self::Brand),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ScrapeRequestDtoFormatsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScrapeRequestDtoFormatsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "HTTP method to fetch the URL with. Use POST for endpoints that only answer a POST, such as JSON and GraphQL APIs. Defaults to GET."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ScrapeRequestDtoMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
}
impl ::std::fmt::Display for ScrapeRequestDtoMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Get => f.write_str("GET"),
            Self::Post => f.write_str("POST"),
        }
    }
}
impl ::std::str::FromStr for ScrapeRequestDtoMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ScrapeRequestDtoMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScrapeRequestDtoMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for ScrapeRequestDtoMethod {
    fn default() -> Self {
        ScrapeRequestDtoMethod::Get
    }
}
#[doc = "`ScrapeResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ScrapeResponseDto {
    pub data: WebScrapeDataDto,
}
#[doc = "`ScrapeStatusResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ScrapeStatusResponseDto {
    #[doc = "Present when completed."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<WebScrapeDataDto>,
    #[doc = "Present when the job failed."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    #[doc = "Job state."]
    pub status: ScrapeStatusResponseDtoStatus,
}
#[doc = "Job state."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ScrapeStatusResponseDtoStatus {
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for ScrapeStatusResponseDtoStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Waiting => f.write_str("waiting"),
            Self::Active => f.write_str("active"),
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for ScrapeStatusResponseDtoStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "waiting" => Ok(Self::Waiting),
            "active" => Ok(Self::Active),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ScrapeStatusResponseDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScrapeStatusResponseDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ScreenshotRequestDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ScreenshotRequestDto {
    #[doc = "Capture the entire scrollable page instead of just the viewport. Default off."]
    #[serde(
        rename = "fullPage",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub full_page: ::std::option::Option<bool>,
    #[doc = "Reuse a stored screenshot if it is younger than this many milliseconds. Defaults to 7 days, so a repeated request returns the stored PNG with `cached: true` without re-rendering — which also means it returns immediately and does not apply `waitFor`. Pass 0 to force a fresh capture."]
    #[serde(
        rename = "maxAge",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_age: ::std::option::Option<u64>,
    #[doc = "Overall time budget for the request, in milliseconds."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout: ::std::option::Option<i64>,
    #[doc = "The URL to screenshot. Must be http(s)."]
    pub url: ::std::string::String,
    #[doc = "Screen size to render at. Defaults to desktop (1920x1080). Ignored when width/height are given explicitly."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub viewport: ::std::option::Option<ScreenshotRequestDtoViewport>,
    #[doc = "Exact viewport height in pixels. Overrides `viewport`. With fullPage the capture still extends to the full page height."]
    #[serde(
        rename = "viewportHeight",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub viewport_height: ::std::option::Option<i64>,
    #[doc = "Exact viewport width in pixels. Overrides `viewport`."]
    #[serde(
        rename = "viewportWidth",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub viewport_width: ::std::option::Option<i64>,
    #[doc = "Extra milliseconds to let the page settle before capture."]
    #[serde(
        rename = "waitFor",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub wait_for: ::std::option::Option<i64>,
}
#[doc = "Screen size to render at. Defaults to desktop (1920x1080). Ignored when width/height are given explicitly."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ScreenshotRequestDtoViewport {
    #[serde(rename = "desktop")]
    Desktop,
    #[serde(rename = "laptop")]
    Laptop,
    #[serde(rename = "tablet")]
    Tablet,
    #[serde(rename = "mobile")]
    Mobile,
}
impl ::std::fmt::Display for ScreenshotRequestDtoViewport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Desktop => f.write_str("desktop"),
            Self::Laptop => f.write_str("laptop"),
            Self::Tablet => f.write_str("tablet"),
            Self::Mobile => f.write_str("mobile"),
        }
    }
}
impl ::std::str::FromStr for ScreenshotRequestDtoViewport {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "desktop" => Ok(Self::Desktop),
            "laptop" => Ok(Self::Laptop),
            "tablet" => Ok(Self::Tablet),
            "mobile" => Ok(Self::Mobile),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ScreenshotRequestDtoViewport {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScreenshotRequestDtoViewport {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ScreenshotResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ScreenshotResponseDto {
    pub data: WebScreenshotDataDto,
}
#[doc = "`SearchRequestDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SearchRequestDto {
    #[doc = "ISO 3166 alpha-2 country code to bias results toward a region."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub country: ::std::option::Option<::std::string::String>,
    #[doc = "Drop results from these domains."]
    #[serde(
        rename = "excludeDomains",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exclude_domains: ::std::vec::Vec<::std::string::String>,
    #[doc = "Only return results from these domains."]
    #[serde(
        rename = "includeDomains",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub include_domains: ::std::vec::Vec<::std::string::String>,
    #[doc = "How many ranked results to return. Omit for the default."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "What to search the web for."]
    pub query: SearchRequestDtoQuery,
    #[serde(
        rename = "scrapeOptions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrape_options: ::std::option::Option<ScrapeOptionsDto>,
    #[doc = "When true, each result also comes back with its page fetched and returned as clean data, at 1 additional credit per page scraped. Defaults to false, which returns just the ranked title, url and snippet for the flat search credit."]
    #[serde(
        rename = "scrapeResults",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrape_results: ::std::option::Option<bool>,
    #[doc = "Restrict results to a recency window relative to now."]
    #[serde(
        rename = "timeRange",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub time_range: ::std::option::Option<SearchRequestDtoTimeRange>,
}
#[doc = "What to search the web for."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SearchRequestDtoQuery(::std::string::String);
impl ::std::ops::Deref for SearchRequestDtoQuery {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SearchRequestDtoQuery> for ::std::string::String {
    fn from(value: SearchRequestDtoQuery) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SearchRequestDtoQuery {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 500usize {
            return Err("longer than 500 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SearchRequestDtoQuery {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SearchRequestDtoQuery {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SearchRequestDtoQuery {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Restrict results to a recency window relative to now."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SearchRequestDtoTimeRange {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}
impl ::std::fmt::Display for SearchRequestDtoTimeRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Day => f.write_str("day"),
            Self::Week => f.write_str("week"),
            Self::Month => f.write_str("month"),
            Self::Year => f.write_str("year"),
        }
    }
}
impl ::std::str::FromStr for SearchRequestDtoTimeRange {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "day" => Ok(Self::Day),
            "week" => Ok(Self::Week),
            "month" => Ok(Self::Month),
            "year" => Ok(Self::Year),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SearchRequestDtoTimeRange {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SearchRequestDtoTimeRange {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SearchResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SearchResponseDto {
    pub data: SearchResultDto,
}
#[doc = "`SearchResultDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SearchResultDto {
    #[doc = "The query you searched."]
    pub query: ::std::string::String,
    #[doc = "The ranked results."]
    pub results: ::std::vec::Vec<SearchResultItemDto>,
    #[doc = "Whether the search found anything: `ok` when it returned results, `no_results` when the web had no match. An empty `results` with `no_results` is a real miss — if the search cannot be completed at all, the request fails with 503 rather than returning an empty list."]
    pub status: SearchResultDtoStatus,
}
#[doc = "Whether the search found anything: `ok` when it returned results, `no_results` when the web had no match. An empty `results` with `no_results` is a real miss — if the search cannot be completed at all, the request fails with 503 rather than returning an empty list."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SearchResultDtoStatus {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "no_results")]
    NoResults,
}
impl ::std::fmt::Display for SearchResultDtoStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ok => f.write_str("ok"),
            Self::NoResults => f.write_str("no_results"),
        }
    }
}
impl ::std::str::FromStr for SearchResultDtoStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ok" => Ok(Self::Ok),
            "no_results" => Ok(Self::NoResults),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SearchResultDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SearchResultDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SearchResultItemDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SearchResultItemDto {
    #[doc = "The scraped page for this result, or null when scraping is off or unavailable."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub data: ::std::option::Option<WebScrapeDataDto>,
    #[doc = "The result's position in the ranked list."]
    pub rank: i64,
    #[doc = "A short snippet describing the result."]
    pub snippet: ::std::string::String,
    #[doc = "The result title."]
    pub title: ::std::string::String,
    #[doc = "The result URL."]
    pub url: ::std::string::String,
}
#[doc = "`StyleguideBandDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StyleguideBandDto {
    pub background: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<i64>,
}
#[doc = "`StyleguideButtonBorderDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StyleguideButtonBorderDto {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub color: ::std::option::Option<::std::string::String>,
    pub style: ::std::string::String,
    pub width: f64,
}
#[doc = "`StyleguideButtonDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StyleguideButtonDto {
    #[doc = "The fill. Absent on an outlined button, which has only a border."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub border: ::std::option::Option<StyleguideButtonBorderDto>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<::std::string::String>,
    #[doc = "The same button as CSS declarations you can paste into a rule."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub css: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "fontSize",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub font_size: ::std::option::Option<f64>,
    #[serde(
        rename = "fontWeight",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub font_weight: ::std::option::Option<::std::string::String>,
    #[doc = "Height of the sampled button as it rendered, not a minimum the site sets."]
    #[serde(
        rename = "minHeight",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub min_height: ::std::option::Option<f64>,
    #[doc = "Width of the sampled button as it rendered, not a minimum the site sets. A button centring its label with flex has no padding to report, so this is the size to copy."]
    #[serde(
        rename = "minWidth",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub min_width: ::std::option::Option<f64>,
    #[serde(
        rename = "paddingX",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub padding_x: ::std::option::Option<f64>,
    #[serde(
        rename = "paddingY",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub padding_y: ::std::option::Option<f64>,
    pub radius: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadow: ::std::option::Option<::std::string::String>,
}
#[doc = "`StyleguideColorDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StyleguideColorDto {
    pub hex: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "Roles rather than a ranked list, so a colour drops straight into a theme instead of leaving you to guess which one is safe behind text."]
    pub role: StyleguideColorDtoRole,
}
#[doc = "Roles rather than a ranked list, so a colour drops straight into a theme instead of leaving you to guess which one is safe behind text."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum StyleguideColorDtoRole {
    #[serde(rename = "background")]
    Background,
    #[serde(rename = "surface")]
    Surface,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "accent")]
    Accent,
}
impl ::std::fmt::Display for StyleguideColorDtoRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Background => f.write_str("background"),
            Self::Surface => f.write_str("surface"),
            Self::Text => f.write_str("text"),
            Self::Accent => f.write_str("accent"),
        }
    }
}
impl ::std::str::FromStr for StyleguideColorDtoRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "background" => Ok(Self::Background),
            "surface" => Ok(Self::Surface),
            "text" => Ok(Self::Text),
            "accent" => Ok(Self::Accent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StyleguideColorDtoRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StyleguideColorDtoRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`StyleguideComponentsDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct StyleguideComponentsDto {
    #[doc = "A full-width section painted in a color far from the page, such as a dark hero band."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub band: ::std::option::Option<StyleguideBandDto>,
    #[doc = "The top navigation bar."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub header: ::std::option::Option<StyleguideHeaderDto>,
    #[doc = "Where a panel's title sits: across its top border like a fieldset legend, inside its padding, or just above it."]
    #[serde(
        rename = "headingOnPanel",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub heading_on_panel: ::std::option::Option<StyleguideComponentsDtoHeadingOnPanel>,
    #[doc = "The treatment most large boxes on the page share: cards, panels, framed sections."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub panel: ::std::option::Option<StyleguidePanelDto>,
    #[doc = "How the selected and other tabs are drawn."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tabs: ::std::option::Option<StyleguideTabsDto>,
}
#[doc = "Where a panel's title sits: across its top border like a fieldset legend, inside its padding, or just above it."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum StyleguideComponentsDtoHeadingOnPanel {
    #[serde(rename = "on-border")]
    OnBorder,
    #[serde(rename = "inside")]
    Inside,
    #[serde(rename = "above")]
    Above,
}
impl ::std::fmt::Display for StyleguideComponentsDtoHeadingOnPanel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OnBorder => f.write_str("on-border"),
            Self::Inside => f.write_str("inside"),
            Self::Above => f.write_str("above"),
        }
    }
}
impl ::std::str::FromStr for StyleguideComponentsDtoHeadingOnPanel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "on-border" => Ok(Self::OnBorder),
            "inside" => Ok(Self::Inside),
            "above" => Ok(Self::Above),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StyleguideComponentsDtoHeadingOnPanel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StyleguideComponentsDtoHeadingOnPanel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`StyleguideContrastDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct StyleguideContrastDto {
    #[serde(
        rename = "accentOnBackground",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub accent_on_background: ::std::option::Option<f64>,
    #[doc = "Whether that ratio clears WCAG AA."]
    #[serde(
        rename = "meetsAA",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meets_aa: ::std::option::Option<bool>,
    #[doc = "Contrast ratio of text against background."]
    #[serde(
        rename = "textOnBackground",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub text_on_background: ::std::option::Option<f64>,
}
#[doc = "`StyleguideDarkButtonDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StyleguideDarkButtonDto {
    pub background: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<::std::string::String>,
}
#[doc = "`StyleguideDarkDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StyleguideDarkDto {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub button: ::std::option::Option<StyleguideDarkButtonDto>,
    pub colors: ::std::vec::Vec<StyleguideColorDto>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub contrast: ::std::option::Option<StyleguideContrastDto>,
}
#[doc = "`StyleguideDecorationDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StyleguideDecorationDto {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<::std::string::String>,
    #[doc = "The pattern's image on the site's own server. Recreate it rather than linking to it."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image: ::std::option::Option<::std::string::String>,
    #[doc = "A layer the site draws behind or beside its panels: a patterned image, a solid block or an outline, offset to read as a shadow."]
    pub kind: StyleguideDecorationDtoKind,
    #[serde(
        rename = "offsetX",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub offset_x: ::std::option::Option<f64>,
    #[serde(
        rename = "offsetY",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub offset_y: ::std::option::Option<f64>,
}
#[doc = "A layer the site draws behind or beside its panels: a patterned image, a solid block or an outline, offset to read as a shadow."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum StyleguideDecorationDtoKind {
    #[serde(rename = "pattern")]
    Pattern,
    #[serde(rename = "fill")]
    Fill,
    #[serde(rename = "outline")]
    Outline,
}
impl ::std::fmt::Display for StyleguideDecorationDtoKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pattern => f.write_str("pattern"),
            Self::Fill => f.write_str("fill"),
            Self::Outline => f.write_str("outline"),
        }
    }
}
impl ::std::str::FromStr for StyleguideDecorationDtoKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pattern" => Ok(Self::Pattern),
            "fill" => Ok(Self::Fill),
            "outline" => Ok(Self::Outline),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StyleguideDecorationDtoKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StyleguideDecorationDtoKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`StyleguideFontDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StyleguideFontDto {
    #[doc = "The rest of the declared stack, for rendering before the webfont arrives."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub fallbacks: ::std::vec::Vec<::std::string::String>,
    pub family: ::std::string::String,
    #[serde(
        rename = "letterSpacing",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub letter_spacing: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "lineHeight",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub line_height: ::std::option::Option<f64>,
    pub role: StyleguideFontDtoRole,
    #[doc = "Computed size in pixels."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub weight: ::std::option::Option<::std::string::String>,
}
#[doc = "`StyleguideFontDtoRole`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum StyleguideFontDtoRole {
    #[serde(rename = "h1")]
    H1,
    #[serde(rename = "h2")]
    H2,
    #[serde(rename = "h3")]
    H3,
    #[serde(rename = "h4")]
    H4,
    #[serde(rename = "body")]
    Body,
    #[serde(rename = "link")]
    Link,
}
impl ::std::fmt::Display for StyleguideFontDtoRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::H1 => f.write_str("h1"),
            Self::H2 => f.write_str("h2"),
            Self::H3 => f.write_str("h3"),
            Self::H4 => f.write_str("h4"),
            Self::Body => f.write_str("body"),
            Self::Link => f.write_str("link"),
        }
    }
}
impl ::std::str::FromStr for StyleguideFontDtoRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "h1" => Ok(Self::H1),
            "h2" => Ok(Self::H2),
            "h3" => Ok(Self::H3),
            "h4" => Ok(Self::H4),
            "body" => Ok(Self::Body),
            "link" => Ok(Self::Link),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StyleguideFontDtoRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StyleguideFontDtoRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`StyleguideFontUsageDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StyleguideFontUsageDto {
    pub family: ::std::string::String,
    #[serde(rename = "percentWords")]
    pub percent_words: f64,
    pub words: i64,
}
#[doc = "`StyleguideHeaderBorderDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct StyleguideHeaderBorderDto {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<f64>,
}
#[doc = "`StyleguideHeaderDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StyleguideHeaderDto {
    pub background: ::std::string::String,
    #[serde(
        rename = "borderBottom",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub border_bottom: ::std::option::Option<StyleguideHeaderBorderDto>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub css: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<f64>,
    #[doc = "Whether the bar stays pinned while the page scrolls."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sticky: ::std::option::Option<bool>,
}
#[doc = "`StyleguidePanelBorderDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct StyleguidePanelBorderDto {
    #[doc = "A hex color, or rgba() when the border is translucent."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub style: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<f64>,
}
#[doc = "`StyleguidePanelDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct StyleguidePanelDto {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub border: ::std::option::Option<StyleguidePanelBorderDto>,
    #[doc = "Text color inside the panel."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<::std::string::String>,
    #[doc = "How many boxes on the page share this treatment."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<i64>,
    #[doc = "The panel as CSS declarations. A patterned decoration is not included; read `decoration` for it."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub css: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub decoration: ::std::option::Option<StyleguideDecorationDto>,
    #[doc = "Vertical then horizontal padding."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub padding: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub radius: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadow: ::std::option::Option<::std::string::String>,
}
#[doc = "`StyleguideRadiusDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct StyleguideRadiusDto {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub button: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub card: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<::std::string::String>,
}
#[doc = "`StyleguideResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StyleguideResponseDto {
    #[doc = "Null when the domain could not be rendered. An unresolved domain is free."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub data: ::std::option::Option<WebStyleguideDataDto>,
}
#[doc = "`StyleguideTabDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StyleguideTabDto {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub border: ::std::option::Option<StyleguideHeaderBorderDto>,
    #[doc = "Absent with `border` set means the tab is open at the bottom, joined to the panel below."]
    #[serde(
        rename = "borderBottom",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub border_bottom: ::std::option::Option<StyleguideHeaderBorderDto>,
    pub color: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub css: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "fontWeight",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub font_weight: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub radius: ::std::option::Option<::std::string::String>,
}
#[doc = "`StyleguideTabsDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct StyleguideTabsDto {
    #[doc = "The selected tab."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub active: ::std::option::Option<StyleguideTabDto>,
    #[doc = "A tab that is not selected."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub inactive: ::std::option::Option<StyleguideTabDto>,
}
#[doc = "`StyleguideWebfontDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StyleguideWebfontDto {
    #[doc = "Weight to file URL on the site's own servers, for a family that has no public source. The files are licensed to the site, so check the licence before linking to them. Empty for a Google family: use `stylesheet` instead."]
    pub files: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "For a family published on Google Fonts, its Google stylesheet, even when the site hosts its own copy."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stylesheet: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: StyleguideWebfontDtoType,
}
#[doc = "`StyleguideWebfontDtoType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum StyleguideWebfontDtoType {
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "custom")]
    Custom,
}
impl ::std::fmt::Display for StyleguideWebfontDtoType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Google => f.write_str("google"),
            Self::Custom => f.write_str("custom"),
        }
    }
}
impl ::std::str::FromStr for StyleguideWebfontDtoType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "google" => Ok(Self::Google),
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StyleguideWebfontDtoType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StyleguideWebfontDtoType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebBrandDataDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebBrandDataDto {
    #[doc = "Only when this brand is known by more than its canonical domain."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub aliases: ::std::vec::Vec<::std::string::String>,
    pub assets: ::std::vec::Vec<BrandAssetDto>,
    pub colors: ::std::vec::Vec<BrandColorDto>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The canonical domain for this company."]
    pub domain: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    pub fonts: ::std::vec::Vec<BrandFontDto>,
    #[serde(
        rename = "legalName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub legal_name: ::std::option::Option<::std::string::String>,
    pub links: BrandLinksDto,
    #[doc = "Industry, as a real NAICS code rather than a phrase nobody can group on."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub naics: ::std::option::Option<BrandNaicsDto>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "Only when you asked for a different domain that resolved here."]
    #[serde(
        rename = "queriedDomain",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub queried_domain: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "resolvedAt",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub resolved_at: ::std::option::Option<::std::string::String>,
    pub socials: ::std::vec::Vec<BrandSocialDto>,
    #[doc = "Per-field provenance, keyed by field name. Lets you tell \"their markup says this\" from \"a model guessed this\"."]
    pub sources: ::std::collections::HashMap<::std::string::String, BrandFieldProvenanceDto>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub tagline: ::std::option::Option<::std::string::String>,
}
#[doc = "`WebBrandSearchDataDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebBrandSearchDataDto {
    pub query: ::std::string::String,
    #[doc = "Best match first."]
    pub results: ::std::vec::Vec<BrandSearchResultDto>,
}
#[doc = "Every failure on this API returns this shape, whatever the status. Successful responses never carry it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebError {
    pub error: WebErrorBody,
    pub meta: WebErrorMeta,
    pub success: bool,
}
#[doc = "`WebErrorBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebErrorBody {
    #[doc = "Stable, machine-readable reason. Branch on this rather than on the message, which may be reworded."]
    pub code: WebErrorBodyCode,
    #[doc = "Optional machine-readable context, present on validation failures."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Human-readable explanation. Safe to show a user, not to parse."]
    pub message: ::std::string::String,
}
#[doc = "Stable, machine-readable reason. Branch on this rather than on the message, which may be reworded."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebErrorBodyCode {
    #[serde(rename = "VALIDATION_ERROR")]
    ValidationError,
    #[serde(rename = "INVALID_INPUT")]
    InvalidInput,
    #[serde(rename = "UNAUTHORIZED")]
    Unauthorized,
    #[serde(rename = "INSUFFICIENT_CREDITS")]
    InsufficientCredits,
    #[serde(rename = "FORBIDDEN")]
    Forbidden,
    #[serde(rename = "WORKSPACE_BANNED")]
    WorkspaceBanned,
    #[serde(rename = "NOT_FOUND")]
    NotFound,
    #[serde(rename = "TOO_MANY_REQUESTS")]
    TooManyRequests,
    #[serde(rename = "UPSTREAM_UNREACHABLE")]
    UpstreamUnreachable,
    #[serde(rename = "SERVICE_UNAVAILABLE")]
    ServiceUnavailable,
    #[serde(rename = "INTERNAL_SERVER_ERROR")]
    InternalServerError,
}
impl ::std::fmt::Display for WebErrorBodyCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ValidationError => f.write_str("VALIDATION_ERROR"),
            Self::InvalidInput => f.write_str("INVALID_INPUT"),
            Self::Unauthorized => f.write_str("UNAUTHORIZED"),
            Self::InsufficientCredits => f.write_str("INSUFFICIENT_CREDITS"),
            Self::Forbidden => f.write_str("FORBIDDEN"),
            Self::WorkspaceBanned => f.write_str("WORKSPACE_BANNED"),
            Self::NotFound => f.write_str("NOT_FOUND"),
            Self::TooManyRequests => f.write_str("TOO_MANY_REQUESTS"),
            Self::UpstreamUnreachable => f.write_str("UPSTREAM_UNREACHABLE"),
            Self::ServiceUnavailable => f.write_str("SERVICE_UNAVAILABLE"),
            Self::InternalServerError => f.write_str("INTERNAL_SERVER_ERROR"),
        }
    }
}
impl ::std::str::FromStr for WebErrorBodyCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "VALIDATION_ERROR" => Ok(Self::ValidationError),
            "INVALID_INPUT" => Ok(Self::InvalidInput),
            "UNAUTHORIZED" => Ok(Self::Unauthorized),
            "INSUFFICIENT_CREDITS" => Ok(Self::InsufficientCredits),
            "FORBIDDEN" => Ok(Self::Forbidden),
            "WORKSPACE_BANNED" => Ok(Self::WorkspaceBanned),
            "NOT_FOUND" => Ok(Self::NotFound),
            "TOO_MANY_REQUESTS" => Ok(Self::TooManyRequests),
            "UPSTREAM_UNREACHABLE" => Ok(Self::UpstreamUnreachable),
            "SERVICE_UNAVAILABLE" => Ok(Self::ServiceUnavailable),
            "INTERNAL_SERVER_ERROR" => Ok(Self::InternalServerError),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebErrorBodyCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebErrorBodyCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebErrorMeta`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct WebErrorMeta {
    #[doc = "Quote this when contacting support about a specific failure."]
    #[serde(
        rename = "requestId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub request_id: ::std::option::Option<::std::string::String>,
}
#[doc = "`WebImageDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebImageDto {
    #[doc = "The image alt text, if any."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub alt: ::std::option::Option<::std::string::String>,
    #[doc = "The image height in pixels, if declared."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub height: ::std::option::Option<i64>,
    #[doc = "The image URL, resolved to absolute."]
    pub src: ::std::string::String,
    #[doc = "The image width in pixels, if declared."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub width: ::std::option::Option<i64>,
}
#[doc = "`WebImagesDataDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebImagesDataDto {
    #[doc = "The final URL after any redirects."]
    #[serde(rename = "finalUrl")]
    pub final_url: ::std::string::String,
    #[doc = "Every image found on the page, with its metadata."]
    pub images: ::std::vec::Vec<WebImageDto>,
    #[doc = "The URL you requested."]
    pub url: ::std::string::String,
}
#[doc = "`WebLinksDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebLinksDto {
    #[doc = "Links pointing to other sites."]
    pub external: ::std::vec::Vec<::std::string::String>,
    #[doc = "Links pointing to the same site."]
    pub internal: ::std::vec::Vec<::std::string::String>,
}
#[doc = "`WebQualityDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebQualityDto {
    #[doc = "Whether the page appeared to be behind a challenge or bot wall."]
    pub blocked: bool,
    #[doc = "Whether the result captured the bulk of the content available on the page. False when the result looks thin or truncated."]
    pub complete: bool,
    #[doc = "How trustworthy the extraction is, from 0 to 1. High when independent checks corroborate substantial content; near zero when the page yielded almost nothing."]
    pub confidence: f64,
}
#[doc = "`WebScrapeDataDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebScrapeDataDto {
    #[doc = "The site's brand: logos, colours and fonts. Returned when `brand` is requested."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub brand: ::std::option::Option<WebBrandDataDto>,
    #[doc = "Whether this result was served from cache."]
    pub cached: bool,
    #[doc = "The final URL after any redirects."]
    #[serde(rename = "finalUrl")]
    pub final_url: ::std::string::String,
    #[doc = "Cleaned main-content HTML. Returned when `html` is requested."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub html: ::std::option::Option<::std::string::String>,
    #[doc = "Schema-shaped JSON. Returned when `json` is requested (LLM-backed)."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub json: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Returned when `links` is requested."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub links: ::std::option::Option<WebLinksDto>,
    #[doc = "Clean Markdown of the main content. Returned when `markdown` is requested."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub markdown: ::std::option::Option<::std::string::String>,
    pub metadata: WebScrapeMetadataDto,
    #[doc = "Per-page extraction quality signals."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<WebQualityDto>,
    #[doc = "The unmodified page HTML. Returned when `rawHtml` is requested."]
    #[serde(
        rename = "rawHtml",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub raw_html: ::std::option::Option<::std::string::String>,
    #[doc = "Whether the origin redirected: true when `finalUrl` differs from the URL you requested. Explicit so a page with no redirect is distinguishable from one whose redirect was not tracked."]
    pub redirected: bool,
    #[doc = "HTTP status of the fetched page."]
    pub status: i64,
    #[doc = "Returned when `structured` is requested."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub structured: ::std::option::Option<WebStructuredDataDto>,
    #[doc = "A concise factual summary. Returned when `summary` is requested (LLM-backed)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "The URL you requested."]
    pub url: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<WebUsageDto>,
    #[doc = "Set when the page was returned with a caveat (e.g. partial content)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub warning: ::std::option::Option<::std::string::String>,
}
#[doc = "`WebScrapeMetadataDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebScrapeMetadataDto {
    #[doc = "The declared author."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub author: ::std::option::Option<::std::string::String>,
    #[doc = "The page's own description (meta description / og:description)."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The page's lead image (og:image)."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub image: ::std::option::Option<::std::string::String>,
    #[doc = "The language the page declares."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub language: ::std::option::Option<::std::string::String>,
    #[doc = "When the page says it was last updated (ISO 8601)."]
    #[serde(
        rename = "modifiedTime",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub modified_time: ::std::option::Option<::std::string::String>,
    #[doc = "Coarse page classification (e.g. article, listing, forum, docs)."]
    #[serde(rename = "pageType")]
    pub page_type: ::std::string::String,
    #[doc = "When the page says it was published (ISO 8601)."]
    #[serde(
        rename = "publishedTime",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub published_time: ::std::option::Option<::std::string::String>,
    #[doc = "The declared site name."]
    #[serde(
        rename = "siteName",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub site_name: ::std::option::Option<::std::string::String>,
    #[doc = "Whether the content carries markdown structure (headings, lists, emphasis) or is plain prose. `plain` is not an error — the default extractor optimises for capturing every word and wins on listing and forum pages. If you need the structure, re-request with `preferStructure: true`."]
    pub structure: WebScrapeMetadataDtoStructure,
    #[doc = "The page title."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub title: ::std::option::Option<::std::string::String>,
    #[doc = "Word count of the extracted main content."]
    #[serde(rename = "wordCount")]
    pub word_count: i64,
}
#[doc = "Whether the content carries markdown structure (headings, lists, emphasis) or is plain prose. `plain` is not an error — the default extractor optimises for capturing every word and wins on listing and forum pages. If you need the structure, re-request with `preferStructure: true`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebScrapeMetadataDtoStructure {
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "plain")]
    Plain,
}
impl ::std::fmt::Display for WebScrapeMetadataDtoStructure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Markdown => f.write_str("markdown"),
            Self::Plain => f.write_str("plain"),
        }
    }
}
impl ::std::str::FromStr for WebScrapeMetadataDtoStructure {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "markdown" => Ok(Self::Markdown),
            "plain" => Ok(Self::Plain),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebScrapeMetadataDtoStructure {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebScrapeMetadataDtoStructure {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebScreenshotDataDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebScreenshotDataDto {
    #[doc = "True when a stored screenshot was reused instead of capturing a new one."]
    pub cached: bool,
    #[doc = "The final URL after any redirects."]
    #[serde(rename = "finalUrl")]
    pub final_url: ::std::string::String,
    #[doc = "Image height in pixels."]
    pub height: i64,
    #[doc = "Public URL of the captured PNG. Stable for the same URL and options."]
    pub screenshot: ::std::string::String,
    #[doc = "Whether the capture covers the whole scrollable page or just the viewport."]
    #[serde(rename = "screenshotType")]
    pub screenshot_type: WebScreenshotDataDtoScreenshotType,
    #[doc = "HTTP status of the captured page."]
    pub status: i64,
    #[doc = "The URL you requested."]
    pub url: ::std::string::String,
    #[doc = "Image width in pixels."]
    pub width: i64,
}
#[doc = "Whether the capture covers the whole scrollable page or just the viewport."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebScreenshotDataDtoScreenshotType {
    #[serde(rename = "fullPage")]
    FullPage,
    #[serde(rename = "viewport")]
    Viewport,
}
impl ::std::fmt::Display for WebScreenshotDataDtoScreenshotType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FullPage => f.write_str("fullPage"),
            Self::Viewport => f.write_str("viewport"),
        }
    }
}
impl ::std::str::FromStr for WebScreenshotDataDtoScreenshotType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "fullPage" => Ok(Self::FullPage),
            "viewport" => Ok(Self::Viewport),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebScreenshotDataDtoScreenshotType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebScreenshotDataDtoScreenshotType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebStructuredDataDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebStructuredDataDto {
    #[doc = "Names of embedded framework app-state blocks detected on the page."]
    #[serde(rename = "appState")]
    pub app_state: ::std::vec::Vec<::std::string::String>,
    #[doc = "The page's own structured data, normalised into one deduplicated list of typed entities."]
    pub entities: ::std::vec::Vec<WebStructuredEntityDto>,
    #[doc = "Raw JSON-LD blocks, as found on the page."]
    #[serde(rename = "jsonLd")]
    pub json_ld: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "Raw microdata items."]
    pub microdata: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "Raw OpenGraph/Twitter-card tags."]
    pub opengraph: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
}
#[doc = "`WebStructuredEntityDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebStructuredEntityDto {
    #[doc = "The entity's properties, as published."]
    pub properties: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Which structured-data syntax the entity came from."]
    pub source: WebStructuredEntityDtoSource,
    #[doc = "The schema.org type of the entity."]
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "Which structured-data syntax the entity came from."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebStructuredEntityDtoSource {
    #[serde(rename = "json-ld")]
    JsonLd,
    #[serde(rename = "microdata")]
    Microdata,
    #[serde(rename = "opengraph")]
    Opengraph,
}
impl ::std::fmt::Display for WebStructuredEntityDtoSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::JsonLd => f.write_str("json-ld"),
            Self::Microdata => f.write_str("microdata"),
            Self::Opengraph => f.write_str("opengraph"),
        }
    }
}
impl ::std::str::FromStr for WebStructuredEntityDtoSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "json-ld" => Ok(Self::JsonLd),
            "microdata" => Ok(Self::Microdata),
            "opengraph" => Ok(Self::Opengraph),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebStructuredEntityDtoSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebStructuredEntityDtoSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebStyleguideDataDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebStyleguideDataDto {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub button: ::std::option::Option<StyleguideButtonDto>,
    pub colors: ::std::vec::Vec<StyleguideColorDto>,
    #[doc = "How the parts are drawn, beyond the tokens: panel borders and decorative shadows, the header bar, and where headings sit on panels. This is what makes a site recognisable."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub components: ::std::option::Option<StyleguideComponentsDto>,
    #[doc = "Whether you can put text on what we handed you."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub contrast: ::std::option::Option<StyleguideContrastDto>,
    #[doc = "The same design as a stylesheet you can paste in: `@font-face` rules for the webfonts and a `:root` block of CSS variables for colors, fonts, the type scale, radius, shadows and spacing. Values that are not plain colors, lengths, font names or https font files are left out."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub css: ::std::option::Option<::std::string::String>,
    #[doc = "The colors the site switches to when the visitor prefers a dark theme. Null means we checked and the site does not change, so do not invent a dark palette from it. Absent on guides read before we checked."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dark: ::std::option::Option<StyleguideDarkDto>,
    pub domain: ::std::string::String,
    #[serde(
        rename = "fontUsage",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub font_usage: ::std::vec::Vec<StyleguideFontUsageDto>,
    #[doc = "Whether the page as first loaded is light or dark. `colors` describe this mode."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mode: ::std::option::Option<WebStyleguideDataDtoMode>,
    #[doc = "The page that was read, when you asked for one with `url`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub radius: ::std::option::Option<StyleguideRadiusDto>,
    #[serde(
        rename = "resolvedAt",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub resolved_at: ::std::option::Option<::std::string::String>,
    #[doc = "The second call to action, painted differently from the first, or outlined."]
    #[serde(
        rename = "secondaryButton",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub secondary_button: ::std::option::Option<StyleguideButtonDto>,
    #[doc = "The colors, radius and fonts under shadcn/ui variable names (`--background`, `--foreground`, `--primary`, `--card`, `--muted`, `--border` and the rest). Replace the `:root` block in a shadcn project's globals.css with it."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadcn: ::std::option::Option<::std::string::String>,
    #[doc = "Keyed by size, for example `sm`, `md`, `lg`."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub shadows: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "Keyed by step, for example `xs` through `xl`."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub spacing: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "The same tokens as a Tailwind CSS v4 stylesheet. Paste it as your main CSS file and classes such as `bg-accent`, `font-heading`, `text-h1`, `rounded-button` and `shadow-md` use the site's design."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tailwind: ::std::option::Option<::std::string::String>,
    pub typography: ::std::vec::Vec<StyleguideFontDto>,
    #[doc = "Keyed by family name."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub webfonts: ::std::collections::HashMap<::std::string::String, StyleguideWebfontDto>,
}
#[doc = "Whether the page as first loaded is light or dark. `colors` describe this mode."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebStyleguideDataDtoMode {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
}
impl ::std::fmt::Display for WebStyleguideDataDtoMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Light => f.write_str("light"),
            Self::Dark => f.write_str("dark"),
        }
    }
}
impl ::std::str::FromStr for WebStyleguideDataDtoMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebStyleguideDataDtoMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebStyleguideDataDtoMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebUsageDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebUsageDto {
    #[doc = "Credits left in your workspace's balance after this call."]
    #[serde(rename = "creditsRemaining")]
    pub credits_remaining: i64,
    #[doc = "Credits this call consumed. Charged only on success."]
    #[serde(rename = "creditsUsed")]
    pub credits_used: i64,
    #[doc = "Whether the result was served from cache or freshly fetched."]
    pub freshness: WebUsageDtoFreshness,
}
#[doc = "Whether the result was served from cache or freshly fetched."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebUsageDtoFreshness {
    #[serde(rename = "cache")]
    Cache,
    #[serde(rename = "fresh")]
    Fresh,
}
impl ::std::fmt::Display for WebUsageDtoFreshness {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cache => f.write_str("cache"),
            Self::Fresh => f.write_str("fresh"),
        }
    }
}
impl ::std::str::FromStr for WebUsageDtoFreshness {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "cache" => Ok(Self::Cache),
            "fresh" => Ok(Self::Fresh),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebUsageDtoFreshness {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebUsageDtoFreshness {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebhookDeliveriesResponseDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebhookDeliveriesResponseDto {
    pub deliveries: ::std::vec::Vec<WebhookDeliveryDto>,
}
#[doc = "`WebhookDeliveryDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebhookDeliveryDto {
    #[doc = "How many delivery attempts have been made."]
    pub attempts: i64,
    #[doc = "When the delivery was first created (ISO 8601)."]
    #[serde(rename = "createdAt")]
    pub created_at: ::std::string::String,
    #[doc = "When the delivery succeeded (ISO 8601)."]
    #[serde(
        rename = "deliveredAt",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub delivered_at: ::std::option::Option<::std::string::String>,
    #[doc = "Why the last attempt failed, if it did."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub error: ::std::option::Option<::std::string::String>,
    #[doc = "The event this delivery carried."]
    pub event: ::std::string::String,
    pub id: ::std::string::String,
    #[doc = "HTTP status your endpoint returned on the last attempt."]
    #[serde(
        rename = "responseStatus",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub response_status: ::std::option::Option<i64>,
    #[doc = "pending while retries remain, success once delivered, failed once given up."]
    pub status: WebhookDeliveryDtoStatus,
}
#[doc = "pending while retries remain, success once delivered, failed once given up."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebhookDeliveryDtoStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for WebhookDeliveryDtoStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pending => f.write_str("pending"),
            Self::Success => f.write_str("success"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for WebhookDeliveryDtoStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pending" => Ok(Self::Pending),
            "success" => Ok(Self::Success),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebhookDeliveryDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebhookDeliveryDtoStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebhookDto`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WebhookDto {
    #[doc = "Which events to receive. `page` fires as each page finishes; `completed` fires once when the job ends. Defaults to both."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub events: ::std::vec::Vec<WebhookDtoEventsItem>,
    #[doc = "Extra headers to send with each callback, e.g. for authentication."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub headers: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "A shared secret. When set, each callback carries an X-Hydrafetch-Signature header you can verify to prove the request came from us and is not a replay."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub secret: ::std::option::Option<WebhookDtoSecret>,
    #[doc = "The URL to POST events to. Must be a public http(s) URL — private, loopback, and link-local addresses are rejected."]
    pub url: ::std::string::String,
}
#[doc = "`WebhookDtoEventsItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebhookDtoEventsItem {
    #[serde(rename = "page")]
    Page,
    #[serde(rename = "completed")]
    Completed,
}
impl ::std::fmt::Display for WebhookDtoEventsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Page => f.write_str("page"),
            Self::Completed => f.write_str("completed"),
        }
    }
}
impl ::std::str::FromStr for WebhookDtoEventsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "page" => Ok(Self::Page),
            "completed" => Ok(Self::Completed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebhookDtoEventsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebhookDtoEventsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A shared secret. When set, each callback carries an X-Hydrafetch-Signature header you can verify to prove the request came from us and is not a replay."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct WebhookDtoSecret(::std::string::String);
impl ::std::ops::Deref for WebhookDtoSecret {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<WebhookDtoSecret> for ::std::string::String {
    fn from(value: WebhookDtoSecret) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for WebhookDtoSecret {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256usize {
            return Err("longer than 256 characters".into());
        }
        if value.chars().count() < 16usize {
            return Err("shorter than 16 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for WebhookDtoSecret {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebhookDtoSecret {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for WebhookDtoSecret {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = " Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_nzu64<T, const V: u64>() -> T
    where
        T: ::std::convert::TryFrom<::std::num::NonZeroU64>,
        <T as ::std::convert::TryFrom<::std::num::NonZeroU64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(::std::num::NonZeroU64::try_from(V).unwrap()).unwrap()
    }
    pub(super) fn map_request_dto_order() -> super::MapRequestDtoOrder {
        super::MapRequestDtoOrder::Newest
    }
    pub(super) fn news_request_dto_time_range() -> super::NewsRequestDtoTimeRange {
        super::NewsRequestDtoTimeRange::Week
    }
    pub(super) fn scrape_format_request_dto_method() -> super::ScrapeFormatRequestDtoMethod {
        super::ScrapeFormatRequestDtoMethod::Get
    }
    pub(super) fn scrape_request_dto_method() -> super::ScrapeRequestDtoMethod {
        super::ScrapeRequestDtoMethod::Get
    }
}
#[doc = " Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
