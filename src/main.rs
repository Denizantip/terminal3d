use std::f32;

use termion::input::TermRead;

mod screen;
mod three;
mod model;

fn main() {
    // Setup camera.
    let mut camera = three::Camera::new(
        three::Point::new(0., 0., -5.), 
        -0., -0., 0., 
        0.1, 1.7,
    );

    // Setup events.
    let mut events = termion::async_stdin().events();

    // Setup monkey model.
    let Ok(contents) = std::fs::read_to_string("./monkey.obj") else {todo!()};
    let mut points: Vec<three::Point> = Vec::new();
    let mut faces: Vec<(usize, usize, usize)> = Vec::new();
    for line in contents.split('\n') {
        let mut tokens = line.split(' ');

        match tokens.next() {
            Some("v") => {
                let Some(x_str) = tokens.next() else { todo!() };
                let Some(y_str) = tokens.next() else { todo!() };
                let Some(z_str) = tokens.next() else { todo!() };
    
                let Ok(x) = x_str.parse::<f32>() else { todo!() };
                let Ok(y) = y_str.parse::<f32>() else { todo!() };
                let Ok(z) = z_str.parse::<f32>() else { todo!() };
    
                points.push(three::Point::new(x, y, -z));
            }
            Some("f") => {
                let Some(p_1_str) = tokens.next() else { todo!() };
                let Some(p_2_str) = tokens.next() else { todo!() };
                let Some(p_3_str) = tokens.next() else { todo!() };
    
                let Some(p_1_index) = p_1_str.split('/').next() else { todo!() };
                let Some(p_2_index) = p_2_str.split('/').next() else { todo!() };
                let Some(p_3_index) = p_3_str.split('/').next() else { todo!() };
    
                let Ok(p_1_final) = p_1_index.parse::<usize>() else { todo!() };
                let Ok(p_2_final) = p_2_index.parse::<usize>() else { todo!() };
                let Ok(p_3_final) = p_3_index.parse::<usize>() else { todo!() };
    
                faces.push((p_1_final - 1, p_2_final - 1, p_3_final - 1));
            }
            _ => ()
        }
    }

    let new_faces = faces.into_iter().map(|(a, b, c)| (points[a], points[b], points[c]));
    let mut edges: Vec<(three::Point, three::Point)> = Vec::new();
    for face in new_faces {
        edges.push((face.0, face.1));
        edges.push((face.1, face.2));
        edges.push((face.2, face.0));
    }

    let monkey = model::Model::new(points, edges, three::Point::new(0., 0., 0.));

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
