extern crate reqwest;

fn main() {
    loop {
        let url: &str = "https://dbase.tube/chart/channels/subscribers/all";

        let body_result  = reqwest::get(url);
        match body_result {
            Ok(text) => text,
            Err(e) => {
                println!("{:?}" , e);
                continue;
            }
        }
    }
}
