use std;
mod renderer;

fn main() {
    // Create new screen.
    let mut screen = renderer::Screen::new();
    
    let mut angle: f64 = 0.;
    loop {
        // Resize to terminal.
        screen.fit_to_terminal();

        let midpoint = renderer::Point::new(
            screen.width as i16 / 2,
            screen.height as i16 / 2,
        );
        let r = (std::cmp::min(midpoint.x, midpoint.y) / 2) as f64;
        
        let point_1: renderer::Point = renderer::Point::new(
            (midpoint.x as f64 + (r / 2.0) * angle.cos()) as i16, 
            (midpoint.y as f64 + (r / 2.0) * angle.sin()) as i16, 
        );

        let point_2: renderer::Point = renderer::Point::new(
            (midpoint.x as f64 + r * (0.3 * angle).cos()) as i16, 
            (midpoint.y as f64 + r * (0.3 * angle).sin()) as i16, 
        );

        let point_3: renderer::Point = renderer::Point::new(
            (midpoint.x as f64 + (r * 1.8) * (0.7 * angle).cos()) as i16, 
            (midpoint.y as f64 + (r * 1.8) * (0.7 * angle).sin()) as i16, 
        );

        screen.clear();
        screen.line(&point_1, &point_2);
        screen.line(&point_2, &point_3);
        screen.line(&point_1, &point_3);
        screen.render();

        std::thread::sleep(std::time::Duration::from_millis(16));
        angle += 0.1;
    }
}
