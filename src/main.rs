#[allow(non_snake_case, unused_variables, dead_code)]
use reqwest::header::{ACCEPT, HeaderMap};
use dotenv::dotenv;
use std::env;
use serde::Deserialize;

// async fn get_url(url: String) -> () {
//     dotenv().ok();
//     let zsessionid = env::var("ZSESSIONID").unwrap();

//     let client = reqwest::Client::new();
//     let mut headers = HeaderMap::new();
//     headers.insert(ACCEPT, "application/json".parse().unwrap());
//     headers.insert("zsessionid", zsessionid.parse().unwrap());
//     let resp = client
//         .get(url)
//         .headers(headers)
//         .send()
//         .await?;
//     resp;
// }

// async fn get_defect_suite(ds_id: String) -> DefectSuite {
//     let url = format!("https://rally1.rallydev.com/slm/webservice/v2.0/DefectSuite?query=(FormattedID = {})", ds_id);
//     let ds = get_url(String::from(url)).await;
//     ds.Response.json::<DefectSuite>().await.unwrap()
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let zsessionid = env::var("ZSESSIONID").unwrap();

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("zsessionid", zsessionid.parse().unwrap());
    // TODO: Remove this hardcoded DS ID
    // TODO: Figure out how to factor out commonly used functinality to a library.
    let ds_id = "DS2820";
    let url = format!("https://rally1.rallydev.com/slm/webservice/v2.0/DefectSuite?query=(FormattedID = {})", ds_id);
    let resp = client
        .get(url)
        .headers(headers)
        .send()
        .await?;
    
    println!("{:#?}", resp);
    
    let ds_json = resp.json::<JsonResult>().await?;

    // let response = get_defect_suite(String::from("DS2820"));
    println!("{:#?}", ds_json);
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
