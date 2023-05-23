
#![allow(dead_code)]

use volatile::Volatile;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    fn new(f: Color, b: Color) -> ColorCode{
        ColorCode((b as u8) << 4 | (f as u8))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar{
    ascii_char: u8,
    color: ColorCode
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer{
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8){
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.column_position >= BUFFER_WIDTH{
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color: self.color_code,
                });

                self.column_position += 1;    


            
            }

            
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }

        }
    }

    fn new_line(&mut self){
        for i in 0..BUFFER_HEIGHT-1{
            for j in 0..BUFFER_WIDTH{
                self.buffer.chars[i][j] = self.buffer.chars[i+1][j].clone();
            }
        }

        for i in 0..BUFFER_WIDTH{
            self.buffer.chars[BUFFER_HEIGHT-1][i].write(ScreenChar{ ascii_char: 0, color: ColorCode(0) });
        }

        self.column_position = 0;
    }
    
}


impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub fn print_something(){
    use core::fmt::Write;
        
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)}
    };


    write!(writer, "    Ola, Mundo").unwrap();

    
}

