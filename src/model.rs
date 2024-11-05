use crate::three;
use std::*;

// Error for .obj parsing failures.
#[derive(Debug)]
struct ObjParseError;

impl ObjParseError {
    fn new() -> ObjParseError {ObjParseError}
}

impl fmt::Display for ObjParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing .obj file.")
    }
}

impl error::Error for ObjParseError {
    fn description(&self) -> &str {
        "Error parsing .obj file."
    }
}

// Simple 3d point wrapper.
pub struct Model {
    // Defined in model space.
    pub points: Vec<three::Point>,
    pub edges: Vec<(three::Point, three::Point)>,

    // Position of (0, 0, 0) in model space, in world space.
    pub position: three::Point
}

#[allow(dead_code)]
impl Model {
    // Creates a new model at a specified position.
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

    // Creates a model from a .obj file.
    pub fn new_obj(path: &str, position: three::Point) -> Result<Model, Box<dyn error::Error>> {

        // Read the file.
        let mut code = fs::read_to_string(path)?;

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
                            let x = x.parse::<f32>()?;
                            let y = y.parse::<f32>()?;
                            let z = z.parse::<f32>()?;
                            vertices.push(three::Point::new(x, y, z));
                        }
                        _ => { return Err(Box::from(ObjParseError::new())) }
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
                                let Ok(vertex_index) = vertex_index.parse::<usize>() else {
                                    return Err(Box::from(ObjParseError::new()))
                                };
                                let Some(vertex_index) = vertex_index.checked_sub(1) else {
                                    return Err(Box::from(ObjParseError::new()))
                                };

                                line.push(vertex_index);
                            }
                            _ => { return Err(Box::from(ObjParseError::new())) }
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
                                let Ok(vertex_index) = vertex_index.parse::<usize>() else {return Err(Box::from(ObjParseError::new()))};
                                let Some(vertex_index) = vertex_index.checked_sub(1) else {return Err(Box::from(ObjParseError::new()))};

                                face.push(vertex_index);
                            }
                            _ => { return Err(Box::from(ObjParseError::new())) }
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
                    *face.last().unwrap(), 
                    *face.first().unwrap()
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
            edges,
            position,
        })
    }

    pub fn model_to_world(&self, point: &three::Point) -> three::Point {
        three::Point{
            x: point.x + self.position.x,
            y: point.y + self.position.y,
            z: point.z + self.position.z
        }
    }

    // Returns the min and max bounds of the model in model space (rectangular prism).
    pub fn world_bounds(&self) -> (three::Point, three::Point) {
        if self.points.is_empty() && self.edges.is_empty() {
            return (
                three::Point::new(0., 0., 0.), 
                three::Point::new(0., 0., 0.)
            )
        }

        let mut min_bounds = self.points[0];
        let mut max_bounds = self.points[0];

        let points_including_edges = self.edges.iter()
            .flat_map(|tup| iter::once(&tup.0).chain(iter::once(&tup.1)))
            .chain(self.points.iter());

        for point in points_including_edges {
            min_bounds.x = f32::min(point.x, min_bounds.x);
            min_bounds.y = f32::min(point.y, min_bounds.y);
            min_bounds.z = f32::min(point.z, min_bounds.z);

            max_bounds.x = f32::max(point.x, max_bounds.x);
            max_bounds.y = f32::max(point.y, max_bounds.y);
            max_bounds.z = f32::max(point.z, max_bounds.z);
        }

        (min_bounds, max_bounds)
    }
}