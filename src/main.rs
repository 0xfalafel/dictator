use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState, hotkey::{HotKey, Modifiers, Code}};
use std::thread;

use tokio_util::sync::CancellationToken;
use tokio::time::Duration;

mod record_sound;
use record_sound::record_wav;

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
                    if let Err(err) = record_wav(token.clone()).await {
                        eprintln!("Failed to record the audio: {err}");
                        break;
                    }                    
                },
                HotKeyState::Released => {
                    token.cancel();
                    println!("Stopped recording");
                    token = CancellationToken::new();
                }
            }
        }


        thread::sleep(Duration::from_millis(100));
    }
}
