use rustsimplegui as sg;

fn main() {
	let layout = vec![
		vec![
			sg::text("Robot Controller")
		],
		vec![],
		vec![
			sg::text(""),
			sg::button("Up"),
			sg::text("")
		], 
		vec![
			sg::button("Left"),
			sg::button("Down"),
			sg::button("Right")
		],
		vec![
			sg::input(),
			sg::input_ex("Hello", sg::RsgObjEx{
				size: (10, 10)
			})
		]
	];

	sg::window("Robot Controller", layout);

	loop {
		let (event, values) = sg::window_read();
		match event {
			_ => {
				println!("{}:{}", event, values.get(1).unwrap());
			}			
		};
	}
}