mod api;
mod palette;
mod render;
mod watcher;

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        watcher::run_watcher().await;
    });

    println!("starting api");
    api::run_api().await;
}
