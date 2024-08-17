mod server;

fn main() {
    server::start();
    println!("existence: {:?}", server::exists("index.html"));
}