// 配置模块
// 负责应用程序配置管理

pub struct Config {
    pub app_name: String,
    pub version: String,
    pub debug: bool,
}

impl Config {
    pub fn load() -> Self {
        Config {
            app_name: "Modular App".to_string(),
            version: "1.0.0".to_string(),
            debug: true,
        }
    }
    
    pub fn is_debug(&self) -> bool {
        self.debug
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::load()
    }
}
