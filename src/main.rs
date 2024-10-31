use termion::input::TermRead;
use std::f32::consts::PI;

mod screen;
mod three;
mod model;

fn main() {
    // Setup camera.
    let mut camera = three::Camera::new(
        three::Point::new(0., 0., 0.), 
        -PI, 0., 0., 
        0.1, 1.7,
    );

    // Setup events.
    let mut events = termion::async_stdin().events();

    let cow = model::Model::new_obj("./cow.obj", three::Point::new(
        4., 0., -8.
    )).unwrap();

    let monkey = model::Model::new_obj("./suzanne.obj", three::Point::new(
        12., -2., -12.
    )).unwrap();

    loop {
        let start = std::time::Instant::now();

        // Take input.
        for event in (&mut events).flatten() {
            if let termion::event::Event::Key(key) = event {
                match key {
                    termion::event::Key::Right => { camera.yaw += 0.03 }
                    termion::event::Key::Left => { camera.yaw -= 0.03 }
                    termion::event::Key::Up => { camera.pitch += 0.03 }
                    termion::event::Key::Down => { camera.pitch -= 0.03 }
                    termion::event::Key::Char('d') => { camera.roll += 0.03 }
                    termion::event::Key::Char('a') => { camera.roll -= 0.03 }
                    termion::event::Key::Ctrl('c') => { 
                        camera.screen.reset_stdout();
                        std::process::exit(0)
                    }
                    _ => {}
                }
            }
        }

        camera.screen.fit_to_terminal();
        camera.screen.clear();
        camera.plot_model_points(&cow);
        camera.plot_model_edges(&monkey);

        camera.coordinates.x += 0.01;

        // Render.
        camera.screen.render();
        
        print!(
            "{}x: {:6.3}, y: {:6.3}, z: {:6.3} | heading: {:6.3}, pitch: {:6.3}, roll: {:6.3} | fps: {:3.0}", 
            termion::clear::CurrentLine,
            camera.coordinates.x, camera.coordinates.y, camera.coordinates.z,
            camera.yaw, camera.pitch, camera.roll,
            1. / start.elapsed().as_secs_f32()
        );
    }
}
