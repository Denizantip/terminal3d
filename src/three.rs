use crate::canvas;

// Simple 3d point wrapper.
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point {
    // Create a new point.
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }
}

pub struct Camera {
    // Location of the camera
    pub coordinates: Point,

    // In Radians.
    // When yaw: 0, pitch: 0, roll 0,
    // looking straight along the z-axis.
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,

    // viewport parameters.
    viewport_distance: f32,

    // In radians
    viewport_fov: f32,

    // Screen to render.
    pub screen: canvas::Screen
}

impl Camera {
    // Create a new camera.
    pub fn new(
        coordinates: Point, 
        yaw: f32, pitch: f32, roll: f32,
        viewport_distance: f32, viewport_fov: f32,
    ) -> Camera {
        Camera { 
            coordinates, 
            yaw, pitch, roll, 
            viewport_distance, viewport_fov, 
            screen: canvas::Screen::new()
        }
    }

    // Projects a 3d point to a canvas point.
    // See https://en.wikipedia.org/wiki/3D_projection#Mathematical_formula.
    // Returns an error if the point is outside the clipping plane.
    fn project(&self, point: &Point) -> Result<canvas::Point, String> {        
        // Compute trig values for camera angles.
        let (s_yaw, s_pitch, s_roll) = (self.yaw.sin(), self.pitch.sin(), self.roll.sin());
        let (c_yaw, c_pitch, c_roll) = (self.yaw.cos(), self.pitch.cos(), self.roll.cos());

        // Compute deltas between camera and point position.
        let delta_x = point.x - self.coordinates.x;
        let delta_y = point.y - self.coordinates.y;
        let delta_z = point.z - self.coordinates.z;

        // Find coordinates of point in camera space.
        let new_x = c_yaw * (-s_roll * delta_y + c_roll * delta_x) - s_yaw * delta_z;
        let new_y: f32 = -s_pitch * (
            c_yaw * delta_z + s_yaw * (-s_roll * delta_y + c_roll * delta_x)
        ) + c_pitch * (c_roll * delta_y + s_roll * delta_x);
        let new_z: f32 = c_pitch * (
            c_yaw * delta_z + s_yaw * (-s_roll * delta_y + c_roll * delta_x)
        ) + s_pitch * (c_roll * delta_y + s_roll * delta_x);

        // Naive near plane clipping.
        // Does not clip all out-of-bounds cases, only ones that cause visual glitches.
        if new_z < self.viewport_distance {
            return Err(format!(
                "Outside of clipping plane. Relative z to camera is {}, \
                greater than the near clipping plane distance, {}.", 
                new_z, self.viewport_distance
            ));
        }

        // Project onto viewport coordinates.
        let viewport_x = new_x * self.viewport_distance / new_z;
        let viewport_y = new_y * self.viewport_distance / new_z;

        // Compute viewport width and height based on screen width, height, and fov.
        let viewport_width = 2. * self.viewport_distance * (self.viewport_fov / 2.).tan();
        let viewport_height = (self.screen.height as f32 / self.screen.width as f32) * viewport_width;

        // Project to screen coordinates.
        let screen_x = (viewport_x / viewport_width + 0.5) * self.screen.width as f32;
        let screen_y = (1.0 - (viewport_y / viewport_height + 0.5)) * self.screen.height as f32;

        // Round.
        Ok(canvas::Point::new(screen_x.round() as i32, screen_y.round() as i32))
    }

    // Plot a 3d point.
    #[allow(dead_code)]
    pub fn write(&mut self, val: bool, point: &Point) {
        if let Ok(point_2d) = self.project(point) {
            self.screen.write(val, &point_2d)
        }
    }

    // Plot a 3d edge.
    pub fn edge(&mut self, start: &Point, end: &Point) {
        if let Ok(start_2d) = self.project(start) {
            if let Ok(end_2d) = self.project(end) {
                self.screen.line(&start_2d, &end_2d)
            }
        }
    }
}
