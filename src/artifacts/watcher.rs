use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};

pub struct ArtifactWatcher {
    _watcher: RecommendedWatcher,
    receiver: Receiver<PathBuf>,
}

impl ArtifactWatcher {
    pub fn new() -> anyhow::Result<Self> {
        let (tx, receiver) = channel();

        let mut path = dirs::data_dir().ok_or_else(|| anyhow::anyhow!("No data dir found"))?;
        path.push("uncver-artifacts");
        path.push("artifacts");

        let _watcher = RecommendedWatcher::new(
            move |res: notify::Result<Event>| {
                if let Ok(event) = res {
                    for path in event.paths {
                        if path.extension().map_or(false, |ext| ext == "json") {
                            let _ = tx.send(path);
                        }
                    }
                }
            },
            notify::Config::default(),
        )?;

        Ok(Self { _watcher, receiver })
    }

    pub fn watch(&mut self) -> anyhow::Result<()> {
        let mut path = dirs::data_dir().ok_or_else(|| anyhow::anyhow!("No data dir found"))?;
        path.push("uncver-artifacts");
        path.push("artifacts");

        self._watcher.watch(&path, RecursiveMode::Recursive)?;
        Ok(())
    }

    pub fn recv(&self) -> Result<PathBuf, std::sync::mpsc::RecvError> {
        self.receiver.recv()
    }

    pub fn try_recv(&self) -> Result<PathBuf, std::sync::mpsc::TryRecvError> {
        self.receiver.try_recv()
    }
}
