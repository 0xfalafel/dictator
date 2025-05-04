use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState, hotkey::{HotKey, Modifiers, Code}};
use std::thread;

use tokio_util::sync::CancellationToken;
use tokio::time::Duration;


async fn demo(token: CancellationToken) {
    let mut count: u64 = 1;
    
    loop {
        if token.is_cancelled() {
            return ;
        }

        thread::sleep(Duration::from_millis(1000));

        println!("{}s", count);
        count += 1;
    }
}


#[tokio::main]
async fn main() {
    // initialize the hotkeys manager
    let manager = GlobalHotKeyManager::new().expect("Failed to initalize the global hotkey manager");
    let hotkey = HotKey::new(Some(Modifiers::SUPER), Code::KeyM);
    manager.register(hotkey).err().map(|e| {
        eprintln!("Could not register the global hotkey Super+M: {}", e);
        std::process::exit(1);
    });

    let mut token = CancellationToken::new();
    
    loop {
        
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            // println!("{:?}", event.state);

            match event.state {
                HotKeyState::Pressed  => {
                    let _ = tokio::spawn(
                        demo(token.clone())
                    );
                },
                HotKeyState::Released => {
                    token.cancel();
                    println!("Bye mom!");
                    token = CancellationToken::new();
                }
            }
        }


        thread::sleep(Duration::from_millis(100));
    }
}
