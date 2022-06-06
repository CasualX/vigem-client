use std::{thread, time, sync, sync::atomic};

fn sleep(ms: u32) {
	thread::sleep(time::Duration::from_millis(ms as u64));
}

fn main() {
	let xinput = rusty_xinput::XInputHandle::load_default().unwrap();

	let client = vigem_client::Client::connect().unwrap();
	let mut target = vigem_client::XTarget::new(client, vigem_client::TargetId::XBOX360_WIRED);

	target.plugin().unwrap();
	target.wait_ready().unwrap();

	// I'm expecting this to return the user index for use with xinput
	// However it seems to always return 0, even though that's my real controller
	// The ViGEm virtual controller has user index 1 on my machine.
	let user_index = 1; //target.get_user_index().unwrap();

	// Fudge the timing a little :D
	sleep(100);

	// The input state of the virtual controller
	let gamepad = vigem_client::XGamepad {
		buttons: vigem_client::XButtons!(UP | RIGHT | LB | A | X),
		..Default::default()
	};
	target.update(&gamepad).unwrap();

	let count = sync::Arc::new(atomic::AtomicUsize::new(0));

	// Handle notifications on a separate thread
	let counter = count.clone();
	let thread = target.request_notification().unwrap().spawn_thread(move |_, data| {
		counter.fetch_add(1, atomic::Ordering::SeqCst);
		println!("{:#?}", data);
	});

	// Give the notification thread some time to start up
	sleep(100);

	// Generate some random motor speeds
	let mut rng = urandom::new();
	for _ in 0..20 {
		let motor_speed = rng.next_u32();
		let left_motor_speed = (motor_speed & 0xffff) as u16;
		let right_motor_speed = (motor_speed >> 16) as u16;
		// XInput the motor speeds are u16, but ViGEm only uses the high byte...
		println!("xinput.set_state({}, {}, {})", user_index, left_motor_speed >> 8, right_motor_speed >> 8);
		xinput.set_state(user_index, left_motor_speed, right_motor_speed).unwrap();
		sleep(rng.range(50..200));
	}

	// Dropping the target causes the notification request to abort and the thread to return
	drop(target);
	thread.join().unwrap();

	println!("Got {} notifications!", count.load(atomic::Ordering::SeqCst));
}
