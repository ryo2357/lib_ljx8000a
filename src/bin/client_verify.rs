use dotenv::dotenv;

use mylogger::info;

use ljx8000a::Ljx8000aConfig;
use ljx8000a::LjxClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    mylogger::init();

    client_verify().await?;

    Ok(())
}

#[allow(dead_code)]
async fn client_verify() -> anyhow::Result<()> {
    dotenv().ok();

    let config = Ljx8000aConfig::from_env().unwrap();
    info!("{:?}", config);

    let mut client = LjxClient::create(config).await?;
    info!("make client");

    client.open_ethernet().await?;
    info!("open ethernet");

    client.initialize_communication().await?;
    info!("initialize communication");

    client.pre_start_communication().await?;
    info!("pre_start communication");

    client.start_communication().await?;
    info!("start communication");

    client.stop_communication().await?;
    info!("stop communication");

    Ok(())
}
