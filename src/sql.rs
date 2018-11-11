use util;

fn conn_str() -> String {
    let db_host: String = util::env("DB_HOST", "192.168.1.63");
    let db_port: String = util::env("DB_PORT", "30000");

    return format!("postgresql://postgres:@{}:{}/youtube", db_host, db_port);
}

fn conn() -> postgres::Connection {
    let tls = postgres::TlsMode::None;
    let params: String = conn_str();

    return postgres::Connection::connect(params, tls).unwrap();
}

pub fn insert(vec: Vec<String>) {
    let c: postgres::Connection = conn();
    let query: &str = "INSERT INTO youtube.entities.channels (serial) VALUES ($1) ON CONFLICT (serial) DO NOTHING";
    for serial in vec {
        println!("{}", serial);
        let params: &[&dyn postgres::types::ToSql] = &[&serial];
        c.execute(query, params).unwrap();
    }
}