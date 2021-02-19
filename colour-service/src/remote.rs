use crate::{AppConfig, PodInfo};
use actix_web::client::Client;

pub async fn get_remote(app_config: &AppConfig) -> Result<PodInfo, actix_web::Error> {
    let client = Client::default();

    // Create request builder and send request
    let pod_info = client
        .get(&app_config.remote_service)
        .header("User-Agent", "actix-web/3.0")
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<PodInfo>()
        .await?;

    Ok(pod_info)
}
