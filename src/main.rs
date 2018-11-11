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
            let vec: Vec<String> = http::channels(i);
            for c in vec {
                println!("{}", c);
            }
        }
    }
}
