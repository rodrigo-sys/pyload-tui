use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use dirs;
use kdl::KdlDocument;
use openapi::apis::Error;
use openapi::apis::configuration::{ApiKey, Configuration};
use openapi::apis::py_load_rest_api::{
    self, ApiAddFilesPostError, ApiAddPackagePostError, ApiDeleteFilesPostError, ApiDeletePackagesPostError, ApiGetFileDataGetError, ApiGetPackageDataGetError, ApiMovePackagePostError, ApiPauseServerPostError, ApiRestartFailedPostError, ApiRestartFilePostError, ApiRestartPackagePostError, ApiStatusServerGetError, ApiStopAllDownloadsPostError, ApiStopDownloadsPostError, ApiTogglePausePostError, ApiUnpauseServerPostError, api_add_files_post, api_add_package_post, api_delete_files_post, api_delete_packages_post, api_get_file_data_get, api_get_package_data_get, api_get_package_order_get, api_move_files_post, api_move_package_post, api_order_file_post, api_order_package_post, api_pause_server_post,     api_restart_file_post, api_restart_package_post, api_restart_failed_post, api_set_package_data_post, api_status_downloads_get, api_status_server_get, api_stop_all_downloads_post, api_stop_downloads_post, api_toggle_pause_post, api_unpause_server_post,
};
use openapi::models::{
    ApiAddFilesPostRequest, ApiAddPackagePostRequest, ApiDeleteFilesPostRequest, ApiDeletePackagesPostRequest,
    ApiSetPackageDataPostRequest, ApiStopDownloadsPostRequest, Destination, DownloadInfo, FileData, PackageData, ServerStatus,
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
            .expect(&format!("api-key is required in config: {}", &app_config_path.display()))
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
    let mut queue = py_load_rest_api::api_get_queue_get(get_pyload_config()).await.map_err(|e| e.to_string())?;

    let collector = py_load_rest_api::api_get_collector_get(get_pyload_config()).await.map_err(|e| e.to_string())?;

    queue.extend(collector);

    Ok(queue)
}

pub async fn fetch_package_data(package_id: i32) -> Result<PackageData, Error<ApiGetPackageDataGetError>> {
    api_get_package_data_get(get_pyload_config(), Some(package_id)).await
}

pub async fn fetch_file_data(file_id: i32) -> Result<FileData, Error<ApiGetFileDataGetError>> {
    api_get_file_data_get(get_pyload_config(), Some(file_id)).await
}

pub async fn fetch_files(package_id: i32) -> Result<Vec<openapi::models::FileData>, String> {
    let pkg = fetch_package_data(package_id).await.map_err(|e| e.to_string())?;

    pkg.links.flatten().ok_or_else(|| "No files found".to_string())
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

pub async fn add_links_to_package(package_id: i32, links: Vec<String>) -> Result<(), Error<ApiAddFilesPostError>> {
    let req = ApiAddFilesPostRequest::new(package_id, links);
    api_add_files_post(get_pyload_config(), Some(req)).await
}

pub async fn remove_packages(package_ids: Vec<i32>) -> Result<(), Error<ApiDeletePackagesPostError>> {
    let req = ApiDeletePackagesPostRequest::new(package_ids);
    api_delete_packages_post(get_pyload_config(), Some(req)).await
}

pub async fn remove_files_from_package(file_ids: Vec<i32>) -> Result<(), Error<ApiDeleteFilesPostError>> {
    let req = ApiDeleteFilesPostRequest::new(file_ids);
    api_delete_files_post(get_pyload_config(), Some(req)).await
}

pub async fn fetch_downloads_info() -> Vec<DownloadInfo> {
    api_status_downloads_get(get_pyload_config()).await.unwrap_or_default()
}

pub async fn stop_downloads(file_ids: Vec<i32>) -> Result<(), Error<ApiStopDownloadsPostError>> {
    let stop_downloads_request = ApiStopDownloadsPostRequest::new(file_ids);
    api_stop_downloads_post(get_pyload_config(), Some(stop_downloads_request)).await
}
pub async fn restart_file(file_id: i32) -> Result<(), Error<ApiRestartFilePostError>> {
    api_restart_file_post(get_pyload_config(), Some(file_id)).await
}

pub async fn restart_package(package_id: i32) -> Result<(), Error<ApiRestartPackagePostError>> {
    api_restart_package_post(get_pyload_config(), package_id).await
}

pub async fn restart_failed() -> Result<(), Error<ApiRestartFailedPostError>> {
    api_restart_failed_post(get_pyload_config()).await
}

pub async fn stop_all_downloads() -> Result<(), Error<ApiStopAllDownloadsPostError>> {
    api_stop_all_downloads_post(get_pyload_config()).await
}

pub async fn pause_server() -> Result<(), Error<ApiPauseServerPostError>> {
    api_pause_server_post(get_pyload_config()).await
}

pub async fn unpause_server() -> Result<(), Error<ApiUnpauseServerPostError>> {
    api_unpause_server_post(get_pyload_config()).await
}

pub async fn toggle_pause() -> Result<bool, Error<ApiTogglePausePostError>> {
    api_toggle_pause_post(get_pyload_config()).await
}

pub async fn fetch_server_status() -> Result<ServerStatus, Error<ApiStatusServerGetError>> {
    api_status_server_get(get_pyload_config()).await
}

pub async fn reorder_package(
    package_id: i32,
    position: i32,
) -> Result<(), Error<py_load_rest_api::ApiOrderPackagePostError>> {
    api_order_package_post(get_pyload_config(), package_id, position.max(0)).await
}

pub async fn move_package(
    destination: Destination,
    package_id: i32,
) -> Result<(), Error<ApiMovePackagePostError>> {
    api_move_package_post(get_pyload_config(), destination, package_id).await
}
