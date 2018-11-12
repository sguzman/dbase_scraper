extern crate postgres;
extern crate rand;
extern crate reqwest;
extern crate scraper;

mod http;
mod sql;
mod util;

fn main() {
    println!("start");
    loop {
        let n: Option<u64> = http::page_count();
        if n.is_none() {
            continue
        }

        let m: u64 = n.unwrap() + 1;

        for i in 1..m {
            let vec: Vec<String> = http::channels(i);
            sql::insert(vec);
        }
    }
}
