mod libs;

use libs::info::{
	get_distro,
	get_shell,
	get_wm,
};

fn main() {
	let distro = get_distro();
	let shell = get_shell();
	let wm = get_wm();
	println!("{}", distro);
	println!("{}", shell);
	println!("{}", wm);
}
