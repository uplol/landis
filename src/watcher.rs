use std::{path::PathBuf, time::Duration};

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

use crate::{palette::BlockPalette, render};

enum RegionFileEvent {
    Write { path: PathBuf },
}

/// Runs a watcher event loop.
/// It is responsible for loading the palette, starting the watcher, and firing off renders.
pub async fn run_watcher() {
    // load palette
    let palette = BlockPalette::load("./palette.tar.gz".into()).await;
    println!("loaded palette");

    // start watcher and really simple render trigger
    let mut rx = spawn_watcher_task();
    println!("spawned watcher");
    loop {
        match rx.recv().await {
            Some(Some(event)) => match event {
                RegionFileEvent::Write { path } => {
                    let palette_clone = palette.clone();
                    // todo: track renders, and maybe stop shotgunning them out.. :)
                    println!("rendering region");
                    tokio::spawn(async move {
                        // tokio::task::spawn_blocking(move || {
                        render::render_map(palette_clone, path).await
                        // })
                        // .await
                        //.ok();
                    });
                }
            },
            _ => {
                panic!("watcher died");
            }
        }
    }
}

/// Runs watcher_task and returns a receiver
fn spawn_watcher_task() -> UnboundedReceiver<Option<RegionFileEvent>> {
    let (tx, rx) = unbounded_channel();
    tokio::spawn(async move {
        let tx_err = tx.clone();
        tokio::task::spawn_blocking(move || watcher_task(tx))
            .await
            .ok();
        tx_err.send(None).ok();
    });
    rx
}

/// Watches the region files for changes and send notifications upstream.
/// This uses std sync primitives so it's probably best spawned with `tokio::task::spawn_blocking`.
fn watcher_task(tx: tokio::sync::mpsc::UnboundedSender<Option<RegionFileEvent>>) {
    let (watcher_tx, watcher_rx) = std::sync::mpsc::channel();
    let mut watcher = watcher(watcher_tx, Duration::from_secs(10)).unwrap();

    watcher
        .watch("./mcserver/world/region", RecursiveMode::Recursive)
        .unwrap();

    // loop over fs notify events and bubble up events of interest
    loop {
        match watcher_rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::NoticeWrite(path) | DebouncedEvent::Write(path) => {
                    let ext = path.extension();
                    if ext.is_some() && ext.unwrap() == "mca" {
                        println!("file modified: {:?}", path);
                        tx.send(Some(RegionFileEvent::Write { path })).ok();
                    }
                }
                _ => {}
            },
            Err(_e) => {
                tx.send(None).ok();
            }
        }
    }
}
