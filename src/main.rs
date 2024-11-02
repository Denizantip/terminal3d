use std::thread;

use termion::raw::IntoRawMode;
use termion::input::MouseTerminal;
use termion::input::TermRead;
use termion::event::*;

mod screen;
mod three;
mod model;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        panic!("Please supply only one file path to visualize.");
    }
    if args.len() < 2 {
        panic!("Please supply a file path to visualize.")
    }

    let file_path = &args[1];

    // Put stdout in raw mode with mouse events enabled.
    let _raw_terminal = MouseTerminal::from(std::io::stdout().into_raw_mode().unwrap());

    // Load model.
    let input_model = model::Model:: new_obj(
        &file_path,
        three::Point::new(
            0., 0., 0.
        )
    ).unwrap();
    let center = input_model.center();

    // Setup camera.
    let mut camera = three::Camera::new(
        center, 
        0., 0., 0., 
        0.1, 1.7,
    );

    // Setup viewer params (relative to model).
    let mut view_yaw: f32 = 0.0;
    let mut view_pitch: f32 = 0.0;
    let mut distance_to_model: f32 = input_model.max_radius() * 2.;

    // Setup events.
    let mut events = termion::async_stdin().events();
    let mut mouse_speed = screen::Point::new(0, 0);
    let mut last_mouse_position = screen::Point::new(0, 0);

    // Start main loop.
    let mut running = true;
    while running {
        let start = std::time::Instant::now();
        // Take mouse input, and extract mouse speed.
        let mut event_count = 0;
        for event in (&mut events).flatten() {
            event_count += 1;
            match event {
                Event::Key(Key::Ctrl('c')) => { running = false }
                Event::Key(Key::Char('+')) => { distance_to_model *= 0.97}
                Event::Key(Key::Char('-')) => { distance_to_model *= 1.03}
                Event::Mouse(mouse_event) => match mouse_event {
                    MouseEvent::Press(_, x, y) => {
                        last_mouse_position.x = x as i32;
                        last_mouse_position.y = y as i32;
                    }
                    MouseEvent::Hold(x, y) => {
                        mouse_speed.x = ((x as f32 - last_mouse_position.x as f32) / camera.screen.width as f32 * 2500.) as i32;
                        mouse_speed.y = ((last_mouse_position.y as f32 - y as f32) / camera.screen.width as f32 * 2500.) as i32;
                        last_mouse_position.x = x as i32;
                        last_mouse_position.y = y as i32;
                    }
                    _ => {
                        mouse_speed.x = 0;
                        mouse_speed.y = 0;
                    }
                }
                _ => {
                    mouse_speed.x = 0;
                    mouse_speed.y = 0;
                }
            }
        }
        if event_count == 0 {
            mouse_speed.x = 0;
            mouse_speed.y = 0;
        }

        // Update viewer params.
        view_yaw -= mouse_speed.x as f32 / 100.;
        view_pitch -= mouse_speed.y as f32 / 100.;

        // Update camera position.
        camera.coordinates.z = -view_yaw.cos() * view_pitch.cos() * distance_to_model + center.z;
        camera.coordinates.x = view_yaw.sin() * view_pitch.cos() * distance_to_model + center.x;
        camera.coordinates.y = view_pitch.sin() * distance_to_model + center.y;
        camera.yaw = -view_yaw;
        camera.pitch = -view_pitch;

        // Render.
        camera.screen.fit_to_terminal();
        camera.screen.clear();
        camera.plot_model_edges(&input_model);
        camera.screen.render();
        
        match std::time::Duration::from_millis(16).checked_sub(start.elapsed()) {
            Some(time) => { thread::sleep(time); }
            None => {}
        }

        // Print info.
        print!(
            "{}x: {:6.3}, y: {:6.3}, z: {:6.3} | heading: {:6.3}, pitch: {:6.3}, roll: {:6.3} | fps: {:3.0}", 
            termion::clear::CurrentLine,
            camera.coordinates.x, camera.coordinates.y, camera.coordinates.z,
            camera.yaw, camera.pitch, camera.roll,
            1. / start.elapsed().as_secs_f32()
        );
    }
}
