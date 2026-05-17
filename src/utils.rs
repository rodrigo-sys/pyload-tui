use std::collections::HashMap;

use openapi::apis::Error;
use openapi::apis::configuration::{ApiKey, Configuration};
use openapi::apis::py_load_rest_api::{
    self, ApiAddFilesPostError, ApiAddPackagePostError, api_add_files_post, api_add_package_post,
    api_set_package_data_post,
};
use openapi::models::{
    ApiAddFilesPostRequest, ApiAddPackagePostRequest, ApiSetPackageDataPostRequest, Destination,
};

fn get_pyload_config() -> Configuration {
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");

    let mut config = Configuration::new();
    config.base_path = api_url;
    config.api_key = Some(ApiKey {
        key: api_key,
        prefix: None,
    });
    config
}

pub async fn fetch_packages() -> Result<Vec<openapi::models::PackageData>, String> {
    let mut queue = py_load_rest_api::api_get_queue_get(&get_pyload_config())
        .await
        .map_err(|e| e.to_string())?;

    let collector = py_load_rest_api::api_get_collector_get(&get_pyload_config())
        .await
        .map_err(|e| e.to_string())?;

    queue.extend(collector);

    Ok(queue)
}

pub async fn fetch_files(package_id: i32) -> Result<Vec<openapi::models::FileData>, String> {
    let pkg = py_load_rest_api::api_get_package_data_get(&get_pyload_config(), Some(package_id))
        .await
        .map_err(|e| e.to_string())?;

    pkg.links
        .flatten()
        .ok_or_else(|| "No files found".to_string())
}

pub async fn add_package(
    name: String,
    links: Vec<String>,
    password: Option<String>,
    dest: Destination,
) -> Result<i32, Error<ApiAddPackagePostError>> {
    let mut pkg = ApiAddPackagePostRequest::new(name, links);
    pkg.dest = Some(dest);
    let pid = api_add_package_post(&get_pyload_config(), Some(pkg)).await?;

    if let Some(pw) = password && !pw.is_empty() {
        let data = HashMap::from([("password".to_string(), serde_json::Value::from(pw))]);
        let req = ApiSetPackageDataPostRequest::new(pid, data);
        let _ = api_set_package_data_post(&get_pyload_config(), Some(req)).await;
    }

    Ok(pid)
}

pub async fn add_links_to_package(
    package_id: i32,
    links: Vec<String>,
) -> Result<(), Error<ApiAddFilesPostError>> {
    let req = ApiAddFilesPostRequest::new(package_id, links);
    api_add_files_post(&get_pyload_config(), Some(req)).await
}
