use cursive::{CursiveExt, Cursive};
use cursive::views::*;
use cursive::view::Nameable;
use std::ops::{Deref, DerefMut};
use loco::internal::LoginData;

pub struct Screen {
	cursive: Cursive
}

impl Deref for Screen {
	type Target = Cursive;

	fn deref(&self) -> &Self::Target {
		return &self.cursive;
	}
}

impl DerefMut for Screen {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.cursive;
	}
}

impl Screen {
	pub fn init() -> Self {
		let mut cursive = Cursive::default();
		let log = TextView::new("").with_name("log");
		cursive.add_fullscreen_layer(log);
		cursive.refresh();
		Self {
			cursive
		}
	}

	pub fn get_login_data(&mut self) -> LoginData {
		let mut list_view = ListView::new();
		let mut login_data : Option<LoginData> = None;
		list_view.add_child("EMAIL", EditView::new().with_name("email"));
		list_view.add_child("PASSWORD", EditView::new().secret().with_name("password"));
		list_view.add_child("PERMANENT", Checkbox::new().unchecked().with_name("permanent"));
		self.add_layer(
			Dialog::around(ResizedView::with_fixed_width(40, list_view))
				.title("로그인하심시오")
				.padding_lrtb(1,1,1,1)
				.button("확인", |s| {
					let email = s.call_on_name("email", |v: &mut EditView| v.get_content()).unwrap().to_string();
					let password = s.call_on_name("password", |v: &mut EditView| v.get_content()).unwrap().to_string();
					let login_data = LoginData::new(
						email.to_owned(),
						password.to_owned(),
						&crate::get_uuid(),
						whoami::hostname(),
						"10.0".to_string(), //TODO
						s.call_on_name("permanent", |v: &mut Checkbox| v.is_checked()).unwrap(),
						false
					);
					s.set_user_data(login_data);
					s.pop_layer();
					s.quit();
				}).with_name("login_dialog")
		);
		self.run();
		self.take_user_data().unwrap()
	}
	pub fn log<S: ToString>(&mut self, log: S) {
		self.call_on_name("log", |v: &mut TextView| {
			v.append(log.to_string());
			v.append("\r\n");
		});
	}
	pub fn dialog<S: ToString>(&mut self, message: S) {
		let dialog = Dialog::around(TextView::new(message.to_string())).button("확인했심시오", |s| {
			s.pop_layer();
			s.quit();
		});
		self.add_layer(dialog);
		self.run();
	}
}