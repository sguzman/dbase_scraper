extern crate reqwest;
extern crate scraper;

mod http;

fn main() {
    loop {
        let n: Option<u64> = http::page_count();
        if n.is_none() {
            continue
        }

        let m: u64 = n.unwrap() + 1;

        for i in 1..m {
            let url: String = if i < 2 {
                format!("https://dbase.tube/chart/channels/subscribers/all")
            } else {
                format!("https://dbase.tube/chart/channels/subscribers/all?page={}", i)
            };

            let url: &str = url.as_ref();
            let body_result: Result<reqwest::Response, reqwest::Error> = reqwest::get(url);
            let mut body: reqwest::Response = match body_result {
                Ok(b) => b,
                _ => continue
            };

            let text: String = match body.text() {
                Ok(t) => t,
                _ => continue
            };

            let document: &str = text.as_ref();
            let doc: scraper::Html = scraper::Html::parse_document(document);

            let selectors: &str = "div.dlist_desk a[href^=\"/c/UC\"]";
            let selector: scraper::Selector = scraper::Selector::parse(selectors).unwrap();

            for element in doc.select(&selector) {
                println!("{}", element.value().attr("href").unwrap());
            }
        }
    }
}
