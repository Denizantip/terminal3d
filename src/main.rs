use std::*;
use process::exit;
use screen::{BlockPixel, BrailePixel};
use time::Duration;

use crossterm::{
    event,
    execute,
    terminal,
    style,
    cursor
};

mod screen;
mod three;
mod model;

// Config.
const VIEWPORT_FOV: f32 = 1.7;
const VIEWPORT_DISTANCE: f32 = 0.1;
const TARGET_DURATION_PER_FRAME: Duration = Duration::from_millis(1000 / 60);
const MOUSE_SPEED_MULTIPLIER: f32 = 30.;
const INITIAL_DISTANCE_MULTIPLIER: f32 = 1.5;
const SCROLL_MULTIPLER: f32 = 0.03;

// Disables raw mode and mouse capture, and shows the cursor.
fn graceful_close() -> ! {
    execute!(
        io::stdout(),
        cursor::Show,
        event::DisableMouseCapture,
    ).unwrap();
    terminal::disable_raw_mode().unwrap();
    exit(0)
}

// Closes with the provided error message.
fn error_close(msg: &dyn fmt::Display) -> ! {
    execute!(
        io::stderr(),
        style::Print(msg)
    ).unwrap();
    graceful_close()
}

fn main() {
    terminal::enable_raw_mode().unwrap();
    execute!(
        io::stdout(),
        cursor::Hide,
        event::EnableMouseCapture,
    ).unwrap();

    // Parse arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 { error_close(&"Please supply only one file path to visualize.") }
    if args.len() < 2 { error_close(&"Please supply a file path to visualize.") }
    let file_path = &args[1];
    
    // Load model.
    let input_model = match model::Model::new_obj(
        file_path,
        three::Point::new(0., 0., 0.)
    ) {
        Ok(model) => model,
        Err(error) => error_close(&error)
    };

    // Get dimensions.
    let bounds = input_model.world_bounds();
    let center = input_model.model_to_world(&three::Point::new(
        (bounds.0.x + bounds.1.x) / 2., 
        (bounds.0.y + bounds.1.y) / 2., 
        (bounds.0.z + bounds.1.z) / 2., 
    ));
    let diagonal = (
        (bounds.0.x - bounds.1.x).powi(2) +
        (bounds.0.y - bounds.1.y).powi(2) +
        (bounds.0.z - bounds.1.z).powi(2)
    ).sqrt();

    // Setup camera.
    let mut camera = three::Camera::new(
        center, 
        0., 0., 0., 
        VIEWPORT_DISTANCE, VIEWPORT_FOV,
    );

    // Setup viewer params (relative to model).
    let mut view_yaw: f32 = 0.0;
    let mut view_pitch: f32 = 0.0;
    let mut distance_to_model = diagonal * INITIAL_DISTANCE_MULTIPLIER;

    // Render modes.
    let mut points_mode = false;
    let mut braile_mode = true;

    // Setup events.
    let mut mouse_speed: (f32, f32) = (0., 0.);
    let mut last_mouse_position = screen::Point::new(0, 0);

    // Start main loop.
    loop {
        let start = time::Instant::now();
        let mut start_mouse_position = last_mouse_position;

        // Look through the queue while there is an available event.
        let mut event_count = 0;
        while event::poll(Duration::from_secs(0)).unwrap() {
            if let Ok(event) = event::read() {
                match event {

                    // Handle ctrl+c explictly (raw mode disables capture).
                    event::Event::Key(key_event) => {
                        let is_ctrl_c = key_event.modifiers == event::KeyModifiers::CONTROL
                            && key_event.code == event::KeyCode::Char('c');
                        if is_ctrl_c { graceful_close() }
                        
                        // Toggle modes.
                        if key_event.code == event::KeyCode::Char('p') { points_mode = !points_mode }
                        if key_event.code == event::KeyCode::Char('b') { braile_mode = !braile_mode }
                    }

                    // Mouse controls.
                    event::Event::Mouse(mouse_event) => {
                        let (x, y) = (mouse_event.column, mouse_event.row);
                        match mouse_event.kind {
                            // If the mouse has been pressed, record this position.
                            event::MouseEventKind::Down(_) => {
                                last_mouse_position.x = x as i32;
                                last_mouse_position.y = y as i32;
                                start_mouse_position = last_mouse_position;
                                event_count += 1;
                            }

                            // If the mouse is dragged, update drag speeds.
                            event::MouseEventKind::Drag(_) => {
                                let delta_x = x as f32 - start_mouse_position.x as f32;
                                let delta_y = start_mouse_position.y as f32 - y as f32;
                                mouse_speed.0 = delta_x / camera.screen.width as f32 * MOUSE_SPEED_MULTIPLIER;
                                mouse_speed.1 = delta_y / camera.screen.width as f32 * MOUSE_SPEED_MULTIPLIER;
                                last_mouse_position.x = x as i32;
                                last_mouse_position.y = y as i32;
                                event_count += 1;
                            }

                            // Zoom out.
                            event::MouseEventKind::ScrollDown => {
                                distance_to_model += diagonal * SCROLL_MULTIPLER;
                            }

                            // Zoom in.
                            event::MouseEventKind::ScrollUp => {
                                distance_to_model -= diagonal * SCROLL_MULTIPLER;
                                distance_to_model = distance_to_model.max(0.);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }

        // If no event happened, reset the mouse speed.
        if event_count == 0 { mouse_speed = (0., 0.) }

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
        if braile_mode { camera.screen.fit_to_terminal::<BrailePixel>()  }
        else { camera.screen.fit_to_terminal::<BlockPixel>() }

        camera.screen.clear();

        if points_mode { camera.plot_model_points(&input_model) }
        else { camera.plot_model_edges(&input_model) }

        if braile_mode { camera.screen.render::<screen::BrailePixel>() }
        else { camera.screen.render::<screen::BlockPixel>() }
        
        // Add buffer time to hit 60 fps.
        if let Some(time) = TARGET_DURATION_PER_FRAME.checked_sub(start.elapsed()) { 
            thread::sleep(time);
        }

        // Create info message variants for responsive resizing.
        let camera_position_msg = format!(
            "x: {:6.3}, y: {:6.3}, z: {:6.3}", 
            camera.coordinates.x, camera.coordinates.y, camera.coordinates.z
        );

        let camera_angle_msg = format!(
            "heading: {:6.3}, pitch: {:6.3}, roll: {:6.3}", 
            camera.yaw, camera.pitch, camera.roll
        );

        let fps_msg = format!(
            "fps: {:3.0}", 1. / start.elapsed().as_secs_f32()
        );

        let resolution_msg = format!(
            "resolution: {} x {}",
            camera.screen.width,
            camera.screen.height,
        );

        let msgs = (
            format!("{} | {} | {} | {}", camera_position_msg, camera_angle_msg, fps_msg, resolution_msg),
            format!("{} | {} | {}", camera_position_msg, camera_angle_msg, fps_msg),
            format!("{} | {}", camera_position_msg, camera_angle_msg),
            camera_position_msg.to_string(),
        );

        let final_msg = match terminal::size().unwrap().0 as usize {
            width if width > msgs.0.len() => { msgs.0 }
            width if width > msgs.1.len() => { msgs.1 }
            width if width > msgs.2.len() => { msgs.2 }
            width if width > msgs.3.len() => { msgs.3 }
            _ => { "".to_string() }
        };

        execute!(
            io::stdout(),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print(final_msg),
        ).unwrap();
    }
}
