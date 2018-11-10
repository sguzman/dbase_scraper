extern crate reqwest;
extern crate scraper;

mod http;

fn main() {
    loop {
        let n: u64 = http::page_count();
        if n == 0 {
            continue
        }

        println!("{}", n);
    }
}
