use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use dirs;
use kdl::KdlDocument;
use openapi::apis::Error;
use openapi::apis::configuration::{ApiKey, Configuration};
use openapi::apis::py_load_rest_api::{
    self, ApiAddFilesPostError, ApiAddPackagePostError, api_add_files_post, api_add_package_post,
    api_set_package_data_post, api_delete_files_post, api_delete_packages_post,
    ApiDeleteFilesPostError, ApiDeletePackagesPostError,
};
use openapi::models::{
    ApiAddFilesPostRequest, ApiAddPackagePostRequest, ApiSetPackageDataPostRequest, Destination,
    ApiDeleteFilesPostRequest, ApiDeletePackagesPostRequest,
};

pub fn get_config_path() -> PathBuf {
    let pkg_name = env!("CARGO_PKG_NAME");
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push(pkg_name);
    config_path.push("config.kdl");
    config_path
}
pub fn create_app_config() -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_path();

    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }

    let config = r#"
pyload-url "http://localhost:8000/"
// api-key YOUR_API_KEY_HERE
"#;

    fs::write(path, config.trim())?;

    Ok(())
}
pub fn ensure_app_config_exists() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(exists) = fs::exists(get_config_path())
        && !exists
    {
        println!("NOT EXISTS");
        create_app_config()?;
    }
    Ok(())
}
pub fn get_pyload_config() -> &'static Configuration {
    static CONFIG: OnceLock<Configuration> = OnceLock::new();
    CONFIG.get_or_init(|| {
        let app_config_path = get_config_path();
        let config = fs::read_to_string(&app_config_path).unwrap();
        let doc: KdlDocument = config.parse().unwrap();

        let api_url: String = doc
            .get("pyload-url")
            .and_then(|n| n.entries().first())
            .and_then(|e| e.value().as_string())
            .unwrap_or("http://localhost:8000/")
            .trim_end_matches('/')
            .to_string();

        let api_key: String = doc
            .get("api-key")
            .and_then(|n| n.entries().first())
            .and_then(|e| e.value().as_string())
            .expect(&format!(
                "api-key is required in config: {}",
                &app_config_path.display()
            ))
            .to_string();

        let mut config = Configuration::new();
        config.base_path = api_url;
        config.api_key = Some(ApiKey {
            key: api_key,
            prefix: None,
        });
        config
    })
}

pub async fn fetch_packages() -> Result<Vec<openapi::models::PackageData>, String> {
    let mut queue = py_load_rest_api::api_get_queue_get(get_pyload_config())
        .await
        .map_err(|e| e.to_string())?;

    let collector = py_load_rest_api::api_get_collector_get(get_pyload_config())
        .await
        .map_err(|e| e.to_string())?;

    queue.extend(collector);

    Ok(queue)
}

pub async fn fetch_files(package_id: i32) -> Result<Vec<openapi::models::FileData>, String> {
    let pkg = py_load_rest_api::api_get_package_data_get(get_pyload_config(), Some(package_id))
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
    let pid = api_add_package_post(get_pyload_config(), Some(pkg)).await?;

    if let Some(pw) = password
        && !pw.is_empty()
    {
        let data = HashMap::from([("password".to_string(), serde_json::Value::from(pw))]);
        let req = ApiSetPackageDataPostRequest::new(pid, data);
        let _ = api_set_package_data_post(get_pyload_config(), Some(req)).await;
    }

    Ok(pid)
}

pub async fn add_links_to_package(
    package_id: i32,
    links: Vec<String>,
) -> Result<(), Error<ApiAddFilesPostError>> {
    let req = ApiAddFilesPostRequest::new(package_id, links);
    api_add_files_post(get_pyload_config(), Some(req)).await
}

pub async fn remove_packages(package_ids: Vec<i32>) -> Result<(), Error<ApiDeletePackagesPostError>>{
    let req = ApiDeletePackagesPostRequest::new(package_ids);
    api_delete_packages_post(get_pyload_config(), Some(req)).await
}

pub async fn remove_files_from_package(file_ids: Vec<i32>) -> Result<(), Error<ApiDeleteFilesPostError>>{
    let req = ApiDeleteFilesPostRequest::new(file_ids);
    api_delete_files_post(get_pyload_config(), Some(req)).await
}
