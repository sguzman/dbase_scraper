extern crate postgres;
extern crate rand;

use util;

fn conn_str() -> String {
    let db_host: String = util::env("DB_HOST", "localhost");
    let db_port: String = util::env("DB_PORT", "30000");

    return format!("postgresql://postgres:@{}:{}/youtube", db_host, db_port);
}

fn conn() -> postgres::Connection {
    let tls = postgres::TlsMode::None;
    let params: String = conn_str();

    return postgres::Connection::connect(params, tls).unwrap();
}