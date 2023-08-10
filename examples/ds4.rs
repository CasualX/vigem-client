use std::{thread, time};

use vigem_client::{BatteryStatus, DS4Buttons, DS4ReportExBuilder, DS4SpecialButtons, DS4Status};

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

    let start = time::Instant::now();
    loop {
        let elapsed = start.elapsed().as_secs_f64();

        // Play for 10 seconds
        if elapsed >= 10.0 {
            break;
        }

        let report = DS4ReportExBuilder::new()
            // Spin the right thumb stick in circles
            .thumb_lx(((elapsed.cos() + 1.) * 127.) as u8)
            .thumb_ly(((elapsed.sin() + 1.) * 127.) as u8)
            // Spin the right thumb stick in circles
            .thumb_rx(255 - ((elapsed.cos() + 1.) * 127.) as u8)
            .thumb_ry(255 - ((elapsed.sin() + 1.) * 127.) as u8)
            // Twiddle the triggers
            .trigger_l(((((elapsed * 1.5).sin() * 127.0) as i32) + 127) as u8)
            .trigger_r(((((elapsed * 1.5).cos() * 127.0) as i32) + 127) as u8)
            .buttons(DS4Buttons::new().cross(true).circle(true))
            .special(DS4SpecialButtons::new().ps_home(true))
            .status(DS4Status::with_battery_status(BatteryStatus::Charging(8)))
            .build();

        let _ = target.update_ex(&report);

        thread::sleep(time::Duration::from_millis(10));
    }
}
