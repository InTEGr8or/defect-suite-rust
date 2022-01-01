#[allow(non_snake_case, unused_variables, dead_code)]
use reqwest::header::{ACCEPT, HeaderMap};
use dotenv::dotenv;
use std::env;
use serde::Deserialize;

use ureq::{Agent, AgentBuilder};
use std::time::Duration;

let agent: Agent = ureq::AgentBuilder::new()
    .timeout_read(Duration::from_secs(5))
    .timeout_write(Duration::from_secs(5))
    .build();

let body: String = agent
    .get("https://rally1.rallydev.com/slm/webservice/v2.0/DefectSuite")
    .call()?
    .into_json()?;

// Reuses the connection from previous request.
let response: String = agent
    .put("http://example.com/upload")
    .set("Authorization", "example-token")
    .call()?
    .into_json()?;

#[tokio::main]
async fn main() -> Result<(), ureq::Error> {
    let body: String = ureq::get("http://example.com")
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;
    Ok(())
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct DefectSuiteFullResponse {
    Name: String,
    description: String,
    approved_project: String,
    found_in_version: String,
    verified_in_version: String,
    impact: String,
    likelihood: String,
    priority: bool,
    product: String,
    activity: String,
    workaround: bool,
    environment: String,
    salesforce_case_number: String,
    severity: String,
    state: String,
    project: String,
    defect_suites: String,
    expedite: bool,
    display_color: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct DefectSuite {
    _rallyAPIMajor: String,
    _rallyAPIMinor: String,
    _ref: String,
    _refObjectName: String,
    _refObjectUUID: String,
    _type: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct QueryResult {
    _rallyAPIMajor: String,
    _rallyAPIMinor: String,
    Errors: Vec<String>,
    Warnings: Vec<String>,
    TotalResultCount: u64,
    StartIndex: u64,
    PageSize: u64,
    Results: Vec<DefectSuite>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct JsonResult {
    QueryResult: QueryResult,
}
