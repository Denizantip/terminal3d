// Pixel type, represents a chunk of 4 cells, 
// to be converted to a single char on the screen.
// Order is top-left, top-right, bottom-left, bottom-right.
type Pixel = (bool, bool, bool, bool);

// Handle pixel to char conversion.
fn pixel_to_char(pixel: &Pixel) -> char {
    match pixel {
        (false, false, false, false) => ' ',
        (true, false, false, false) => '▘',
        (false, true, false, false) => '▝',
        (true, true, false, false) => '▀',
        (false, false, true, false) => '▖',
        (true, false, true, false) => '▌',
        (false, true, true, false) => '▞',
        (true, true, true, false) => '▛',
        (false, false, false, true) => '▗',
        (true, false, false, true) => '▚',
        (false, true, false, true) => '▐',
        (true, true, false, true) => '▜',
        (false, false, true, true) => '▄',
        (true, false, true, true) => '▙',
        (false, true, true, true) => '▟',
        (true, true, true, true) => '█'
    }
}

// Simple 2d point wrapper.
pub struct Point {
    pub x: usize,
    pub y: usize
}

impl Point {
    // Create a new point.
    pub fn new(x: usize, y: usize) -> Point {
        Point {
            x: x, 
            y: y
        }
    }
}

// Wrapper for a "screen" to render.
pub struct Screen {
    pub dimensions: Point,
    content: Vec<Vec<bool>>
}

impl Screen {

    // Create a new screen.
    pub fn new(dimensions: Point) -> Screen {
        print!(
            "{}{}", 
            termion::clear::All, 
            termion::cursor::Goto(1, 1)
        );
        Screen{
            content: vec![vec![false; dimensions.x]; dimensions.y],
            dimensions: dimensions
        }
    }

    // Write a value to a coord on the screen.
    pub fn write(&mut self, val: bool, point: &Point) {
        assert!(point.x < self.dimensions.x);
        assert!(point.y < self.dimensions.y);
        self.content[point.y][point.x] = val;
    }

    // Clears the whole screen.
    pub fn clear(&mut self) {
        self.content = vec![vec![false; self.dimensions.x]; self.dimensions.y];
    }

    // Render the screen.
    pub fn render(&self) {
        print!("{}", termion::cursor::Goto(1, 1));

        for real_y in 0..(self.dimensions.y / 2) {
            // Extract the relavent rows in the content matrix.
            let rows = (
                &self.content[real_y * 2],
                &self.content[real_y * 2 + 1]
            );

            for real_x in 0..(self.dimensions.x / 2) {
                // Extract the relavent pixel in the content matrix, and print it out.
                let pixel: Pixel = (
                    rows.0[real_x * 2], rows.0[real_x * 2 + 1],
                    rows.1[real_x * 2], rows.1[real_x * 2 + 1]
                );
                print!("{}", pixel_to_char(&pixel));
            }

            // Handle case of odd width by adding another char.
            if self.dimensions.x % 2 == 1 {
                let pixel: Pixel = (
                    rows.0[self.dimensions.x - 1], false,
                    rows.1[self.dimensions.x - 1], false
                );
                print!("{}", pixel_to_char(&pixel));
            }

            print!("\n")
        }

        // Handle case of odd height by adding another char to every column.
        if self.dimensions.y % 2 == 1 {
            let last_row = &self.content[self.dimensions.y - 1];
            for real_x in 0..(self.dimensions.x / 2) {
                // Extract the relavent pixel in the content matrix, and print it out.
                let pixel: Pixel = (
                    last_row[real_x * 2], last_row[real_x * 2 + 1],
                    false, false
                );
                print!("{}", pixel_to_char(&pixel));
            }

            // Handle odd width.
            if self.dimensions.x % 2 == 1 {
                let pixel: Pixel = (
                    last_row[self.dimensions.x - 1], false,
                    false, false
                );
                print!("{}", pixel_to_char(&pixel));
            }
        }
    }
}