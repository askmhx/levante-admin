use actix_web::App;
use actix_web::server;
use app::config::AppConfig;
use actix::SyncArbiter;
use actix::SystemRunner;
use database::manager;
use actix::Addr;
use database::manager::DbExecutor;
use app::route::init_route;

pub struct Bootstrap {
    app: App<State>,
    config: AppConfig,
    db_addr: Addr<DbExecutor>,
    system_runer: SystemRunner,
}

pub struct State {
    db: Addr<manager::DbExecutor>,
}


impl  Bootstrap {

    pub fn new(filepath: String) ->  Bootstrap  {
        let config = AppConfig::from_file(filepath).unwrap();
        let pool = manager::init_pool(config.database.to_string());
        let addr = SyncArbiter::start(3, move || manager::DbExecutor(pool.clone()));
        let app = App::with_state(State { db: addr.clone() });
        let runer = actix::System::new("levante-admin");
        let bootstrap = Bootstrap {
            app: app,
            config: config,
            db_addr: addr,
            system_runer: runer,
        };
        init_route(&bootstrap.app);
        return bootstrap
    }

    pub fn launch(&self) -> i32 {
        server::new(move || { self.app }).bind(self.config.server.to_string()).unwrap().run();
        self.system_runer.run()
    }
}
