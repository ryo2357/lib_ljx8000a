use log::error;
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
        let builder = match envy::prefixed("Ljx8000aConfig_").from_env::<Ljx8000aConfigBuilder>() {
            Ok(builder) => builder,
            Err(err) => {
                error!(".envからLjx8000aConfigの読み込み失敗：{:?}", err);
                anyhow::bail!(".envからLjx8000aConfigの読み込み失敗：{:?}", err)
            }
        };
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

#[derive(Deserialize, Serialize, Debug)]
pub struct LjxDataConverterConfig {
    // .envから取得しないデータはOptionで
    pub ljx_data_path: Option<String>,
    pub output_dir: String,
    pub output_name: String,

    pub convert_quantity: usize,

    pub y_start_num: usize,
    pub y_pitch: f64,
    pub y_take_num: usize,
    pub y_overlap: usize,

    pub x_start_num: usize,
    pub x_pitch: f64,
    pub x_take_num: usize,

    pub z_lower_limit: i32,
    pub z_upper_limit: i32,

    pub have_brightness: bool,
}

impl LjxDataConverterConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let config =
            match envy::prefixed("LjxDataConverterConfig_").from_env::<LjxDataConverterConfig>() {
                Ok(config) => config,
                Err(err) => {
                    error!(".envからLjxDataConverterConfigの読み込み失敗：{:?}", err);
                    anyhow::bail!(".envからLjxDataConverterConfigの読み込み失敗：{:?}", err)
                }
            };
        Ok(config)
    }
    pub fn set_ljx_data_path(&mut self, path: String) {
        self.ljx_data_path = Some(path);
    }
}
