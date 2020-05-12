use cursive::{CursiveExt, Cursive};
use cursive::views::*;
use cursive::view::*;
use cursive::utils::markup::StyledString;
use cursive::theme::{Style, ColorStyle, ColorType, Color};
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
		let log = DebugView::new();
		cursive.add_fullscreen_layer(log);
		cursive.refresh();
		Self {
			cursive
		}
	}

	pub fn login(&mut self) {
		let mut list_view = ListView::new();
		list_view.add_child("EMAIL", EditView::new().with_name("email"));
		list_view.add_child("PASSWORD", EditView::new().secret().with_name("password"));
		list_view.add_child("PERMANENT", Checkbox::new().unchecked().with_name("permanent"));
		list_view.add_child("FORCE", Checkbox::new().unchecked().with_name("force"));
		list_view.add_child("", TextView::empty().with_name("status"));
		self.add_layer(
			Dialog::around(ResizedView::with_fixed_width(40, list_view))
				.title("로그인하심시오")
				.padding_lrtb(1,1,1,1)
				.button("확인", |s| {
					s.call_on_name("status", |v: &mut TextView| v.set_content(
						StyledString::single_span("로그인중이심시오", Color::parse("light_green").unwrap().into())
					));
					let email = s.call_on_name("email", |v: &mut EditView| v.get_content()).unwrap().to_string();
					let password = s.call_on_name("password", |v: &mut EditView| v.get_content()).unwrap().to_string();
					let permanent = s.call_on_name("permanent", |v: &mut Checkbox| v.is_checked()).unwrap();
					let force = s.call_on_name("force", |v: &mut Checkbox| v.is_checked()).unwrap();
					let login_data = LoginData::new(
						email.to_owned(),
						password.to_owned(),
						&crate::get_uuid(),
						whoami::hostname(),
						"10.0".to_string(), //TODO
						permanent,
						force
					);
					let response = crate::CLIENT.lock().unwrap().request_login(&login_data);
					println!("{:?}", &response);
					match response {
						Ok(login_access_data) => {
							s.call_on_name("status", |v: &mut TextView| v.set_content(
								StyledString::single_span(
									match login_access_data.status {
										12 => "비번틀렸심시오",
										30 => "없는아이디심시오",
										-100 => "기기등록않대있심시오",
										0 => {
											""
										},
										_ => "알수없는 오류"
									}
									, Color::parse("red").unwrap().into())
							));
							match login_access_data.status {
								-100 => {
									let dialog = Dialog::around(
										TextView::new("기기등록하시겠심시오?")
									).button("확인했심시오", |s| {
										s.pop_layer();
										Screen::register_device(s);
									}).dismiss_button("취소").padding_lrtb(1,1,1,1);
									s.add_layer(dialog);
								},
								_ => {}
							}
						},
						Err(e) => panic!("{:?}", e)
					}
				}).with_name("login_dialog")
		);
	}

	pub fn register_device(cursive: &mut Cursive) {
		let mut list_view = ListView::new();
		list_view.add_child("PASSCODE",
							EditView::new()
								.on_edit(
									|s, content: &str, length| {
										if content.parse::<u8>().is_err() {
											s.call_on_name("passcode", |v: &mut EditView| v.remove(1));
										}
									}
								)
								.with_name("passcode")
								.fixed_width(4)
		);
		cursive.add_layer(
			Dialog::around(list_view.fixed_width(40))
				.title("로그인하심시오")
				.padding_lrtb(1,1,1,1)
				.button("확인", |s| {})
		);
	}

	pub fn dialog<S: ToString>(&mut self, message: S) {
		let dialog = Dialog::around(TextView::new(message.to_string())).button("확인했심시오", |s| {
			s.pop_layer();
			s.quit();
		}).padding_lrtb(1,1,1,1);
		self.add_layer(dialog);
		self.run();
	}
}