/// Contain data about how to draw a sprite on the screen
#[derive(Debug)]
pub struct ObjectOnScreen {
    /// the x coordinate, in pixel of where to draw the center of the sprite on screen
    pub x_pixel: f64,
    /// the y coordinate, in pixel of where to draw the center of the sprite on screen
    pub y_pixel: f64,
    /// the angle at which this character should be drawn
    pub angle: f64,
}

/// Represent a 2D camera
#[derive(Debug)]
pub struct Camera {
    scale: f64,
    x_unit: f64,
    y_unit: f64,
    x_pixel: f64,
    y_pixel: f64,
    x_pixel_screen_center: f64,
    y_pixel_screen_center: f64,
    angle: f64, // radians
}


impl Camera {
    pub fn new(scale: f64, (x_unit, y_unit): (f64, f64), angle: f64) -> Self {
        Self {
            scale,
            x_unit,
            y_unit,
            x_pixel: x_unit * scale,
            y_pixel: y_unit * scale,
            x_pixel_screen_center: 0.0,
            y_pixel_screen_center: 0.0,
            angle
        }
    }

    /// Compute where to draw something, based in its x and y coordinate (in unit) and it's angle (in radians)
    /// Return: x and y, in pixel, and the drawing angle, also in radians.
    pub fn compute_display_data(&self, (x_unit, y_unit): (f64, f64), angle: f64) -> ObjectOnScreen {
        //TODO: take into account the initial angle
        ObjectOnScreen {
            x_pixel: self.x_pixel + x_unit * self.scale + self.x_pixel_screen_center,
            y_pixel: self.y_pixel + y_unit * self.scale + self.y_pixel_screen_center,
            angle,
        }
    }

    pub fn set_screen_size(&mut self, (x_pixel, y_pixel): (f64, f64)) {
        self.x_pixel_screen_center = x_pixel/2.0;
        self.y_pixel_screen_center = y_pixel/2.0;
    }

}
