use crate::three;
use std::iter;

// Simple 3d point wrapper.
pub struct Model {
    // Defined in model space.
    pub points: Vec<three::Point>,
    pub edges: Vec<(three::Point, three::Point)>,

    // Position of (0, 0, 0) in model space, in world space.
    pub position: three::Point
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
    #[allow(dead_code)]
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

    // Creates a model from a .obj file.
    pub fn new_obj(path: &str, position: three::Point) -> Result<Model, &str> {

        // Read the file.
        let Ok(mut code) = std::fs::read_to_string(path) else 
            { return Err("Error reading path.") };

        // Start by pre-processing our code, to convert '\' chars followed by a newline, 
        // to the same line as the backslash, seperated by whitespace.
        code = code.replace("\\\n", " ");
        
        // Destination data.
        let mut vertices = Vec::<three::Point>::new();

        // These vectors contain indicies to the vertices they refer to.
        let mut lines = Vec::<Vec<usize>>::new();
        let mut faces = Vec::<Vec<usize>>::new();

        for line in code.split('\n') {
            // Extract tokens split by whitespace.
            let mut tokens = line
                .split_whitespace()
                .filter(|&line| !line.is_empty());

            // Identify the command.
            match tokens.next() {
                // Handle vertex.
                Some("v") => {
                    match (tokens.next(), tokens.next(), tokens.next(), tokens.next(), tokens.next()) {
                        (Some(x), Some(y), Some(z), _, None) => {
                            let Ok(x) = x.parse::<f32>() else { return Err("Invalid value for x.") };
                            let Ok(y) = y.parse::<f32>() else { return Err("Invalid value for y.") };
                            let Ok(z) = z.parse::<f32>() else { return Err("Invalid value for z.") };

                            vertices.push(three::Point::new(x, y, z));
                        }
                        _ => { return Err("Invalid pattern.") }
                    }
                }
                
                // Handle line.
                Some("l") => {
                    let mut line = Vec::<usize>::new();
                    for point in tokens {

                        // A line is made of point tokens, split by forward slashes.
                        // We only care about the first value.
                        let mut params = point.split('/');

                        // Get the vertext index, and push it to the line.
                        match (params.next(), params.next(), params.next()) {
                            (Some(vertex_index), _, None) => {
                                let Ok(vertex_index) = vertex_index.parse::<usize>() else {return Err("Invalid index.")};
                                let Some(vertex_index) = vertex_index.checked_sub(1) else {return Err("Invalid index.")};

                                line.push(vertex_index);
                            }
                            _ => { return Err("Invalid pattern.") }
                        }
                    }

                    lines.push(line);
                } 

                // Handle Face.
                Some("f") | Some("fo") => {
                    let mut face = Vec::<usize>::new();
                    for point in tokens {

                        // A line is made of point tokens, split by forward slashes.
                        // We only care about the first value.
                        let mut params = point.split('/');

                        // Get the vertext index, and push it to the line.
                        match (params.next(), params.next(), params.next(), params.next()) {
                            (Some(vertex_index), _, _, None) => {
                                let Ok(vertex_index) = vertex_index.parse::<usize>() else {return Err("Invalid index.")};
                                let Some(vertex_index) = vertex_index.checked_sub(1) else {return Err("Invalid index.")};

                                face.push(vertex_index);
                            }
                            _ => { return Err("Invalid pattern.") }
                        }
                    }

                    faces.push(face);
                }

                // Handle comments with no action.
                Some("#") => {}

                // Handle unsupported keywords with no action.
                _ => {}
            }
        }

        // Convert face and line lists to a list of tuples representing edges.
        let mut edges = Vec::<(usize, usize)>::new();
        for line in lines.iter() {
            if line.len() >= 2 {
                for start in 0..line.len() - 1 {
                    let end = start + 1;
                    edges.push((line[start], line[end]));
                }
            }
        }
        for face in faces.iter() {
            if face.len() >= 2 {
                for start in 0..face.len() - 1 {
                    let end = start + 1;
                    edges.push((face[start], face[end]));
                }

                // Handle the closing edge.
                edges.push((
                    face.last().unwrap().clone(), 
                    face.first().unwrap().clone()
                ));
            }   
        }

        // Remove duplicates for performance.
        edges.sort();
        edges.dedup();

        // Convert edges to actual points.
        let edges: Vec<(three::Point, three::Point)> = edges.into_iter().map(
            |(start_index, end_index)| 
                (vertices[start_index], vertices[end_index])
        ).collect();

        Ok(Model{
            points: vertices,
            edges: edges,
            position: position,
        })
    }

