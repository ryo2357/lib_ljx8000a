use dotenv::dotenv;

use mylogger::info;

use ljx8000a::Ljx8000aConfig;

fn main() -> anyhow::Result<()> {
    mylogger::init();
    // parse_verify()?;
    config_verify()?;
    Ok(())
}

#[allow(dead_code)]
fn config_verify() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Ljx8000aConfig::from_env().unwrap();

    info!("{:?}", config);
    Ok(())
}

#[allow(dead_code)]
fn parse_verify() -> anyhow::Result<()> {
    let input = String::from("192,168,0,1");
    info!("input:{:?}", input);
    let input: Vec<&str> = input.split_terminator(',').collect();
    let mut output = [0u8; 4];
    output[0] = input[0].parse()?;
    output[1] = input[1].parse()?;
    output[2] = input[2].parse()?;
    output[3] = input[3].parse()?;

    info!("output:{:?}", output);

    Ok(())
}
