use std::io;
use crossterm::{
    execute, 
    terminal,
    cursor,
    style
};

// Setup ability to get dimensions out of matrix arrays.
pub trait Dim {
    const WIDTH: usize;
    const HEIGHT: usize;
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Dim for [[T; WIDTH]; HEIGHT] {
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;
}

const DEFAULT_TERMINAL_DIMENSIONS: (u16, u16) = (80, 24);

// Pixel types, represent a single char.
type BlockPixel = [[bool; 2]; 2];
type BrailePixel = [[bool; 2]; 4];

// Handle block pixel to char conversion.
fn block_pixel_to_char(pixel: &BlockPixel) -> char {
    match pixel {
        [[false, false], [false, false]] => ' ',
        [[true, false], [false, false]] => '▘',
        [[false, true], [false, false]] => '▝',
        [[true, true], [false, false]] => '▀',
        [[false, false], [true, false]] => '▖',
        [[true, false], [true, false]] => '▌',
        [[false, true], [true, false]] => '▞',
        [[true, true], [true, false]] => '▛',
        [[false, false], [false, true]] => '▗',
        [[true, false], [false, true]] => '▚',
        [[false, true], [false, true]] => '▐',
        [[true, true], [false, true]] => '▜',
        [[false, false], [true, true]] => '▄',
        [[true, false], [true, true]] => '▙',
        [[false, true], [true, true]] => '▟',
        [[true, true], [true, true]] => '█'
    }
}

// Handle braile pixel to char conversion.
fn braile_pixel_to_char(pixel: &BrailePixel) -> char {
    let mut unicode: u32 = 0;
    if pixel[0][0] { unicode |= 1 << 0 }
    if pixel[1][0] { unicode |= 1 << 1 }
    if pixel[2][0] { unicode |= 1 << 2 }

    if pixel[0][1] { unicode |= 1 << 3 }
    if pixel[1][1] { unicode |= 1 << 4 }
    if pixel[2][1] { unicode |= 1 << 5 }

    if pixel[3][0] { unicode |= 1 << 6 }
    if pixel[3][1] { unicode |= 1 << 7 }

    unicode |= 0x28 << 8;

    char::from_u32(unicode).unwrap()
}

// Simple 2d point wrapper.
#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    // Create a new point.
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

// Wrapper for a "screen" to render.
pub struct Screen {
    pub width: u16,
    pub height: u16,
    content: Vec<Vec<bool>>,
}

impl Screen {
    // Create a new screen, sized to the terminal.
    pub fn new() -> Screen {
        // Clear term and go to 0, 0.
        execute!(
            io::stdout(),
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::All)
        ).unwrap();

        // Create screen.
        Screen{
            content: Vec::new(),
            width: 0,
            height: 0
        }
    }

    // Resize braile screen to fit terminal width and height.
    pub fn fit_braile_to_terminal(&mut self) {
        let (terminal_width, terminal_height) = match terminal::size() {
            Ok(dim) => dim,
            Err(_) => DEFAULT_TERMINAL_DIMENSIONS
        };

        self.resize(
            terminal_width * BrailePixel::WIDTH as u16, 
            (terminal_height - 1) * BrailePixel::HEIGHT as u16
        );
    }

    // Resize block screen to fit terminal width and height.
    pub fn fit_block_to_terminal(&mut self) {
        let (terminal_width, terminal_height) = match terminal::size() {
            Ok(dim) => dim,
            Err(_) => DEFAULT_TERMINAL_DIMENSIONS
        };

        self.resize(
            terminal_width *  BlockPixel::WIDTH as u16, 
            (terminal_height - 1) *  BlockPixel::HEIGHT as u16
        );
    }

    // Write a value to a coord on the screen.
    // If out of bounds, will simply not write.
    pub fn write(&mut self, val: bool, point: &Point) {
        let x_in_bounds = 0 < point.x && point.x < self.width as i32;
        let y_in_bounds = 0 < point.y && point.y < self.height as i32;
        if x_in_bounds && y_in_bounds {
            self.content[point.y as usize][point.x as usize] = val;
        }
    }

    // Clears the whole screen.
    pub fn clear(&mut self) {
        self.content = vec![vec![false; self.width as usize]; self.height as usize];
    }

