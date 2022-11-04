
{% if useconfig -%}
use pdk::api::classy::bootstrap::Launcher;
use pdk::api::classy::event::Exchange;
use anyhow::{anyhow, Result};
use pdk::api::logger;
use serde::Deserialize;
use pdk::api::classy::event::HeadersAccessor;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(alias = "header_name")]
    pub header_name: String
}

// This filters shows how to log an specific request header.
// It uses the `header_name` from the policy configuration
async fn filter(exchange: Exchange, config: &Config) -> Result<()> {
    // Waits for request headers event to happen
    let exchange = exchange.wait_for_request_headers().await;
    //Once headers were received ask for them
    if let Some(event) = exchange.event_data() {
        //Obtain the header name from the config
        let header = &config.header_name;
        // Log the header value
        logger::info!("header value: {}", event.header(header.as_str()).unwrap_or_default());
    }
    Ok(())
}

#[pdk::api::entrypoint]
async fn configure(launcher: Launcher) -> Result<()> {
    let config_bytes = launcher
        .configuration()
        .ok_or_else(|| anyhow!("Missing configuration"))?;
    let config = serde_json::from_slice(&config_bytes)?;
    launcher.launch(|e| filter(e, &config)).await?;
    Ok(())
}
{% else -%}
use pdk::api::classy::bootstrap::Launcher;
use pdk::api::classy::event::Exchange;
use anyhow::{anyhow, Result};
use pdk::api::logger;
use pdk::api::classy::event::HeadersAccessor;

// This filters shows how to log an specific request header.
// It uses the `header_name` from the policy configuration
#[pdk::api::entrypoint]
async fn filter(exchange: Exchange) -> Result<()> {
    // Waits for request headers event to happen
    let exchange = exchange.wait_for_request_headers().await;
    //Once headers were received ask for them
    if let Some(event) = exchange.event_data() {
        //Obtain the header name from the config
        let header = "Token";
        // Log the header value
        logger::info!("header value: {}", event.header(header.as_str()).unwrap_or_default());
    }
    Ok(())
}

{% endif -%}

