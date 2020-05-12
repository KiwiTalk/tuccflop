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
	screen.login();
	screen.run();
}

fn get_uuid() -> String {
	let data_path = home_dir().unwrap().join(".tuccflop");
	match create_dir(&data_path) {
		_ => {}
	}
	let uuid_path = data_path.join("uuid.txt");
	match read_to_string(&uuid_path) {
		Ok(s) => s,
		Err(e) => {
			let s = Uuid::new_v4().to_hyphenated().to_string();
			match File::create(uuid_path) {
				Ok(mut file) => file.write_all(s.as_bytes()),
				Err(e) => panic!("{:?}", e)
			}.expect("ee");
			s
		}
	}
}