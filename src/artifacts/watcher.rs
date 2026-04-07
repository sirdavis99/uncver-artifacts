use iced::futures::StreamExt;
use iced::Subscription;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;

pub fn watch_artifacts() -> Subscription<PathBuf> {
    Subscription::run(build_stream)
}

fn build_stream() -> impl iced::futures::Stream<Item = PathBuf> {
    let (tx, rx) = iced::futures::channel::mpsc::channel(100);

    let mut path = dirs::data_dir().expect("No data dir found");
    path.push("uncver-artifacts");
    path.push("artifacts");

    let mut watcher = RecommendedWatcher::new(
        move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                for path in event.paths {
                    if path.extension().map_or(false, |ext| ext == "json") {
                        let mut tx_clone = tx.clone();
                        let _ = tx_clone.try_send(path);
                    }
                }
            }
        },
        notify::Config::default(),
    )
    .expect("Failed to create watcher");

    watcher
        .watch(&path, RecursiveMode::Recursive)
        .expect("Failed to watch directory");

    // Return a stream that keeps the watcher alive
    rx.map(move |p| {
        let _ = &watcher;
        p
    })
}
