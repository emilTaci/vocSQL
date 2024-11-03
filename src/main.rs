mod db;
#[tokio::main]
async fn main() {
    if let Err(e) = db::connect_and_setup().await {
        println!("Error: {}", e);
    }
}
