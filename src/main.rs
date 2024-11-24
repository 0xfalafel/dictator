use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState, hotkey::{HotKey, Modifiers, Code}};
use std::thread;
use std::time::Duration;

fn main() {
    // initialize the hotkeys manager
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::SUPER), Code::KeyM);
    manager.register(hotkey).err().map(|e| {
        eprintln!("Could not register the global hotkey Super+M: {}", e);
        std::process::exit(1);
    });


    loop {
        
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            // println!("{:?}", event.state);

            match event.state {
                HotKeyState::Pressed  => println!("Hi mom!"),
                HotKeyState::Released => println!("Bye mom!"),
            }
        }


        thread::sleep(Duration::from_millis(100));
    }
}
