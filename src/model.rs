use crate::three;

// Simple 3d point wrapper.
pub struct Model {
    // Defined in model space.
    points: Vec<three::Point>,
    edges: Vec<(three::Point, three::Point)>,

    // Position of (0, 0, 0) in model space, in world space.
    position: three::Point
}

impl Model {
    // Creates a new model at a specified position.
    #[allow(dead_code)]
    pub fn new(
        points: Vec<three::Point>,
        edges: Vec<(three::Point, three::Point)>,
        position: three::Point
    ) -> Model {
        Model{
            points,
            position,
            edges,
        }
    }

    // Creates a new cube, centered at the specified position, with the specified side-length.
    pub fn new_cube(
        side_length: f32,
        position: three::Point
    ) -> Model {
        let front = (
            three::Point::new(-side_length/2., -side_length/2., side_length/2.),
            three::Point::new(-side_length/2., side_length/2., side_length/2.),
            three::Point::new(side_length/2., side_length/2., side_length/2.),
            three::Point::new(side_length/2., -side_length/2., side_length/2.),
        );

        let rear = (
            three::Point::new(-side_length/2., -side_length/2., -side_length/2.),
            three::Point::new(-side_length/2., side_length/2., -side_length/2.),
            three::Point::new(side_length/2., side_length/2., -side_length/2.),
            three::Point::new(side_length/2., -side_length/2., -side_length/2.),
        );

        Model{
            points: Vec::new(),
            edges: vec![
                (front.0, front.1),
                (front.1, front.2),
                (front.2, front.3),
                (front.3, front.0),

                (rear.0, rear.1),
                (rear.1, rear.2),
                (rear.2, rear.3),
                (rear.3, rear.0),

                (rear.0, front.0),
                (rear.1, front.1),
                (rear.2, front.2),
                (rear.3, front.3),
            ],
            position,
        }
    }

    pub fn model_to_world(&self, point: &three::Point) -> three::Point {
        three::Point{
            x: point.x + self.position.x,
            y: point.y + self.position.y,
            z: point.z + self.position.z
        }
    }

    pub fn render(&self, camera: &mut three::Camera) {
        for point in self.points.iter() {
            camera.write(true, &self.model_to_world(point));
        }

        for edge in self.edges.iter() {
            camera.edge( 
                &self.model_to_world(&edge.0),
                &self.model_to_world(&edge.1)
            );
        }
    }
}