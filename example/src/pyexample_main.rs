use rustsimplegui as sg;

fn main() {
	let layout = vec![	vec![sg::text("What's your name?")],
						vec![sg::input()],
						vec![sg::button("Ok")] ];

	let window = sg::window("Window Title", layout);

	let (_event, values) = window.read();

	println!("Hello {}! Thanks for trying RustSimpleGUI", values[0]);

	window.close();
}