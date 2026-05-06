use openapi::apis::configuration::{ApiKey, Configuration};
use openapi::apis::py_load_rest_api;

const API_URL: &str = "http://localhost:8000";
const API_KEY: &str = "API_KEY_HERE";

fn get_config() -> Configuration {
    let mut config = Configuration::new();
    config.base_path = API_URL.to_string();
    config.api_key = Some(ApiKey {
        key: API_KEY.to_string(),
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
