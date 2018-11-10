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

        let text: String = match body.text() {
            Ok(t) => t,
            _ => continue
        };

        let document: &str = text.as_str();
        let doc: scraper::Html = scraper::Html::parse_document(document);

        let selector: scraper::Selector = scraper::Selector::parse("li.ffwd a[href]").unwrap();
        let mut elements: scraper::html::Select = doc.select(&selector);
        println!("{:?}", elements.next().unwrap().value().attr("href").unwrap());
    }
}
