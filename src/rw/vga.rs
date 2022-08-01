use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(C)]
struct Character {
    character: u8,
    color_code: u8,
}

#[repr(transparent)]
struct Buffer {
    items: [[Volatile<Character>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Screen<'s> {
    row: usize,
    col: usize,
    color_code: u8,
    buffer: &'s mut Buffer,
}

#[allow(dead_code)]
impl<'screen> Screen<'screen> {
    /// Constructor for the VGA Screen writer
    pub fn new(color: u8) -> Self {
        Screen {
            col: 0,
            row: 0,
            color_code: color,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    /// Print a single char onto the VGA buffer
    fn putch(&mut self, ch: u8) {
        if ch == b'\n' || self.col >= BUFFER_WIDTH {
            self.new_line();
        } else {
            self.buffer.items[self.row][self.col].write(Character {
                character: ch,
                color_code: self.color_code,
            });
            self.col += 1;
        }
    }

    /// Put a string onto the VGA buffer
    fn puts(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                0x20..=0x7e | 0xa => self.putch(byte),
                _ => self.putch(0xfe),
            }
        }
    }

    /// Insert a new line in the VGA buffer
    pub fn new_line(&mut self) {
        self.row += 1;
        self.col = 0;
        if self.row >= BUFFER_HEIGHT {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    self.buffer.items[row - 1][col].write(self.buffer.items[row][col].read());
                }
            }
            self.row = BUFFER_HEIGHT - 1;
            self.clear_row(self.row);
        }
    }

    /// Clear the specified row in the VGA buffer
    pub fn clear_row(&mut self, row: usize) {
        let clear = Character {
            character: 0x20,
            color_code: 0x00,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.items[row][col].write(clear);
        }
    }
}

impl core::fmt::Write for Screen<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.puts(s);
        Ok(())
    }
}
