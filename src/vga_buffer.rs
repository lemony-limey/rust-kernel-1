// This code IS NOT MY OWN and is based off of the tutorial at the following URL:
// https://os.phil-opp.com/vga-text-mode/
// I have been following this tutorial throughout the duration of this project.

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
struct VGABuffer
{
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
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
                self.vga_buffer.chars[row][column] = ScreenChar {
                    ascii_character: byte,
                    colour_code,
                };

                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) { /* TODO */ }
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
