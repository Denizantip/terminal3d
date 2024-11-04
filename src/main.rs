use std::*;

use termion::raw::IntoRawMode;
use termion::input::MouseTerminal;
use termion::input::TermRead;
use termion::event::*;
use time::Duration;

mod screen;
mod three;
mod model;

const VIEWPORT_FOV: f32 = 1.7;
const VIEWPORT_DISTANCE: f32 = 0.1;
const TARGET_DURATION_PER_FRAME: Duration = Duration::from_millis(16);
const INITIAL_DISTANCE_MULTIPLIER: f32 = 1.5;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("Please supply only one file path to visualize.");
    }
    if args.len() < 2 {
        panic!("Please supply a file path to visualize.")
    }

    let file_path = &args[1];

    // Put stdout in raw mode with mouse events enabled.
    let _raw_terminal = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    // Load model.
    let input_model = model::Model:: new_obj(
        file_path,
        three::Point::new(
            0., 0., 0.
        )
    ).unwrap();
    let bounds = input_model.world_bounds();
    let center = input_model.model_to_world(&three::Point::new(
        (bounds.0.x + bounds.1.x) / 2., 
        (bounds.0.y + bounds.1.y) / 2., 
        (bounds.0.z + bounds.1.z) / 2., 
    ));

    // Setup camera.
    let mut camera = three::Camera::new(
        center, 
        0., 0., 0., 
        VIEWPORT_DISTANCE, VIEWPORT_FOV,
    );

    // Setup viewer params (relative to model).
    let mut view_yaw: f32 = 0.0;
    let mut view_pitch: f32 = 0.0;
    let mut distance_to_model = (
        (bounds.0.x - bounds.1.x).powi(2) +
        (bounds.0.y - bounds.1.y).powi(2) +
        (bounds.0.z - bounds.1.z).powi(2)
    ).sqrt() * INITIAL_DISTANCE_MULTIPLIER;

    // Setup events.
    let mut events = termion::async_stdin().events();
    let mut mouse_speed: (f32, f32) = (0., 0.);
    let mut last_mouse_position = screen::Point::new(0, 0);

    // The actual, non-target time of each frame
    let mut duration_per_frame = TARGET_DURATION_PER_FRAME;

    // Start main loop.
    let mut running = true;
    while running {
        let start = time::Instant::now();

        // Take mouse input, and extract mouse speed.
        let mut event_count = 0;
        for event in &mut events {
            event_count += 1;
            match event {
                Ok(Event::Key(Key::Ctrl('c'))) => { running = false }
                Ok(Event::Key(Key::Char('+'))) => { distance_to_model *= 0.97}
                Ok(Event::Key(Key::Char('-'))) => { distance_to_model *= 1.03}
                Ok(Event::Mouse(mouse_event)) => match mouse_event {
                    MouseEvent::Press(_, x, y) => {
                        last_mouse_position.x = x as i32;
                        last_mouse_position.y = y as i32;
                    }

                    MouseEvent::Hold(x, y) => {
                        let delta_x = x as f32 - last_mouse_position.x as f32;
                        let delta_y = last_mouse_position.y as f32 - y as f32;
                        mouse_speed.0 = delta_x / camera.screen.width as f32 / duration_per_frame.as_secs_f32();
                        mouse_speed.1 = delta_y / camera.screen.width as f32 / duration_per_frame.as_secs_f32();
                        last_mouse_position.x = x as i32;
                        last_mouse_position.y = y as i32;
                    }
                    _ => {}
                }
                _ => {}
            }
        }
        if event_count == 0 {
            mouse_speed.0 = 0.;
            mouse_speed.1 = 0.;
        }

        // Update viewer params.
        view_yaw -= mouse_speed.0;
        view_pitch -= mouse_speed.1;

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
        
        // Add buffer time to hit 60 fps.
        if let Some(time) = TARGET_DURATION_PER_FRAME.checked_sub(start.elapsed()) { 
            thread::sleep(time);
        }

        // Print info.
        print!(
            "{}x: {:6.3}, y: {:6.3}, z: {:6.3} | heading: {:6.3}, pitch: {:6.3}, roll: {:6.3} | fps: {:3.0}", 
            termion::clear::CurrentLine,
            camera.coordinates.x, camera.coordinates.y, camera.coordinates.z,
            camera.yaw, camera.pitch, camera.roll,
            1. / duration_per_frame.as_secs_f32()
        );

        duration_per_frame = start.elapsed();
    }
}
