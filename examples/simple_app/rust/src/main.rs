use celestia_core::{send_command, App, AppConfig, Command, Event};
use std::{thread, time::Duration};

fn main() {
    let app = App::new(AppConfig { name: "simple".into(), version: "0.1.0".into() });

    thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));
        let _ = send_command(Command::Ping);
        thread::sleep(Duration::from_millis(100));
        let _ = send_command(Command::Echo("hello from example".into()));
        thread::sleep(Duration::from_millis(100));
        let _ = send_command(Command::Shutdown);
    });

    app.run(|_handle, event| match event {
        Event::Ready => println!("Ready."),
        Event::Command(Command::Ping) => println!("Ping!"),
        Event::Command(Command::Echo(s)) => println!("Echo: {}", s),
        Event::Command(Command::Shutdown) => println!("Shutdown."),
        Event::Exit => println!("Bye."),
    });
}
