mod event_watcher;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    println!("watching {}", path);

    futures::executor::block_on(async {
        if let Err(e) = event_watcher::async_watch(path).await {
            println!("error: {:?}", e)
        }
    });
}
