mod server;

fn main() {
    println!("Root file existence: {:?}", server::exists("/spring/index.html"));
    server::start();
}