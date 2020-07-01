#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::*;

mod style;

use crate::style::HexColor;
use style::CalcButton;

fn main() {
	Calculator::run(Settings {
		window: window::Settings {
			size: (330, 350),
			..Default::default()
		},
		..Default::default()
	})
}

#[derive(Default)]
struct Calculator {
	button_states: ButtonStates,
	tokens: Vec<Token>,
}

#[derive(Debug)]
enum Token {
	Operation(Operation),
	Number(String),
	Percentage(String),
	Result(String),
}

impl ToString for Token {
	fn to_string(&self) -> String {
		match self {
			Token::Operation(op) => op.to_string(),
			Token::Number(n) | Token::Result(n) => n.clone(),
			Token::Percentage(n) => format!("{}%", n),
		}
	}
}

impl Application for Calculator {
	type Executor = executor::Default;
	type Message = Message;

	fn new() -> (Self, Command<Self::Message>) {
		(Self::default(), Command::none())
	}

	fn title(&self) -> String {
		String::from("Calculator")
	}

	fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
		use Operation::*;
		match message {
			Message::OperationButton(AC) => {
				self.tokens.clear();
			}
			Message::OperationButton(SignChange) => {
				let last_num_buf = self.tokens.iter_mut().rev().find_map(|t| match t {
					Token::Number(n) | Token::Percentage(n) => Some(n),
					_ => None,
				});
				if let Some(num_buf) = last_num_buf {
					if &num_buf[..][..1] == "-" {
						num_buf.remove(0);
					} else {
						num_buf.insert(0, '-');
					}
				}
			}
			Message::OperationButton(Percent) => match self.tokens.last_mut() {
				Some(t @ Token::Number(_)) => {
					let percentage = match t {
						Token::Number(n) => Token::Percentage(n.clone()),
						_ => unreachable!(),
					};
					*t = percentage;
				}
				_ => (),
			},
			Message::OperationButton(Equals) => {
				self.tokens.push(Token::Operation(Equals));
				self.calculate();
			}
			Message::OperationButton(Dot) => {
				let last_num_buf = self.tokens.iter().rev().find_map(|t| {
					if let Token::Number(n) = t {
						Some(n)
					} else {
						None
					}
				});
				if let Some(num_buf) = last_num_buf {
					if num_buf.contains(".") {
						return Command::none();
					}
				}
				match self.tokens.last_mut() {
					Some(Token::Number(ref mut num_buf)) => num_buf.push('.'),
					Some(_) | None => self.tokens.push(Token::Number(String::from("0."))),
				}
			}
			Message::OperationButton(op) => {
				if let Some(Token::Operation(op)) = self.tokens.last() {
					if let Sum | Subtract | Multiply | Divide = op {
						return Command::none();
					}
				}
				self.tokens.push(Token::Operation(op));
			}
			Message::NumberButton(number) => {
				let number = (number + b'0') as char;
				match self.tokens.last_mut() {
					Some(Token::Number(ref mut num_buf)) => num_buf.push(number),
					Some(_) | None => self.tokens.push(Token::Number(number.to_string())),
				}
			}
		}
		println!("{:?}", self.tokens);
		Command::none()
	}

	fn view(&mut self) -> Element<Message> {
		self.button_states.reset();
		let calc_button_label = |label| {
			Text::new(label)
				.horizontal_alignment(HorizontalAlignment::Center)
				.vertical_alignment(VerticalAlignment::Center)
				.size(28)
		};
		let calc_button = |state, content| {
			Button::new(state, content)
				.width(Length::FillPortion(1))
				.height(Length::FillPortion(1))
		};
		Container::new(
			Column::new()
				.push(
					Row::new()
						.push(
							Text::new(self.display())
								.horizontal_alignment(HorizontalAlignment::Right)
								.vertical_alignment(VerticalAlignment::Center)
								.color(HexColor(0xffffff))
								.width(Length::Fill)
								.size(38),
						)
						.push(
							Text::new("\u{202F}".repeat(2))
								.horizontal_alignment(HorizontalAlignment::Right)
								.vertical_alignment(VerticalAlignment::Center)
								.width(Length::Shrink)
								.size(38),
						)
						.height(Length::Fill),
				)
				.push(
					Row::new()
						.push(
							calc_button(self.button_states.next(), calc_button_label("AC"))
								.style(CalcButton::SecondaryOperation)
								.on_press(Message::OperationButton(Operation::AC)),
						)
						.push(
							calc_button(self.button_states.next(), calc_button_label("+/-"))
								.style(CalcButton::SecondaryOperation)
								.on_press(Message::OperationButton(Operation::SignChange)),
						)
						.push(
							calc_button(self.button_states.next(), calc_button_label("%"))
								.style(CalcButton::SecondaryOperation)
								.on_press(Message::OperationButton(Operation::Percent)),
						)
						.push(
							calc_button(self.button_states.next(), calc_button_label("\u{00F7}"))
								.style(CalcButton::PrimaryOperation)
								.on_press(Message::OperationButton(Operation::Divide)),
						)
						.height(Length::Fill),
				)
				.push(
					Row::new()
						.push(
							calc_button(self.button_states.next(), calc_button_label("7"))
								.style(CalcButton::Number)
								.on_press(Message::NumberButton(7)),
						)
						.push(
							calc_button(self.button_states.next(), calc_button_label("8"))
								.style(CalcButton::Number)
								.on_press(Message::NumberButton(8)),
						)
						.push(
							calc_button(self.button_states.next(), calc_button_label("9"))
								.style(CalcButton::Number)
								.on_press(Message::NumberButton(9)),
						)
						.push(
							calc_button(self.button_states.next(), calc_button_label("X"))
								.style(CalcButton::PrimaryOperation)
								.on_press(Message::OperationButton(Operation::Multiply)),
						)
						.height(Length::Fill),
				)
				.push(
					Row::new()
						.push(
							calc_button(self.button_states.next(), calc_button_label("4"))
								.style(CalcButton::Number)
								.on_press(Message::NumberButton(4)),
						)
						.push(
							calc_button(self.button_states.next(), calc_button_label("5"))
								.style(CalcButton::Number)
								.on_press(Message::NumberButton(5)),
						)
						.push(
							calc_button(self.button_states.next(), calc_button_label("6"))
								.style(CalcButton::Number)
								.on_press(Message::NumberButton(6)),
						)
						.push(
							calc_button(self.button_states.next(), calc_button_label("-"))
								.style(CalcButton::PrimaryOperation)
								.on_press(Message::OperationButton(Operation::Subtract)),
						)
						.height(Length::Fill),
				)
				.push(
					Row::new()
						.push(
							calc_button(self.button_states.next(), calc_button_label("1"))
								.style(CalcButton::Number)
								.on_press(Message::NumberButton(1)),
						)
						.push(
							calc_button(self.button_states.next(), calc_button_label("2"))
								.style(CalcButton::Number)
								.on_press(Message::NumberButton(2)),
						)
						.push(
							calc_button(self.button_states.next(), calc_button_label("3"))
								.style(CalcButton::Number)
								.on_press(Message::NumberButton(3)),
						)
						.push(
							calc_button(self.button_states.next(), calc_button_label("+"))
								.style(CalcButton::PrimaryOperation)
								.on_press(Message::OperationButton(Operation::Sum)),
						)
						.height(Length::Fill),
				)
				.push(
					Row::new()
						.push(
							calc_button(self.button_states.next(), calc_button_label("0"))
								.style(CalcButton::Number)
								.width(Length::FillPortion(2))
								.on_press(Message::NumberButton(0)),
						)
						.push(
							calc_button(self.button_states.next(), calc_button_label("."))
								.style(CalcButton::Number)
								.on_press(Message::OperationButton(Operation::Dot)),
						)
						.push(
							calc_button(self.button_states.next(), calc_button_label("="))
								.style(CalcButton::PrimaryOperation)
								.on_press(Message::OperationButton(Operation::Equals)),
						)
						.height(Length::Fill),
				),
		)
		.style(style::Container)
		.into()
	}
}

