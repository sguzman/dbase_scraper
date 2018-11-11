pub fn page_count() -> Option<u64> {
    let url: &str = "https://dbase.tube/chart/channels/subscribers/all";

    let body_result: Result<reqwest::Response, reqwest::Error> = reqwest::get(url);
    let mut body: reqwest::Response = match body_result {
        Ok(b) => b,
        _ => return None
    };

    let text: String = match body.text() {
        Ok(t) => t,
        _ => return None
    };

    let document: &str = text.as_str();
    let doc: scraper::Html = scraper::Html::parse_document(document);

    let selectors: &str = "li.ffwd a[href]";
    let selector: scraper::Selector = scraper::Selector::parse(selectors).unwrap();
    let mut elements: scraper::html::Select = doc.select(&selector);
    let href: &str = elements.next().unwrap().value().attr("href").unwrap();

    let pat: &str = "=";
    let begin: usize = href.rfind(pat).unwrap();
    let n: usize = begin + 1;
    let num: String = href.chars().skip(n).collect();
    let real_num: u64 = num.parse::<u64>().unwrap();

    return real_num;
}