mod database{
    use actix::prelude::*;
    use diesel::mysql::MysqlConnection;
    use diesel::r2d2;
    use diesel::r2d2::{ConnectionManager, Pool};


    pub fn init_pool(db_url: String) -> Pool<ConnectionManager<MysqlConnection>> {
        let manager = r2d2::ConnectionManager::<MysqlConnection>::new(db_url);
        r2d2::Pool::new(manager).expect("db pool")
    }

    pub struct DbExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

    impl Actor for DbExecutor {
        type Context = SyncContext<Self>;
    }
}