    pub fn model_to_world(&self, point: &three::Point) -> three::Point {
        three::Point{
            x: point.x + self.position.x,
            y: point.y + self.position.y,
            z: point.z + self.position.z
        }
    }

    // Returns the center of the bounding box of the model.
    pub fn center(&self) -> three::Point {
        let mut x_bounds: (f32, f32) = (self.points[0].x, self.points[0].x);
        let mut y_bounds: (f32, f32) = (self.points[0].y, self.points[0].y);
        let mut z_bounds: (f32, f32) = (self.points[0].z, self.points[0].z);

        for point in &self.points {
            x_bounds.0 = f32::min(point.x, x_bounds.0);
            x_bounds.1 = f32::max(point.x, x_bounds.1);
            y_bounds.0 = f32::min(point.y, y_bounds.0);
            y_bounds.1 = f32::max(point.y, y_bounds.1);
            z_bounds.0 = f32::min(point.z, z_bounds.0);
            z_bounds.1 = f32::max(point.z, z_bounds.1);
        }

        for point in self.edges.iter().flat_map(|tup| iter::once(tup.0).chain(iter::once(tup.1))) {
            x_bounds.0 = f32::min(point.x, x_bounds.0);
            x_bounds.1 = f32::max(point.x, x_bounds.1);
            y_bounds.0 = f32::min(point.y, y_bounds.0);
            y_bounds.1 = f32::max(point.y, y_bounds.1);
            z_bounds.0 = f32::min(point.z, z_bounds.0);
            z_bounds.1 = f32::max(point.z, z_bounds.1);
        }

        self.model_to_world(&three::Point::new(
            (x_bounds.0 + x_bounds.1) / 2., 
            (y_bounds.0 + y_bounds.1) / 2., 
            (z_bounds.0 + z_bounds.1) / 2.)
        )
    }

    // Returns the max radius of the obejct.
    pub fn max_radius(&self) -> f32 {
        let furthest_point = self.points.iter().max_by(
            |a, b| 
                (a.x.powi(2) + a.y.powi(2) + a.z.powi(2)).total_cmp(&(b.x.powi(2) + b.y.powi(2) + b.z.powi(2)))
        );

        let furthest_edge_point = self.edges.iter()
            .flat_map(|tup| iter::once(tup.0).chain(iter::once(tup.1)))
            .max_by(
                |a, b| 
                    (a.x.powi(2) + a.y.powi(2) + a.z.powi(2)).total_cmp(&(b.x.powi(2) + b.y.powi(2) + b.z.powi(2)))
            );

        match (furthest_point, furthest_edge_point) {
            (Some(point), None) => {
                (point.x.powi(2) + point.y.powi(2) + point.z.powi(2)).sqrt()
            }
            (None, Some(point)) => {
                (point.x.powi(2) + point.y.powi(2) + point.z.powi(2)).sqrt()
            }
            (Some(point_1), Some(point_2)) => {
                let dist_1 = (point_1.x.powi(2) + point_1.y.powi(2) + point_1.z.powi(2)).sqrt();
                let dist_2 = (point_2.x.powi(2) + point_2.y.powi(2) + point_2.z.powi(2)).sqrt();
                f32::max(dist_1, dist_2)
            }
            _ => {0.}
        }
    }

}