extern crate reqwest;
extern crate scraper;

fn main() {
    loop {
        let url: &str = "https://dbase.tube/chart/channels/subscribers/all";

        let body_result: Result<reqwest::Response, reqwest::Error> = reqwest::get(url);
        let mut body: reqwest::Response = match body_result {
            Ok(b) => b,
            _ => continue
        };

        println!("{:?}", body.text());
    }
}
