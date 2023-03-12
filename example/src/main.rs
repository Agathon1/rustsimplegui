use rustsimplegui as rsg;

fn main() {
	let layout = vec![
		vec![rsg::text("Hello World!")],
		vec![rsg::button_ex("Test Button 1", rsg::RsgObjEx{
			size: (0, 0), color: (rsg::RsgColor::None, rsg::RsgColor::Red),
			pad: (100, 10), ..rsg::RsgObjEx::default()
		})], 
		vec![rsg::button_ex("Test Button 2", rsg::RsgObjEx{
			size: (0, 0), color: (rsg::RsgColor::Red, rsg::RsgColor::None),
			pad: (10, 4), ..rsg::RsgObjEx::default()
		})],
		vec![rsg::separator()],
		vec![rsg::slider()],
		vec![rsg::separator()],
		vec![rsg::checkbox("Hello")],
		vec![rsg::radio("Radior"), rsg::radio("Mr_Sandman")]
	];

	let window = rsg::window("Window Title", layout);

	loop {
		let (_event, _values) = window.read();

		println!("{}", _event);

		//println!("{}", _values.get(0).unwrap_or(&"".to_string()));

		if _event == "Quit".to_string() {
			break;
		}

	}

	window.close();

}