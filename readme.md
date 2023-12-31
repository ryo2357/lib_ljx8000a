# ljx8000a

ljx8000a をコントロールするクライアント

## usage

### 設定値

.env ファイルに記入

```Dotenv
# ログファイルの出力先
Ljx8000aConfig_save_dir=./output
# Ljx8000aのhost
Ljx8000aConfig_host=192,168,0,1
# Ljx8000aのport
Ljx8000aConfig_port=24691
# Ljx8000aのhigh-speed-communication用のport
Ljx8000aConfig_high_speed_port=24692
# 1プロファイル当たりの測定点数
Ljx8000aConfig_profile_data_num=3200
# 光量データを含むか
Ljx8000aConfig_fetch_brightness_data=true
```

### ライブラリ

コンパイル時に./vendor/LJX8_IF.lib が必用

実行時./LJX8_IF.dll が必用

### コード

```rust
use dotenv::dotenv;

use ljx8000a::Ljx8000aConfig;
use ljx8000a::LjxClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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


```
