use actix_web::{App, HttpServer, middleware, Body};
use crate::database::manager;
use crate::app::route::init_route;
use crate::app::config::AppConfig;

pub struct Bootstrap {
    app: App<AppEntry, Body>,
    config: AppConfig,
}

impl Bootstrap {
    pub fn new(filepath: String) -> Bootstrap {
        let config = AppConfig::from_file(filepath).unwrap();
        let pool = manager::init_pool(config.database.to_string());
        let app = App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default());
        let bootstrap = Bootstrap {
            app: app,
            config: config,
        };
        init_route(&bootstrap.app);
        return bootstrap;
    }

    pub fn launch(&self) -> std::io::Result<()> {
        HttpServer::new(move || { self.app }).bind(self.config.server.to_string())?.run()
    }
}
