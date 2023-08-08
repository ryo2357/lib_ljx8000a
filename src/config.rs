use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Ljx8000aConfig {
    pub save_dir: String,
    // 配列は使えないので処理する必用がある
    pub host: [u8; 4],
    pub port: u16,
    pub high_speed_port: u16,

    pub profile_data_num: usize,
    pub fetch_brightness_data: bool,
}
impl Ljx8000aConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let builder = envy::prefixed("Ljx8000aConfig_").from_env::<Ljx8000aConfigBuilder>()?;
        let config = builder.build().unwrap();
        Ok(config)
    }
}

#[derive(Deserialize, Serialize)]
struct Ljx8000aConfigBuilder {
    pub save_dir: String,
    // 配列は使えないので処理する必用がある
    pub host: String,
    pub port: u16,
    pub high_speed_port: u16,

    pub profile_data_num: usize,
    pub fetch_brightness_data: bool,
}

impl Ljx8000aConfigBuilder {
    fn build(self) -> anyhow::Result<Ljx8000aConfig> {
        let host = convert_host(self.host)?;

        Ok(Ljx8000aConfig {
            save_dir: self.save_dir,
            host,
            port: self.port,
            high_speed_port: self.high_speed_port,
            profile_data_num: self.profile_data_num,
            fetch_brightness_data: self.fetch_brightness_data,
        })
    }
}

fn convert_host(host: String) -> anyhow::Result<[u8; 4]> {
    let input: Vec<&str> = host.split_terminator(',').collect();
    let mut output = [0u8; 4];
    output[0] = input[0].parse()?;
    output[1] = input[1].parse()?;
    output[2] = input[2].parse()?;
    output[3] = input[3].parse()?;
    Ok(output)
}
