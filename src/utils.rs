use openapi::apis::configuration::{ApiKey, Configuration};
use openapi::apis::py_load_rest_api;

fn get_config() -> Configuration {
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
    let mut queue = py_load_rest_api::api_get_queue_get(&get_config())
        .await
        .map_err(|e| e.to_string())?;

    let collector = py_load_rest_api::api_get_collector_get(&get_config())
        .await
        .map_err(|e| e.to_string())?;

    queue.extend(collector);

    Ok(queue)
}

pub async fn fetch_files(package_id: i32) -> Result<Vec<openapi::models::FileData>, String> {
    let pkg = py_load_rest_api::api_get_package_data_get(&get_config(), Some(package_id))
        .await
        .map_err(|e| e.to_string())?;

    pkg.links.flatten().ok_or_else(|| "No files found".to_string())
}
