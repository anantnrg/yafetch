use std::{
	env::var,
	fs::read_to_string,
};

pub fn get_distro() -> String {
	match read_to_string("/etc/os-release") {
		Ok(content) => {
			content
				.lines()
				.find(|line| line.starts_with("PRETTY_NAME="))
				.map(|line| line.splitn(2, '=').nth(1).unwrap_or("").trim_matches('"'))
				.unwrap_or("Unknown")
				.to_string()
		}
		Err(_) => "Unknown".to_string(),
	}
}

pub fn get_shell() -> String {
	let shell = var("SHELL").unwrap_or_else(|_| {
		let user = var("USER").unwrap_or_else(|_| "Unknown".to_string());
		let passwd_path = format!("/etc/passwd");
		let content = read_to_string(&passwd_path).unwrap_or_default();
		for line in content.lines() {
			let parts: Vec<&str> = line.split(':').collect();
			if parts.len() >= 7 && parts[0] == user {
				return parts[6].to_string();
			}
		}
		"Unknown".to_string()
	});
	shell
}

pub fn get_wm() -> String {
	var("XDG_CURRENT_DESKTOP").unwrap_or("Unknown".to_string())
}
