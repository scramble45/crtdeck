use notify_rust::Notification;
use sdl2::{
    controller::{Button, GameController},
    event::Event,
    keyboard::Keycode,
};
use std::collections::HashMap;
use std::process::Command;

// Desktop notification
fn notify(body: &str) {
    Notification::new()
        .summary("CRT-Deck")
        .appname("CRT-Deck")
        .body(body)
        .icon("firefox")
        .show()
        .expect("Failed to notify!");
}

fn steam_ascii() {
    println!("{}", "
      .:::...
       :::::::..
      ... ..:::::.
  ^!??J??7~. .::::.
:?YYYJJJJYYJ^ .::::.
JYJJJJJJJJJJJ: .::::
JYJJJJJJJJJJJ: .::::
:?YYJJJJJJJJ^ .::::.
  ^!??J??7~. .::::.
      ... ..:::::.
       :::::::..
      .:::...
    ");
}

fn main() {
    steam_ascii();
    // Initialize SDL
    let sdl_ctx = sdl2::init().unwrap();
    // Initialize game controller subsystem
    let controller_subsystem = sdl_ctx.game_controller().unwrap();
    let mut gamepads: HashMap<u32, GameController> = HashMap::new();
    // Obtain SDL event pump
    let mut event_pump = sdl_ctx.event_pump().unwrap();
    // Default button combo state
    let mut buttons_vector: Vec<bool> = vec![false, false, false];

    'running: loop {
        // Obtain polling iterator for events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    println!("{}", keycode)
                }
                Event::ControllerDeviceAdded { which, .. } => {
                    println!("Device added index={}", which);
                    // When device connected open it so we receive button events
                    let gamepad = controller_subsystem.open(which).unwrap();
                    gamepads.insert(which, gamepad);
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    println!("Device removed index={}", which);
                    gamepads.remove(&(which as u32));
                }
                Event::ControllerButtonUp { which, button, .. } => {
                    match button {
                        Button::RightShoulder => buttons_vector[0] = false,
                        Button::Start => buttons_vector[1] = false,
                        Button::LeftShoulder => buttons_vector[2] = false,
                        _ => {},
                    };
                }
                Event::ControllerButtonDown { which, button, .. } => {
                    // Gamepad button pressed
                    println!("Controller index={} button={:?}", which, button);

                    match button {
                        Button::RightShoulder => buttons_vector[0] = true,
                        Button::Start => buttons_vector[1] = true,
                        Button::LeftShoulder => buttons_vector[2] = true,
                        _ => {},
                    };

                    let right_shoulder_button: bool = buttons_vector[0];
                    let start_button: bool = buttons_vector[1];
                    let left_shoulder_button: bool = buttons_vector[2];

                    // Resolution 1280x800
                    if right_shoulder_button && start_button {
                        notify("Changing resolution to 1280x800@60");

                        Command::new("kscreen-doctor")
                            .arg("output.eDP.mode.800x1280@60")
                            .spawn()
                            .expect("failed to set resolution with kscreen-doctor");
                    };

                    // Resolution 640x480
                    if left_shoulder_button && start_button {
                        notify("Changing resolution to 640x480@60");
                        Command::new("kscreen-doctor")
                            .arg("output.eDP.mode.640x480@60")
                            .spawn()
                            .expect("failed to set resolution with kscreen-doctor");
                    };

                    println!(
                        "Right shoulder {} Start {} Left shoulder {}",
                        right_shoulder_button, start_button, left_shoulder_button
                    );
                }
                _ => {}
            }
        }
    }
}
