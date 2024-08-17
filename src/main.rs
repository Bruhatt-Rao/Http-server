mod server;

fn main() {
    println!("Root file existence: {:?}", server::exists("index.html"));
    server::start();
}