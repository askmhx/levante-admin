mod app {
    use std::fs::File;
    use std::fmt;
    use std::net::IpAddr;
    use std::error::Error;
    use serde_json;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DbConfig {
        pub host: IpAddr,
        pub port: u16,
        pub user: String,
        pub password: String,
        pub schema: String,
    }

    impl fmt::Display for DbConfig {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let connect = format!("mysql://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.schema);
            write!(f, "{}", connect)
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct LogConfig {
        pub file: String,
        pub rotate_count: u16,
        pub rotate_type: String,
        pub rotate_value: String,
        pub level: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ServerConfig {
        pub port: u16,
        pub addr: String,
        pub charset: String,
    }

    impl fmt::Display for ServerConfig {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let connect = format!("{}:{}", self.addr, self.port);
            write!(f, "{}", connect)
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct StaticConfig {
        pub path: String,
        pub uri: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct HtmlConfig {
        pub path: String,
        pub uri: String,
    }


    #[derive(Serialize, Deserialize, Debug)]
    pub struct TemplateConfig {
        pub layout: String,
        pub path: String,
        pub ext: String,
        pub reload: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ViewConfig{
        pub statics: StaticConfig,
        pub htmls: HtmlConfig,
        pub templates: TemplateConfig,
    }


    #[derive(Serialize, Deserialize, Debug)]
    pub struct AppConfig {
        pub home: String,
        pub server: ServerConfig,
        pub database: DbConfig,
        pub log: LogConfig,
        pub view: ViewConfig,
    }

    impl AppConfig {
        pub fn from_file(filepath: String) -> Result<AppConfig, Box<Error>> {
            let file = File::open(filepath)?;
            Ok(serde_json::from_reader(file)?)
        }
    }
}