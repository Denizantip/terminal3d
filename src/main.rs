use std;
mod renderer;

fn main() {
    // Extract width and height
    let (width, height) = termion::terminal_size().unwrap();
    let dimensions = renderer::Point::new(
        width as usize * 2 - 1,
        height as usize * 2 - 1
    );

    // Create new screen.
    let mut screen = renderer::Screen::new(dimensions);
    let midpoint = renderer::Point::new(
        screen.dimensions.x / 2,
        screen.dimensions.y / 2,
    );

    let r = (std::cmp::min(midpoint.x, midpoint.y) / 2) as f64;
    let mut angle: f64 = 0.;

    loop {
        let point: renderer::Point = renderer::Point::new(
            (midpoint.x as f64 + r * angle.cos()) as usize, 
            (midpoint.y as f64 + r * angle.sin()) as usize, 
        );

        screen.clear();
        screen.write(true, &point);
        screen.render();

        std::thread::sleep(std::time::Duration::from_millis(16));
        angle += 0.1;
    }
}
