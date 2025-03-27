use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

pub struct ConfigLoader;

#[derive(Debug, Deserialize)]
pub struct BrokerConfig {
    pub id: u32,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    pub log_dir: String,
    pub segment_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct ConfigStruct {
    pub broker: BrokerConfig,
    pub storage: StorageConfig,
}

impl ConfigStruct {
    pub fn new() -> Result<Self, ConfigError> {
        let builder = Config::builder()
            // 1. 读取默认配置文件 `config/default.toml`
            .add_source(File::with_name("cfg").required(false))
            // 2. 读取环境变量（使用 `BROKER_HOST` 这样的格式）
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()?;

        builder.try_deserialize()
    }
}
