use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod screen;
mod three;
mod model;

fn main() {
    let mut camera = three::Camera::new(
        three::Point::new(0., 0., 0.), 
        0., -0.12, 0., 
        0.1, 1.7,
    );

    let Ok(mut _stdout) = std::io::stdout().into_raw_mode() else {todo!()};
    let mut events = termion::async_stdin().events();

    let cube_1 = model::Model::new_cube(0.1, three::Point::new(-0.2, 0., -0.75));
    let cube_2 = model::Model::new_cube(0.1, three::Point::new(0.2, 0., -0.75));

    let cube_3 = model::Model::new_cube(0.1, three::Point::new(-0.2, 0., -0.25));
    let cube_4 = model::Model::new_cube(0.1, three::Point::new(0.2, 0., -0.25));

    let cube_5 = model::Model::new_cube(0.1, three::Point::new(-0.2, 0., 0.25));
    let cube_6 = model::Model::new_cube(0.1, three::Point::new(0.2, 0., 0.25));

    let cube_7 = model::Model::new_cube(0.1, three::Point::new(-0.2, 0., 0.75));
    let cube_8 = model::Model::new_cube(0.1, three::Point::new(0.2, 0., 0.75));

    let bound_cube = model::Model::new_cube(1., three::Point::new(0., 0., 0.));

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
                    termion::event::Key::Ctrl('c') => { std::process::exit(0) }
                    _ => {}
                }
            }
        }

        camera.screen.fit_to_terminal();
        camera.screen.clear();
        cube_1.render(&mut camera);
        cube_2.render(&mut camera);
        cube_3.render(&mut camera);
        cube_4.render(&mut camera);
        cube_5.render(&mut camera);
        cube_6.render(&mut camera);
        cube_7.render(&mut camera);
        cube_8.render(&mut camera);
        bound_cube.render(&mut camera);

        // camera.coordinates.z += 0.01;

        // Render.
        camera.screen.render();
        print!(
            "[ x: {:6.3}, y: {:6.3}, z: {:6.3} | yaw: {:6.3}, pitch: {:6.3}, roll: {:6.3} ]", 
            camera.coordinates.x, camera.coordinates.y, camera.coordinates.z,
            camera.yaw, camera.pitch, camera.roll
        );


        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
