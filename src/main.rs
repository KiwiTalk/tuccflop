#[macro_use] extern crate lazy_static;
mod screen;
use cursive::views::{Dialog, TextView};
use cursive::views::*;
use cursive::view::Nameable;
use std::fs::{File, read_to_string, create_dir};
use std::io::{Read, Write};
use uuid::Uuid;
use loco::internal::{LoginData, TokenClient};
use loco::internal::agent::Os;
use cursive::{CursiveExt, Cursive};
use dirs::home_dir;
use std::sync::{Arc, Mutex};

lazy_static! {
	static ref CLIENT: Mutex<TokenClient> = Mutex::new(TokenClient::new(Os::Win32));
}

fn main() {
	let mut screen = screen::Screen::init();
	while true {
		let login_data = screen.get_login_data();
		let mut response = CLIENT.lock().unwrap().request_login(&login_data);
		screen.log(&format!("{:?}", response));
		match response {
			Ok(login_access_data) => {
				match login_access_data.status {
					12 => screen.dialog("비번틀렸심시오"),
					30 => screen.dialog("없는아이디심시오"),
					-100 => screen.dialog("기기등록않대있심시오"),
					0 => break,
					_ => {}
				}
			},
			Err(e) => panic!(e)
		}
	}
	screen.get_login_data();
	screen.get_login_data();
}

fn get_uuid() -> String {
	let data_path = home_dir().unwrap().join(".tuccflop");
	create_dir(&data_path);
	let uuid_path = data_path.join("uuid.txt");
	match read_to_string(&uuid_path) {
		Ok(s) => s,
		Err(e) => {
			let s = Uuid::new_v4().to_hyphenated().to_string();
			match File::create(uuid_path) {
				Ok(mut file) => file.write_all(s.as_bytes()),
				Err(e) => panic!(e)
			};
			s
		}
	}
}