// This code IS NOT MY OWN and is based off of the tutorial at the following URL:
// https://os.phil-opp.com/vga-text-mode/
// I have been following this tutorial throughout the duration of this project,
// although there may be variations in variable names, type names and formatting.

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

// Lazy static means that, unlike regular static variables which are initialised
// at compile time, variables can be initialised on the first access.
// We need this for the WRITER below as we cannot obtain a Mutex at compile time.
lazy_static!
{
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        colour_code: ColourCode::new(Colour::Yellow, Colour::Black),
        vga_buffer: unsafe { &mut *(0xb8000 as *mut VGABuffer) }
    });
}

#[allow(dead_code)] // Silences compiler warnings about unused variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // Make it printable and comparable
#[repr(u8)] // Each enum attribute should be stored as a u8
pub enum Colour
{
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

/// Contains the full colour byte (foreground and background colours)
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // Enable copy semantics once again
#[repr(transparent)] // Ensure it has the exact same layout as u8
struct ColourCode(u8);

impl ColourCode
{
    fn new(foreground: Colour, background: Colour) -> Self
    {
        // Shift the background value left by 4 bits to occupy the left half of the byte.
        // Use bitwise OR to add the foreground value to the right half (the first 4 bits).
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}

/// A representation of a character on the screen.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)] // Specify C-style layout of this struct, since struct layout is undefined in Rust.
           // This also guarantees the ordering of the fields.
struct ScreenChar
{
    ascii_character: u8,
    colour_code: ColourCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// A representation of the VGA Buffer
#[repr(transparent)]
struct VGABuffer {
    /// The introduction of the Volatile wrapper means that the buffer won't be optimised away,
    /// but we must now use the write() method for writing to the buffer
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// An abstraction for safe writing to the screen (via the VGA Buffer).
///
/// The writer will write to the last line and shift lines up when a line is full or a newline
/// character (\n) is read.
pub struct Writer
{
    column_position: usize,
    colour_code: ColourCode,
    vga_buffer: &'static mut VGABuffer, // 'static used here because VGA Buffer will live for the
                                        // lifetime of the OS
}

impl fmt::Write for Writer
{
    /// Implementing this trait means we can use the write!() and writeln!() macros.
    fn write_str(&mut self, s: &str) -> fmt::Result
    {
        self.write_string(s);
        Ok(())
    }
}

impl Writer
{
    pub fn write_string(&mut self, string: &str)
    {
        for byte in string.bytes()
        {
            match byte
            {
                // Part of printable ASCII range, or a newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Not part of printable ASCII range, display a box instead
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8)
    {
        match byte
        {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH
                {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let column = self.column_position;

                let colour_code = self.colour_code;
                self.vga_buffer.chars[row][column].write(ScreenChar {
                    ascii_character: byte,
                    colour_code,
                });

                self.column_position += 1;
            }
        }
    }

    /// Iterate over every character in the buffer from row 1 to BUFFER_HEIGHT - 1 and write
    /// each character to the row above.
    /// We start at row 1 since the first row is row 0. Everything in row 0
    /// is overwritten.
    fn new_line(&mut self)
    {
        for row in 1..BUFFER_HEIGHT
        {
            for col in 0..BUFFER_WIDTH
            {
                let character = self.vga_buffer.chars[row][col].read();
                self.vga_buffer.chars[row - 1][col].write(character);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Overwrite the contents of an entire row with spaces.
    fn clear_row(&mut self, row: usize)
    {
        let blank = ScreenChar {
            ascii_character: b' ',
            colour_code: self.colour_code,
        };

        for col in 0..BUFFER_WIDTH
        {
            self.vga_buffer.chars[row][col].write(blank);
        }
    }
}

pub fn print_hello_world()
{
    let mut writer = Writer {
        column_position: 0,
        colour_code: ColourCode::new(Colour::LightBlue, Colour::Black),
        vga_buffer: unsafe { &mut *(0xb8000 as *mut VGABuffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("World!");
}


#[macro_export]
macro_rules! print
{
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println
{
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments)
{
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[test_case]
fn test_println_simple()
{
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many()
{
    for _ in 0..200
    {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output()
{
    let s = "Some test string that fits on a single line";
    println!("{}", s);

    // Iterate over every character in the VGA buffer to ensure the string "s"
    // was actually printed.
    for (i, c) in s.chars().enumerate()
    {
        let screen_char = WRITER.lock().vga_buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}
