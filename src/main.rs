use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::event::{self, Event};
use std::io::stdout;
use std::process::exit;

mod canvas;
mod three;

fn main() {
    let mut camera = three::Camera::new(
        three::Point::new(0., 0., 0.), 
        0., 0., 0., 
        0.1, 1.7,
    );

    let stdout: std::io::Stdout = stdout();
    let _ = stdout.lock().into_raw_mode().unwrap();
    let user_input = termion::async_stdin();
    let mut user_events = user_input.events();

    loop {
        camera.screen.fit_to_terminal();
        camera.screen.clear();

        // Back points.
        let (
            back_1,
            back_2,
            back_3,
            back_4
        ) = (
            three::Point::new(-0.1, -0.1, 1.5),
            three::Point::new(-0.1, 0.1, 1.5),
            three::Point::new(0.1, 0.1, 1.5),
            three::Point::new(0.1, -0.1, 1.5)
        );

        // Front points.
        let (
            front_1,
            front_2,
            front_3,
            front_4
        ) = (
            three::Point::new(-0.1, -0.1, 1.),
            three::Point::new(-0.1, 0.1, 1.),
            three::Point::new(0.1, 0.1, 1.),
            three::Point::new(0.1, -0.1, 1.)
        );

        // Draw edges.
        camera.edge(&front_1, &front_2);
        camera.edge(&front_2, &front_3);
        camera.edge(&front_3, &front_4);
        camera.edge(&front_4, &front_1);

        camera.edge(&back_1, &back_2);
        camera.edge(&back_2, &back_3);
        camera.edge(&back_3, &back_4);
        camera.edge(&back_4, &back_1);

        camera.edge(&front_1, &back_1);
        camera.edge(&front_2, &back_2);
        camera.edge(&front_3, &back_3);
        camera.edge(&front_4, &back_4);

        for event in &mut user_events {
            match event {
                Ok(event) => match event {
                    Event::Key(key) => {
                        match key {
                            event::Key::Right => { camera.yaw += 0.05 }
                            event::Key::Left => { camera.yaw -= 0.05 }
                            event::Key::Up => {
                                camera.coordinates.z += camera.yaw.cos() * 0.05;
                                camera.coordinates.x += camera.yaw.sin() * 0.05;
                                camera.coordinates.y += camera.pitch.sin() * 0.05;
                            }
                            event::Key::Down => {
                                camera.coordinates.z -= camera.yaw.cos() * 0.05;
                                camera.coordinates.x -= camera.yaw.sin() * 0.05;
                                camera.coordinates.y -= camera.pitch.sin() * 0.05;
                            }
                            event::Key::Char(c) => {
                                if c == 'w' { camera.pitch += 0.05 }
                                if c == 's' { camera.pitch -= 0.05 }
                            }
                            event::Key::Ctrl(c) => { 
                                if c == 'c' { exit(0) }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                },
                Err(_) => {}
            }
        }

        // Render.
        camera.screen.render();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
