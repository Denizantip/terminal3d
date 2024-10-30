use crate::{model, screen};

// Simple 3d point wrapper.
#[derive(Copy, Clone)]
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
    // Operations applied in order: yaw, pitch, roll,
    // Starting from z+ direction.
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,

    // viewport parameters.
    pub viewport_distance: f32,

    // In radians
    pub viewport_fov: f32,

    // Screen to render.
    pub screen: screen::Screen
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
            screen: screen::Screen::new()
        }
    }

    fn world_to_camera(&self, point: &Point) -> Point {
        // Compute trig values for camera angles.
        let (s_yaw, s_pitch, s_roll) = (self.yaw.sin(), self.pitch.sin(), self.roll.sin());
        let (c_yaw, c_pitch, c_roll) = (self.yaw.cos(), self.pitch.cos(), self.roll.cos());

        // Compute deltas between camera and point position.
        let delta_x = point.x - self.coordinates.x;
        let delta_y = point.y - self.coordinates.y;
        let delta_z = point.z - self.coordinates.z;

        // Undo yaw.
        let unyawed_x = delta_x * c_yaw - delta_z * s_yaw;
        let unyawed_y = delta_y;
        let unyawed_z = delta_x * s_yaw + delta_z * c_yaw;

        // Undo pitch.
        let unpitched_x = unyawed_x;
        let unpitched_y = unyawed_y * c_pitch - unyawed_z * s_pitch;
        let unpitched_z = unyawed_y * s_pitch + unyawed_z * c_pitch;

        // Undo roll.
        let unrolled_x = unpitched_x * c_roll - unpitched_y * s_roll;
        let unrolled_y = unpitched_x * s_roll + unpitched_y * c_roll;
        let unrolled_z = unpitched_z;

        Point::new(unrolled_x, unrolled_y, unrolled_z)
    }

    fn camera_to_screen(&self, point: &Point) -> screen::Point {
        // Project onto viewport coordinates.
        let viewport_x = point.x * self.viewport_distance / point.z;
        let viewport_y = point.y * self.viewport_distance / point.z;

        // Compute viewport width and height based on screen width, height, and fov.
        let viewport_width = 2. * self.viewport_distance * (self.viewport_fov / 2.).tan();
        let viewport_height = (self.screen.height as f32 / self.screen.width as f32) * viewport_width;

        // Project to screen coordinates.
        let screen_x = (viewport_x / viewport_width + 0.5) * self.screen.width as f32;
        let screen_y = (1.0 - (viewport_y / viewport_height + 0.5)) * self.screen.height as f32;

        // Round.
        screen::Point::new(screen_x.round() as i32, screen_y.round() as i32)
    }

    pub fn plot_model(&mut self, model: &model::Model) {
        for point in model.points.iter() {
            self.write(true, &model.model_to_world(point));
        }

        for edge in model.edges.iter() {
            self.edge( 
                &model.model_to_world(&edge.0),
                &model.model_to_world(&edge.1)
            );
        }
    }

    // Plot a 3d point.
    pub fn write(&mut self, val: bool, point: &Point) {
        let camera_point = self.world_to_camera(point);
        if camera_point.z >= self.viewport_distance {
            self.screen.write(val, &self.camera_to_screen(&camera_point));
        }
    }

    // Plot a 3d edge.
    pub fn edge(&mut self, start: &Point, end: &Point) {

        // Compute points in camera space, and find if we need to clip.
        let camera_start = self.world_to_camera(start);
        let camera_end = self.world_to_camera(end);
        let clip_start = camera_start.z < self.viewport_distance;
        let clip_end = camera_end.z < self.viewport_distance;

        // If we need to clip both points, don't plot.
        if clip_start && clip_end { return }

        // If we don't need to clip either point, plot a line.
        if !clip_start && !clip_end {
            self.screen.line(
                &self.camera_to_screen(&camera_start), 
                &self.camera_to_screen(&camera_end)
            );
            return
        }

        // Otherwise identify the clipped and unclipped point.
        let (clipped, unclipped) = 
            if clip_start { (camera_start, camera_end) } else { (camera_end, camera_start) };

        // Clip the clipped point.
        let distance_behind_viewport = self.viewport_distance - clipped.z;
        let (delta_x, delta_y, delta_z) = (
            unclipped.x - clipped.x,
            unclipped.y - clipped.y,
            unclipped.z - clipped.z
        );
        let lambda = distance_behind_viewport / delta_z;
        let new_clipped = Point::new(
            lambda * delta_x + clipped.x, 
            lambda * delta_y + clipped.y, 
            self.viewport_distance
        );

        // Plot.
        self.screen.line(
            &self.camera_to_screen(&new_clipped), 
            &self.camera_to_screen(&unclipped)
        )    
    }
}
