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
        print!(
            "{}{}", 
            termion::clear::All, 
            termion::cursor::Goto(1, 1)
        );

        // Create screen.
        let mut res = Screen{
            content: Vec::new(),
            width: 0,
            height: 0
        };

        // Fit to terminal and return.
        res.fit_to_terminal();
        res
    }

    // Resize screen to fit terminal width and height.
    pub fn fit_to_terminal(&mut self) {
        let (
            terminal_width, 
            terminal_height
        ) = termion::terminal_size().unwrap();
        self.resize(terminal_width * 2, (terminal_height - 1) * 2);
    }

    // Write a value to a coord on the screen.
    // If out of bounds, will simply not write.
    pub fn write(&mut self, val: bool, point: &Point) {
        let x_in_bounds = 0 < point.x && (point.x as u16) < self.width;
        let y_in_bounds = 0 < point.y && (point.y as u16) < self.height;
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

    // Render the screen.
    pub fn render(&self) {
        print!("{}", termion::cursor::Goto(1, 1));

        for real_y in 0..(self.height / 2) {
            // Extract the relavent rows in the content matrix.
            let rows = (
                &self.content[real_y as usize * 2],
                &self.content[real_y as usize * 2 + 1]
            );

            for real_x in 0..(self.width / 2) {
                // Extract the relavent pixel in the content matrix, and print it out.
                let pixel: Pixel = (
                    rows.0[real_x as usize * 2], rows.0[real_x as usize * 2 + 1],
                    rows.1[real_x as usize * 2], rows.1[real_x as usize * 2 + 1]
                );
                print!("{}", pixel_to_char(&pixel));
            }

            // Handle case of odd width by adding another char.
            if self.width % 2 == 1 {
                let pixel: Pixel = (
                    rows.0[self.width as usize - 1], false,
                    rows.1[self.width as usize - 1], false
                );
                print!("{}", pixel_to_char(&pixel));
            }

            print!("\r\n");
        }

        // Handle case of odd height by adding another char to every column.
        if self.height % 2 == 1 {
            let last_row = &self.content[self.height as usize - 1];
            for real_x in 0..(self.width / 2) {
                // Extract the relavent pixel in the content matrix, and print it out.
                let pixel: Pixel = (
                    last_row[real_x as usize * 2], last_row[real_x as usize * 2 + 1],
                    false, false
                );
                print!("{}", pixel_to_char(&pixel));
            }

            // Handle odd width.
            if self.width % 2 == 1 {
                let pixel: Pixel = (
                    last_row[self.width as usize - 1], false,
                    false, false
                );
                print!("{}", pixel_to_char(&pixel));
            }

            print!("\r\n");
        }
    }
}