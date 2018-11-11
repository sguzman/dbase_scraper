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

    return Some(real_num);
}

pub fn channels(i: u64) -> Vec<String> {
    let url: String = if i < 2 {
        format!("https://dbase.tube/chart/channels/subscribers/all")
    } else {
        format!("https://dbase.tube/chart/channels/subscribers/all?page={}", i)
    };

    let url: &str = url.as_ref();
    let body_result: Result<reqwest::Response, reqwest::Error> = reqwest::get(url);
    let mut body: reqwest::Response = match body_result {
        Ok(b) => b,
        _ => return Vec::new()
    };

    let text: String = match body.text() {
        Ok(t) => t,
        _ => return Vec::new()
    };

    let document: &str = text.as_ref();
    let doc: scraper::Html = scraper::Html::parse_document(document);

    let selectors: &str = "div.dlist_desk a[href^=\"/c/UC\"]";
    let selector: scraper::Selector = scraper::Selector::parse(selectors).unwrap();

    let mut vec: Vec<String> = Vec::new();
    for element in doc.select(&selector) {
        let value: &str = element.value().attr("href").unwrap();
        let value: String = value.to_string();
        let value: Vec<&str> = value.split("/").collect::<Vec<&str>>();
        let value: String = value[2].to_string();

        vec.push(value);
    }

    return vec;
}