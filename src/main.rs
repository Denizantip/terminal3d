use std::*;
use process::exit;
use time::Duration;

use crossterm::{
    event,
    execute,
    terminal,
    style
};

mod screen;
mod three;
mod model;

const VIEWPORT_FOV: f32 = 1.7;
const VIEWPORT_DISTANCE: f32 = 0.1;
const TARGET_DURATION_PER_FRAME: Duration = Duration::from_millis(1000 / 60);
const MOUSE_SPEED_MULTIPLIER: f32 = 0.4;
const INITIAL_DISTANCE_MULTIPLIER: f32 = 1.5;

fn graceful_close() -> ! {
    execute!(
        io::stdout(),
        event::DisableMouseCapture,
    ).unwrap();
    terminal::disable_raw_mode().unwrap();
    exit(0)
}

fn error_close(msg: &dyn fmt::Display) -> ! {
    execute!(
        io::stdout(),
        style::Print(msg)
    ).unwrap();
    graceful_close()
}

fn main() {
    // Setup raw mode, mouse capture, and event stream.
    terminal::enable_raw_mode().unwrap();
    execute!(
        io::stdout(),
        event::EnableMouseCapture,
    ).unwrap();

    // Parse arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        error_close(&"Please supply only one file path to visualize.");
    }
    if args.len() < 2 {
        error_close(&"Please supply a file path to visualize.");
    }
    let file_path = &args[1];
    
    // Load model.
    let input_model = match model::Model::new_obj(
        file_path,
        three::Point::new(0., 0., 0.)
    ) {
        Ok(model) => model,
        Err(error) => {
            error_close(&error)
        }
    };

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
    let mut mouse_speed: (f32, f32) = (0., 0.);
    let mut last_mouse_position = screen::Point::new(0, 0);

    // The actual, non-target time of each frame
    let mut duration_per_frame = TARGET_DURATION_PER_FRAME;

    // Start main loop.
    loop {
        let start = time::Instant::now();
        let mut start_mouse_position = last_mouse_position;

        // Take mouse input, and extract mouse speed.
        let mut event_count = 0;
        while event::poll(Duration::from_secs(0)).unwrap() {
            match event::read() {
                Ok(event) => {
                    match event {
                        event::Event::Key(key_event) => {
                            let is_ctrl_c = key_event.modifiers == event::KeyModifiers::CONTROL
                                && key_event.code == event::KeyCode::Char('c');
                            if is_ctrl_c { graceful_close() }

                            else if key_event.code == event::KeyCode::Char('+') {
                                distance_to_model *= 0.97;
                            } else if key_event.code == event::KeyCode::Char('-') {
                                distance_to_model *= 1.03;
                            }
                        }
                        event::Event::Mouse(mouse_event) => {
                            let (x, y) = (mouse_event.column, mouse_event.row);
                            match mouse_event.kind {
                                event::MouseEventKind::Up(_) => {}
                                event::MouseEventKind::Down(_) => {
                                    last_mouse_position.x = x as i32;
                                    last_mouse_position.y = y as i32;
                                    start_mouse_position = last_mouse_position;
                                    event_count += 1;
                                }
                                event::MouseEventKind::Drag(_) => {
                                    let delta_x = x as f32 - start_mouse_position.x as f32;
                                    let delta_y = start_mouse_position.y as f32 - y as f32;
                                    mouse_speed.0 = delta_x / camera.screen.width as f32 / duration_per_frame.as_secs_f32() * MOUSE_SPEED_MULTIPLIER;
                                    mouse_speed.1 = delta_y / camera.screen.width as f32 / duration_per_frame.as_secs_f32() * MOUSE_SPEED_MULTIPLIER;
                                    last_mouse_position.x = x as i32;
                                    last_mouse_position.y = y as i32;
                                    event_count += 1;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
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
        let camera_position_msg = format!(
            "x: {:6.3}, y: {:6.3}, z: {:6.3}", 
            camera.coordinates.x, camera.coordinates.y, camera.coordinates.z
        );

        let camera_angle_msg = format!(
            "heading: {:6.3}, pitch: {:6.3}, roll: {:6.3}", 
            camera.yaw, camera.pitch, camera.roll
        );

        let fps_msg = format!(
            "fps: {:3.0}", 1. / duration_per_frame.as_secs_f32()
        );

        let msgs = (
            format!("{} | {} | {}", camera_position_msg, camera_angle_msg, fps_msg),
            format!("{} | {}", camera_position_msg, camera_angle_msg),
            format!("{}", camera_position_msg),
        );

        let final_msg = match terminal::size().unwrap().0 as usize {
            width if width > msgs.0.len() => { msgs.0 }
            width if width > msgs.1.len() => { msgs.1 }
            width if width > msgs.2.len() => { msgs.2 }
            _ => { "".to_string() }
        };

        execute!(
            io::stdout(),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print(final_msg),
        ).unwrap();

        duration_per_frame = start.elapsed();
    }
}
