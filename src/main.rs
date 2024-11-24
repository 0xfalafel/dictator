use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState, hotkey::{HotKey, Modifiers, Code}};
use std::thread;

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio::task::spawn;
use tokio::select;


async fn demo(mut rx: mpsc::Receiver<()>) {
    let mut count: u64 = 0;
    
    loop {
        if rx.recv().await.is_some() {
            return;
        }

        println!("{}", count);
        count += 1;
    }
}


#[tokio::main]
async fn main() {
    // initialize the hotkeys manager
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::SUPER), Code::KeyM);
    manager.register(hotkey).err().map(|e| {
        eprintln!("Could not register the global hotkey Super+M: {}", e);
        std::process::exit(1);
    });


    // channels used to stop the counter
    let (tx, rx) = mpsc::channel(1);

    loop {
        
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            // println!("{:?}", event.state);

            match event.state {
                HotKeyState::Pressed  => {
                    tokio::spawn(demo(rx));
                },
                HotKeyState::Released => println!("Bye mom!"),
            }
        }


        thread::sleep(Duration::from_millis(100));
    }
}
