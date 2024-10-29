mod canvas;
mod three;

fn main() {
    let mut camera = three::Camera::new(
        three::Point::new(0., 0., 0.), 
        0., 0., 0., 
        0.1, 1.7,
    );

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

        camera.yaw += 0.01;

        // Render.
        camera.screen.render();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
