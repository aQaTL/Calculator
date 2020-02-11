use iced::widget::button::Style;
use iced::*;

pub enum CalcButton {
	Number,
	PrimaryOperation,   //Usually the left veritacal bar
	SecondaryOperation, //Usually the top horizontal bar0
}

impl button::StyleSheet for CalcButton {
	fn active(&self) -> Style {
		let style: Box<dyn button::StyleSheet> = Default::default();
		let mut style = style.active();
		style.border_radius = 0;
		style.border_color = color_from_hex(0x999999);
		style.text_color = color_from_hex(0x111111);
		match self {
			CalcButton::PrimaryOperation => {
				style.background = Some(Background::from(color_from_hex(0xf79432)));
				style.text_color = color_from_hex(0xfff1df);
			}
			CalcButton::SecondaryOperation => {
				style.background = Some(Background::from(color_from_hex(0xd6d6d6)));
			}
			CalcButton::Number => {
				style.background = Some(Background::from(color_from_hex(0xe0e0e0)));
			}
		}
		style
	}

	fn hovered(&self) -> Style {
		self.active()
	}

	fn pressed(&self) -> Style {
		self.active()
	}

	fn disabled(&self) -> Style {
		self.active()
	}
}

pub struct HexColor(pub u32);

pub fn color_from_hex(c: u32) -> Color {
	Color::from(HexColor(c))
}

impl From<HexColor> for Color {
	fn from(c: HexColor) -> Self {
		Color {
			r: (c.0 >> 8 * 2 & 0xff) as f32 / 255.0,
			g: (c.0 >> 8 * 1 & 0xff) as f32 / 255.0,
			b: (c.0 >> 8 * 0 & 0xff) as f32 / 255.0,
			a: 1.0,
		}
	}
}

pub struct Container;

impl container::StyleSheet for Container {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::from(color_from_hex(0x272728))),
			..Default::default()
		}
	}
}
