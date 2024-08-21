mod server;

fn main() {
    println!("Root file existence: {:?}", server::exists("/index.html").exists);
    server::start();
}