use cursive::views::{Dialog, TextView};
use cursive::views::*;
use cursive::view::Nameable;
use std::fs::{File, read_to_string, create_dir};
use std::io::{Read, Write};
use uuid::Uuid;
use loco::internal::{LoginData, TokenClient};
use loco::internal::agent::Os;
use cursive::CursiveExt;
use dirs::home_dir;

fn main() {
	let mut siv = cursive::Cursive::default();
	let mut list_view = ListView::new();
	list_view.add_child("EMAIL", EditView::new().with_name("email"));
	list_view.add_child("PASSWORD", EditView::new().secret().with_name("password"));
	let log = TextView::new("").with_name("log");
	siv.add_fullscreen_layer(log);

	let token_client = TokenClient::new(Os::Win32);
	let data_path = home_dir().unwrap().join(".tuccflop");
	create_dir(&data_path);
	let uuid_path = data_path.join("uuid.txt");
	let uuid = match read_to_string(&uuid_path) {
		Ok(s) => s,
		Err(e) => {
			let s = Uuid::new_v4().to_hyphenated().to_string();
			match File::create(uuid_path) {
				Ok(mut file) => file.write_all(s.as_bytes()),
				Err(e) => panic!(e)
			};
			s
		}
	};

	siv.add_layer(
		Dialog::around(ResizedView::with_fixed_width(40, list_view))
			.title("로그인하심시오")
			.padding_lrtb(1,1,1,1)
			.button("확인", move |s| {
				let email = s.call_on_name("email", |v: &mut EditView| v.get_content()).unwrap().to_string();
				let password = s.call_on_name("password", |v: &mut EditView| v.get_content()).unwrap().to_string();

				let login_data = LoginData::new(
					email.to_owned(),
					password.to_owned(),
					&uuid,
					whoami::hostname(),
					"10.0".to_string(), //TODO
					false,
					false
				);
				let handler = token_client.request_login(&login_data);
				s.call_on_name("log", move |v: &mut TextView| {
					v.append(format!("using email: {}\r\n", &email));
					v.append(format!("{:?}\r\n", handler));
				});
				s.pop_layer();
			}).with_name("login_dialog")
	);

	// Starts the event loop.
	siv.run();
}