use std::*;
use process::exit;
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
const PAN_MULTIPLIER: f32 = 0.1;
const HELP_MSG: &str = "\
\x1b[1mt3d\x1b[0m: Visualize .obj files in the terminal!

\x1b[1mUsage\x1b[0m:
    \"t3d <filepath.obj>\": Interactively view the provided .obj file.
    \"t3d --h\", \"t3d --help\", \"t3d -h\", \"t3d -help\", \"t3d\": Help and info.

\x1b[1mControls\x1b[0m:
    Scroll down to zoom out, scroll up to zoom in.
    Click and drag the mouse to rotate around the model.
    Click and drag the mouse while holding [shift] to pan.

    Press [b] to toggle block mode. 
    Press [p] to toggle vertices mode. 
";

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
    // Parse arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 { error_close(&"Please supply only one file path to visualize.") }
    if args.len() < 1 { error_close(&"Error parsing arguments.") }
    
    let help_mode = args.len() == 1 || 
        ["-h", "-help", "--h", "--help"].map(String::from).contains(&args[1]);

    if help_mode {
        execute!(
            io::stdout(),
            style::Print(HELP_MSG)
        ).unwrap();
        graceful_close();
    }

    terminal::enable_raw_mode().unwrap();
    execute!(
        io::stdout(),
        cursor::Hide,
        event::EnableMouseCapture,
    ).unwrap();

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
    let mut center = input_model.model_to_world(&three::Point::new(
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
    let mut pan_mode = false;

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
                    event::Event::Key(key_event) => {
                        let is_ctrl_c = key_event.modifiers == event::KeyModifiers::CONTROL
                            && key_event.code == event::KeyCode::Char('c');

                        if is_ctrl_c { graceful_close() }
                        if key_event.code == event::KeyCode::Char('p') { points_mode = !points_mode }
                        if key_event.code == event::KeyCode::Char('b') { braile_mode = !braile_mode }
                    }

                    // Mouse controls.
                    event::Event::Mouse(mouse_event) => {
                        let (x, y) = (mouse_event.column, mouse_event.row);
                        match mouse_event.kind {

                            // If the mouse has been pressed, record this position.
                            event::MouseEventKind::Down(_) => {
                                pan_mode = mouse_event.modifiers == event::KeyModifiers::SHIFT;
                                last_mouse_position.x = x as i32;
                                last_mouse_position.y = y as i32;
                                start_mouse_position = last_mouse_position;
                                event_count += 1;
                            }

                            // If the mouse is dragged, update drag speeds.
                            event::MouseEventKind::Drag(_) => {
                                pan_mode = mouse_event.modifiers == event::KeyModifiers::SHIFT;
                                let delta_x = x as f32 - start_mouse_position.x as f32;
                                let delta_y = start_mouse_position.y as f32 - y as f32;
                                mouse_speed.0 = delta_x / camera.screen.width as f32 * MOUSE_SPEED_MULTIPLIER;
                                mouse_speed.1 = delta_y / camera.screen.width as f32 * MOUSE_SPEED_MULTIPLIER;
                                last_mouse_position.x = x as i32;
                                last_mouse_position.y = y as i32;
                                event_count += 1;
                            }

                            event::MouseEventKind::ScrollDown => {
                                // Zoom out.
                                distance_to_model += diagonal * SCROLL_MULTIPLER;
                            }

                            event::MouseEventKind::ScrollUp => {
                                // Zoom in.
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

        // If no event happened, reset the mouse.
        if event_count == 0 { 
            mouse_speed = (0., 0.);
            pan_mode = false;
         }

        // Update viewer params.
        if pan_mode {

            // Handle horizontal pan.
            center.x -= mouse_speed.0 * camera.yaw.cos() * diagonal * PAN_MULTIPLIER;
            center.z += mouse_speed.0 * camera.yaw.sin() * diagonal * PAN_MULTIPLIER;

            // Handle vertical pan.
            center.y -= mouse_speed.1 * camera.pitch.cos() * diagonal * PAN_MULTIPLIER;
            center.x += mouse_speed.1 * camera.yaw.sin() * camera.pitch.sin() * diagonal * PAN_MULTIPLIER;
            center.z += mouse_speed.1 * camera.yaw.cos() * camera.pitch.sin() * diagonal * PAN_MULTIPLIER;
        } else {
            view_yaw -= mouse_speed.0;
            view_pitch -= mouse_speed.1;
        }

        // Update camera position.
        camera.coordinates.z = -view_yaw.cos() * view_pitch.cos() * distance_to_model + center.z;
        camera.coordinates.x = view_yaw.sin() * view_pitch.cos() * distance_to_model + center.x;
        camera.coordinates.y = view_pitch.sin() * distance_to_model + center.y;
        camera.yaw = -view_yaw;
        camera.pitch = -view_pitch;

        // Render.
        if braile_mode { camera.screen.fit_to_terminal::<screen::BrailePixel>()  }
        else { camera.screen.fit_to_terminal::<screen::BlockPixel>() }

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
        let points_mode_msg = format!(
            "rendering: {}", 
            if points_mode {"vertices"} else {"edges"}
        );

        let braile_mode_msg = format!(
            "display mode: {}", 
            if braile_mode {"braile"} else {"blocks"}
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
            format!("{} | {} | {} | {}", points_mode_msg, braile_mode_msg, resolution_msg, fps_msg),
            format!("{} | {} | {}", points_mode_msg, braile_mode_msg, resolution_msg),
            format!("{} | {}", points_mode_msg, braile_mode_msg),
            points_mode_msg.to_string(),
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
