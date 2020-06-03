use std::convert::TryFrom;

pub struct Colour {
	r: u8,
	g: u8,
	b: u8,
	a: u8,
}

impl From<u32> for Colour {
	fn from(value: u32) -> Self {
		Colour {
			r: (value >>  0) as u8,
			g: (value >>  8) as u8,
			b: (value >> 16) as u8,
			a: (value >> 24) as u8,
		}
	}
}

impl TryFrom<&str> for Colour {
	type Error = &'static str;
	
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		fn try_from_option(value: &str) -> Option<Colour> {
			let mut chars = value.chars().peekable();
			
			let formatted_value: String;
			
			if chars.peek() == Some(&'#') { chars.next(); }
			let c0 = chars.next()?;
			let c1 = chars.next()?;
			let c2 = chars.next()?;
			if chars.peek() == None {
				formatted_value = format!("{0}{0}{1}{1}{2}{2}FF", c0, c1, c2);
			} else {
				let c3 = chars.next()?;
				if chars.peek() == None {
					formatted_value = format!("{0}{0}{1}{1}{2}{2}{3}{3}", c0, c1, c2, c3);
				} else {
					let c4 = chars.next()?;
					let c5 = chars.next()?;
					if chars.peek() == None {
						formatted_value = format!("{0}{1}{2}{3}{4}{5}FF", c0, c1, c2, c3, c4, c5);
					} else {
						let c6 = chars.next()?;
						let c7 = chars.next()?;
						if chars.peek() == None {
							formatted_value = format!("{0}{1}{2}{3}{4}{5}{6}{7}", c0, c1, c2, c3, c4, c5, c6, c7);
						} else {
							return None;
						}
					}
				}
			}
			
			let colour_int = u32::from_str_radix(&formatted_value, 16).ok()?;
			
			Some(Colour::from(colour_int))
		}
		
		try_from_option(value).ok_or("Not a valid colour")
	}
}

impl From<Colour> for u32 {
	fn from(value: Colour) -> u32 {
		(value.r as u32) <<  0 |
		(value.g as u32) <<  8 |
		(value.b as u32) << 16 |
		(value.a as u32) << 24
	}
}

impl From<Colour> for String {
	fn from(value: Colour) -> String {
		format!("#{:X}", u32::from(value))
	}
}
