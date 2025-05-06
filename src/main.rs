use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState, hotkey::{HotKey, Modifiers, Code}};
use std::thread;

use tokio_util::sync::CancellationToken;
use tokio::time::Duration;

mod record_sound;
use record_sound::record_wav;

async fn record_and_transcribe(token: CancellationToken) { 
    let res = record_wav(token).await;

    if let Err(err) = res {
        eprintln!("Failed to record microphone: {err}");
        return;
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
                    let clone_token = token.clone();
                    let _handle = tokio::spawn(
                        record_and_transcribe(clone_token)
                    );

                    // // Wait 
                    // match handle.await {
                    //     Err(err) => {
                    //         eprintln!("Task failed to execute: {err}");
                    //     },
                    //     Ok(result) => {
                    //         if let Err(err) = result {
                    //             eprintln!("Error recording sound: {err}")
                    //         }
                    //     }
                    // }

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
