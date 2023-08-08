use std::{thread, time};

fn main() {
    // Connect to the ViGEmBus driver
    let client = vigem_client::Client::connect().unwrap();

    // Create the virtual controller target
    let id = vigem_client::TargetId::DUALSHOCK4_WIRED;
    let mut target = vigem_client::DualShock4Wired::new(client, id);

    // Plugin the virtual controller
    target.plugin().unwrap();

    // Wait for the virtual controller to be ready to accept updates
    target.wait_ready().unwrap();

    // The input state of the virtual controller
    let mut gamepad = vigem_client::DS4ReportEx {
        ..Default::default()
    };

    let start = time::Instant::now();
    loop {
        let elapsed = start.elapsed().as_secs_f64();

        // Play for 10 seconds
        if elapsed >= 10.0 {
            break;
        }

        // Spin the left thumb stick in circles
        gamepad.thumb_lx = ((elapsed.cos() + 1.) * 127.) as u8;
        gamepad.thumb_ly = ((elapsed.sin() + 1.) * 127.) as u8;

        // Spin the right thumb stick in circles
        gamepad.thumb_rx = gamepad.thumb_ly;
        gamepad.thumb_ry = gamepad.thumb_lx;

        // Twiddle the triggers
        gamepad.trigger_l = ((((elapsed * 1.5).sin() * 127.0) as i32) + 127) as u8;
        gamepad.trigger_r = ((((elapsed * 1.5).cos() * 127.0) as i32) + 127) as u8;

        let _ = target.update_ex(&gamepad);

        thread::sleep(time::Duration::from_millis(10));
    }
}
