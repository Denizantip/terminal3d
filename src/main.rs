use three::Point;

mod screen;
mod three;
mod model;

fn main() {
    let mut camera = three::Camera::new(
        three::Point::new(0., 0.1, -0.5), 
        0., -0.12, 0., 
        0.1, 1.7,
    );

    let cube_1 = model::Model::new_cube(0.1, Point::new(-0.2, 0., 0.));
    let cube_2 = model::Model::new_cube(0.1, Point::new(0.2, 0., 0.));

    let cube_3 = model::Model::new_cube(0.1, Point::new(-0.2, 0., 0.5));
    let cube_4 = model::Model::new_cube(0.1, Point::new(0.2, 0., 0.5));

    let cube_5 = model::Model::new_cube(0.1, Point::new(-0.2, 0., 1.));
    let cube_6 = model::Model::new_cube(0.1, Point::new(0.2, 0., 1.));

    let cube_7 = model::Model::new_cube(0.1, Point::new(-0.2, 0., 1.5));
    let cube_8 = model::Model::new_cube(0.1, Point::new(0.2, 0., 1.5));

    loop {
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

        camera.coordinates.z += 0.01;

        // Render.
        camera.screen.render();
        println!(
            "[ x: {:6.3}, y: {:6.3}, z: {:6.3} | yaw: {:6.3}, pitch: {:6.3}, roll: {:6.3} ]", 
            camera.coordinates.x, camera.coordinates.y, camera.coordinates.z,
            camera.yaw, camera.pitch, camera.roll
        );

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