    // Resizes the screen.
    // Either crops the image if the requested size is smaller,
    // or extends the image with empty cells if the request is larger.
    pub fn resize(&mut self, width: u16, height: u16) {
        // Handle height.
        if height > self.height {
            self.content.extend(vec![
                vec![false; width as usize]; 
                (height - self.height) as usize
            ])
        } else {
            self.content.truncate(height as usize);
        }
        self.height = height;

        // Handle width.
        if width > self.width {
            for row in self.content.iter_mut() {
                row.extend(vec![false; (width - self.width) as usize]);
            }
        } else {
            for row in self.content.iter_mut() {
                row.truncate(width as usize);
            }
        }
        self.width = width;
    }

    // Draw a line with Bresenham's line algorithm.
    // See https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm.
    pub fn line(&mut self, start: &Point, end: &Point) {            
        let delta_x = (end.x - start.x).abs();
        let step_x: i32 = if start.x < end.x {1} else {-1};
        let delta_y = -(end.y - start.y).abs();
        let step_y: i32 = if start.y < end.y {1} else {-1};
        let mut err = delta_x + delta_y;

        let mut x = start.x;
        let mut y = start.y;

        self.write(true, &Point::new(x, y));

        while !(x == end.x && y == end.y) {
            self.write(true, &Point::new(x, y));
            let curr_err = err;

            if 2 * curr_err >= delta_y {
                err += delta_y;
                x += step_x;
            }

            if 2 * curr_err <= delta_x {
                err += delta_x;
                y += step_y;
            }
        }
    }

    // Render the screen in blocks.
    pub fn render_block(&self) {
        execute!(
            io::stdout(),
            cursor::MoveTo(0, 0)
        ).unwrap();

        // Chunk rows by the height of a single pixel.
        let chunked_rows = self.content.chunks(BlockPixel::HEIGHT);

        // Run through chunks.
        for subrows in chunked_rows {

            // Produce a "real row" - a row of Pixel types.
            let real_row_width = self.width.div_ceil(BlockPixel::WIDTH as u16) as usize;
            let mut real_row = vec![[
                [false; BlockPixel::WIDTH]; BlockPixel::HEIGHT
            ]; real_row_width];

            // Run through every subrow, where subpixel_y is the y index within the pixel.
            for (subpixel_y, subrow) in subrows.iter().enumerate() {

                // Chunk the subrow by the width of the pixel.
                let chunked_subrow = subrow.chunks_exact(2);
                let remainder = chunked_subrow.remainder();

                // Update real row.
                for (real_x, pixel_row) in chunked_subrow.enumerate() {
                    real_row[real_x][subpixel_y][..pixel_row.len()].copy_from_slice(pixel_row);
                }
                
                // Handle remainder (indivisible width).
                real_row[real_row_width - 1][subpixel_y][..remainder.len()].copy_from_slice(remainder);
            }

            // Render.
            for pixel in real_row {
                execute!(io::stdout(), style::Print(block_pixel_to_char(&pixel))).unwrap();
            }
            execute!(io::stdout(), style::Print("\r\n")).unwrap();
        }
    }

    // Render the screen in braile.
    pub fn render_braile(&self) {
        execute!(
            io::stdout(),
            cursor::MoveTo(0, 0)
        ).unwrap();

        // Chunk rows by the height of a single pixel.
        let chunked_rows = self.content.chunks(BrailePixel::HEIGHT);

        // Run through chunks.
        for subrows in chunked_rows {

            // Produce a "real row" - a row of Pixel types.
            let real_row_width = self.width.div_ceil(BrailePixel::WIDTH as u16) as usize;
            let mut real_row = vec![[
                [false; BrailePixel::WIDTH]; BrailePixel::HEIGHT
            ]; real_row_width];

            // Run through every subrow, where subpixel_y is the y index within the pixel.
            for (subpixel_y, subrow) in subrows.iter().enumerate() {

                // Chunk the subrow by the width of the pixel.
                let chunked_subrow = subrow.chunks_exact(2);
                let remainder = chunked_subrow.remainder();

                // Update real row.
                for (real_x, pixel_row) in chunked_subrow.enumerate() {
                    real_row[real_x][subpixel_y][..pixel_row.len()].copy_from_slice(pixel_row);
                }
                
                // Handle remainder (indivisible width).
                real_row[real_row_width - 1][subpixel_y][..remainder.len()].copy_from_slice(remainder);
            }

            // Render.
            for pixel in real_row {
                execute!(io::stdout(), style::Print(braile_pixel_to_char(&pixel))).unwrap();
            }
            execute!(io::stdout(), style::Print("\r\n")).unwrap();
        }
    }
}