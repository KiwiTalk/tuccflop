use cursive::{CursiveExt, Cursive};
use cursive::views::*;
use cursive::view::*;
use cursive::utils::markup::StyledString;
use cursive::theme::*;
use std::ops::{Deref, DerefMut};
use loco::internal::{LoginData, DeviceRegisterData};

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
		let log = TextView::empty().with_name("log");
		cursive.add_fullscreen_layer(log);
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
						StyledString::single_span("로그인중이심시오", Color::Light(BaseColor::Green).into())
					));
					let email = s.call_on_name("email", |v: &mut EditView| v.get_content()).unwrap().to_string();
					let password = s.call_on_name("password", |v: &mut EditView| v.get_content()).unwrap().to_string();
					let permanent = s.call_on_name("permanent", |v: &mut Checkbox| v.is_checked()).unwrap();
					let force = s.call_on_name("force", |v: &mut Checkbox| v.is_checked()).unwrap();
					s.set_user_data(LoginData::new(
						email.to_owned(),
						password.to_owned(),
						&crate::get_uuid(),
						whoami::hostname(),
						"10.0".to_string(), //TODO
						permanent,
						force
					));
					let login_data: &LoginData = s.user_data().unwrap();
					let response = crate::CLIENT.lock().unwrap().request_login(login_data);
					s.call_on_name("log", |v: &mut TextView| v.append(format!("{:?}\r\n", &response)));
					println!("{:?}", &response);
					match response {
						Ok(login_access_data) => {
							s.call_on_name("status", |v: &mut TextView| v.set_content(
								StyledString::single_span(
									match login_access_data.status {
										12 => "비번틀렸심시오",
										30 => "없는아이디심시오",
										-100 => "",
										0 => {
											""
										},
										_ => "알수없는 오류"
									}
									, Color::Dark(BaseColor::Red).into())
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
								0 => {
									s.quit();
									println!("{:?}", &login_access_data);
								}
								_ => {}
							}
						},
						Err(e) => panic!("{:?}", e)
					}
				}).with_name("login_dialog")
		);
	}

	pub fn register_device(cursive: &mut Cursive) {
		let login_data = cursive.take_user_data().unwrap();
		let response = crate::CLIENT.lock().unwrap().request_passcode(&login_data);
		cursive.call_on_name("log", |v: &mut TextView| v.append(format!("{:?}\r\n", &response)));
		println!("{:?}", &response);
		cursive.set_user_data(DeviceRegisterData::new(
							  login_data,
			"0000".to_owned()
		));
		let mut list_view = ListView::new();
		list_view.add_child("PASSCODE",
							EditView::new()
								.on_edit(
									|s, content, length| {
										if length == 0 {
											return;
										}
										if !content.chars().last().unwrap().is_ascii_digit() {
											s.call_on_name("passcode", |v: &mut EditView| v.set_content(&content[0..content.len()-1]));
										}
									}
								)
								.on_submit(|s, passcode| {
									if passcode.len() != 4 {
										s.call_on_name("status_passcode",
											|v: &mut TextView| {
												v.set_content(
													StyledString::single_span(
														"4글자입니다 휴먼",
														Color::Dark(BaseColor::Red).into()
													)
												)
											}
										);
										return;
									}
									let device_register_data: &mut DeviceRegisterData = s.user_data().unwrap();
									device_register_data.passcode = passcode.to_owned();
									let response = crate::CLIENT.lock().unwrap().register_device(&device_register_data).unwrap();
									let text: String = response.text().unwrap(); //TODO handle
									s.call_on_name("log", |v: &mut TextView| v.append(format!("{:?}\r\n", &text)));
									println!("{:?}", &text);
									if text == "{\"status\":-111}" {
										s.pop_layer();
									}
								})
								.max_content_width(4)
								.with_name("passcode")
								.fixed_width(5)
		);
		list_view.add_child("", TextView::empty().with_name("status_passcode"));
		cursive.add_layer(
			Dialog::around(list_view.fixed_width(40))
				.title("PASSCODE 입력하심시오")
				.padding_lrtb(1,1,1,1)
				.button("PASSCODE 다시받기", |s| {
					let login_data: &DeviceRegisterData = &s.user_data().unwrap();
					let response = crate::CLIENT.lock().unwrap().request_passcode(login_data);
					s.call_on_name("log", |v: &mut TextView| v.append(format!("{:?}\r\n", &response)));
					println!("{:?}", &response);
				})
		);
	}

	#[allow(dead_code)]
	pub fn dialog<S: ToString>(&mut self, message: S) {
		let dialog = Dialog::around(TextView::new(message.to_string())).button("확인했심시오", |s| {
			s.pop_layer();
			s.quit();
		}).padding_lrtb(1,1,1,1);
		self.add_layer(dialog);
		self.run();
	}
}