impl Calculator {
	fn display(&self) -> String {
		let str = self
			.tokens
			.iter()
			.map(|t| t.to_string())
			.collect::<String>();
		str
	}

	fn calculate(&mut self) {
		let mut result: f64 = 0.0;
		let mut last_op = None;
		for token in self.tokens.iter() {
			match token {
				Token::Number(num_buf) => {
					let num = num_buf.parse::<f64>().unwrap();
					match last_op {
						Some(op) => match op {
							Operation::Sum => result += num,
							Operation::Subtract => result -= num,
							Operation::Multiply => result *= num,
							Operation::Divide => result /= num,
							_ => unreachable!(),
						},
						None => result = num,
					}
				}
				Token::Percentage(num_buf) => {
					let num = num_buf.parse::<f64>().unwrap();
					match last_op {
						Some(Operation::Sum) => {
							result += result * (num / 100.0);
						}
						Some(Operation::Subtract) => {
							result -= result * (num / 100.0);
						}
						Some(Operation::Multiply) => {
							result *= num / 100.0;
						}
						Some(Operation::Divide) => {
							result /= num / 100.0;
						}
						None => {
							result = num / 100.0;
						}
						_ => unreachable!(),
					}
				}
				Token::Operation(Operation::Equals) => {
					//TODO repeat last op after first equals
				}
				Token::Operation(op) => last_op = Some(*op),
				Token::Result(_res) => (),
			}
		}
		self.tokens.push(Token::Result(result.to_string()));
	}
}

#[derive(Debug, Clone)]
enum Message {
	OperationButton(Operation),
	NumberButton(u8),
}

#[derive(Debug, Clone, Copy)]
enum Operation {
	AC,
	SignChange,
	Percent,
	Divide,
	Multiply,
	Subtract,
	Sum,
	Equals,
	Dot,
}

impl Operation {
	fn to_str(&self) -> &'static str {
		use Operation::*;
		match self {
			AC => "AC",
			SignChange => "-",
			Percent => "%",
			Divide => "\u{00F7}",
			Multiply => "*",
			Subtract => "-",
			Sum => "+",
			Equals => "=",
			Dot => ".",
		}
	}
}

impl ToString for Operation {
	fn to_string(&self) -> String {
		self.to_str().to_string()
	}
}

#[derive(Default)]
struct ButtonStates {
	idx: usize,
	buf: Vec<button::State>,
}

impl ButtonStates {
	pub fn next<'a, 'b>(&'a mut self) -> &'b mut button::State {
		if self.idx == self.buf.len() {
			self.buf.push(button::State::new());
		}
		self.idx += 1;
		let r = &mut self.buf[self.idx - 1];
		//Stop screaming
		unsafe { &mut *(r as *mut button::State) }
	}

	pub fn reset(&mut self) {
		self.idx = 0;
	}
}
