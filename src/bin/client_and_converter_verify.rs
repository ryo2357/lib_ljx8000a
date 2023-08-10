use dotenv::dotenv;

use mylogger::info;

use ljx8000a::Ljx8000aConfig;
use ljx8000a::LjxClient;

use ljx8000a::LjxDataConverter;
use ljx8000a::LjxDataConverterConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    mylogger::init();

    client_verify().await?;

    Ok(())
}

#[allow(dead_code)]
async fn client_verify() -> anyhow::Result<()> {
    dotenv().ok();

    let ljx_config = Ljx8000aConfig::from_env().unwrap();
    let mut converter_config = LjxDataConverterConfig::from_env().unwrap();

    let mut client = LjxClient::create(ljx_config).await?;
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

    let row_filepath = client.get_last_filepath()?;
    converter_config.set_ljx_data_path(row_filepath);

    let converter = LjxDataConverter::create(converter_config)?;
    converter.execute()?;

    Ok(())
}
