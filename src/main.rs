use termion::input::TermRead;
use std::f32::consts::PI;

mod screen;
mod three;
mod model;

fn main() {
    // Setup camera.
    let mut camera = three::Camera::new(
        three::Point::new(-2.5, 1., 8.), 
        -PI, 0., 0., 
        0.1, 1.7,
    );

    // Setup events.
    let mut events = termion::async_stdin().events();

    let monkey = model::Model::new_obj("./suzanne.obj", three::Point::new(
        0., 0., 0.
    )).unwrap();

    loop {
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
        camera.plot_model(&monkey);

        camera.coordinates.x += 0.01;
        // Render.
        camera.screen.render();
        print!(
            "[ x: {:6.3}, y: {:6.3}, z: {:6.3} | heading: {:6.3}, pitch: {:6.3}, roll: {:6.3} ]", 
            camera.coordinates.x, camera.coordinates.y, camera.coordinates.z,
            camera.yaw, camera.pitch, camera.roll
        );

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
