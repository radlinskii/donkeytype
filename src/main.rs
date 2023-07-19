mod config;

fn main() {
    let config = config::Config::new();

    println!("config = {:?}", config);
}
