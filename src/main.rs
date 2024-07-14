use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let url = env::var("BACKEND_URL").unwrap();
    
    println!("{}", url);
}
