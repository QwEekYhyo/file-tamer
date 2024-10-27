mod event_watcher;
mod event_handler;
mod file_utils;

fn main() {
    let watched_path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    let destination_path = std::env::args()
        .nth(2)
        .expect("Argument 2 needs to be a path");
    println!("Watching {}", watched_path);
    println!("Organizing to {}", destination_path);

    futures::executor::block_on(async {
        if let Err(e) = event_watcher::async_watch(watched_path, destination_path).await {
            println!("error: {:?}", e)
        }
    });
}
