use std::sync::{mpsc::{self, Receiver, Sender}, OnceLock};

static GLOBAL_TX: OnceLock<Sender<Command>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub enum Command {
    Ping,
    Echo(String),
    Shutdown,
}

#[derive(Debug, Clone)]
pub enum Event {
    Ready,
    Command(Command),
    Exit,
}

#[derive(Clone)]
pub struct AppHandle {
    tx: Sender<Command>,
}

impl AppHandle {
    pub fn send(&self, cmd: Command) -> bool {
        self.tx.send(cmd).is_ok()
    }
}

pub fn send_command(cmd: Command) -> bool {
    if let Some(tx) = GLOBAL_TX.get() {
        tx.send(cmd).is_ok()
    } else {
        false
    }
}

pub struct App {
    config: AppConfig,
    rx: Receiver<Command>,
    tx: Sender<Command>,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        let (tx, rx) = mpsc::channel();
        Self { config, rx, tx }
    }

    pub fn handle(&self) -> AppHandle {
        AppHandle { tx: self.tx.clone() }
    }

    pub fn run<F>(self, mut handler: F)
    where
        F: FnMut(&AppHandle, Event),
    {
        let _ = GLOBAL_TX.set(self.tx.clone());
        let handle = self.handle();
        handler(&handle, Event::Ready);

        while let Ok(cmd) = self.rx.recv() {
            let is_shutdown = matches!(cmd, Command::Shutdown);
            handler(&handle, Event::Command(cmd));
            if is_shutdown {
                break;
            }
        }

        handler(&handle, Event::Exit);
    }
}

pub mod prelude {
    pub use crate::{send_command, App, AppConfig, Command, Event};
}
