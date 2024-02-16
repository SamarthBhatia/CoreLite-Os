use core::fmt::{Write, Result};
use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]

pub enum Colour {
	Red = 0,
	Green = 1,
	BLue = 2,
	Black = 3,
	White = 4,
	Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]


struct ColourCode(u8);
impl ColourCode {
	fn new(foreground: Colour, background: Colour) -> ColourCode {
		ColourCode((background as u8) << 4 | (foreground as u8))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]


struct ScreenCharacter {
	ascii_char: u8,
	colour_code: ColourCode,
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[repr(transparent)]

struct Buffer {
	chars: [[Volatile<ScreenCharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct ScreenWriter {
	column_pos: usize,
	colour_code: ColourCode,
	buffer: &'static mut Buffer,	
}


impl ScreenWriter {
	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.column_pos >= BUFFER_WIDTH {
					self.new_line();
				}

				let row = BUFFER_HEIGHT - 1;
				let col = self.column_pos;

				let colour_code = self.colour_code;
				self.buffer.chars[row][col].write(ScreenCharacter {
					ascii_char: byte, 
					colour_code,
				});
				self.column_pos += 1;
			}
		}
	}

	fn write_string(&mut self, s: &str) {
		for byte in s.bytes() {
			match byte {
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				_ => self.write_byte(0xfe),
			}
		}
	}
	fn new_line(&mut self) {
		/* Work In Prog*/
	}
}

impl Write for ScreenWriter {
	fn write_str(&mut self, s: &str) -> Result {
		self.write_string(s);
		Ok(())
	}
}

pub fn print_on_Display() {
	let mut screenWriter = ScreenWriter {
		column_pos: 0,
		colour_code: ColourCode::new(Colour::Red, Colour::Green),
		buffer: unsafe {
			&mut *(0xb8000 as *mut Buffer)
		},
	};

	screenWriter.write_byte(b'G');
	screenWriter.write_string("ood Day Everyone ");
	screenWriter.write_string("I am BareMetal-OS");
	write!(screenWriter, "The numbers are {} and {}", 42 ,1.0 / 3.0).unwrap();
}

