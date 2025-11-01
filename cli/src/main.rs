use celestia_core::{send_command, App, AppConfig, Command, Event};
use std::{thread, time::Duration};

fn main() {
    let app = App::new(AppConfig { name: "celestia-app".into(), version: "0.1.0".into() });

    thread::spawn(|| {
        thread::sleep(Duration::from_millis(200));
        let _ = send_command(Command::Ping);
        thread::sleep(Duration::from_millis(200));
        let _ = send_command(Command::Echo("hello from CLI".into()));
        thread::sleep(Duration::from_millis(200));
        let _ = send_command(Command::Shutdown);
    });

    app.run(|_handle, event| match event {
        Event::Ready => println!("App ready"),
        Event::Command(Command::Ping) => println!("Ping received"),
        Event::Command(Command::Echo(s)) => println!("Echo: {}", s),
        Event::Command(Command::Shutdown) => println!("Shutdown requested"),
        Event::Exit => println!("App exit"),
    });
}
