use macroquad::prelude::*;

pub struct Camera {
    pub corner: (f32, f32),
    pub res: (usize, usize),
    pub scale: f32,
    pub grab_loc: Option<(f32, f32)>,
}

impl Camera {
    pub fn new(res: (usize, usize), corner: (f32, f32)) -> Camera {
        Camera {
            corner, res,
            scale: 1.0,
            grab_loc: None,
        }
    }

    pub fn is_tile_visible(&self, pos: (usize, usize)) -> bool {
        let screen_pos = (pos.0 as f32 * 40.0, pos.1 as f32 * 40.0);
        let bounds = (-self.corner.0 + self.res.0 as f32 * self.scale,
                      -self.corner.1 + self.res.1 as f32 * self.scale);
        screen_pos.0 >= -self.corner.0 - 40.0 && screen_pos.1 >= -self.corner.1 - 40.0 && screen_pos.0 <= bounds.0 && screen_pos.1 <= bounds.1
    }
}

pub fn input_camera_movement(cam: &mut Camera) {
    if cam.grab_loc.is_some() {
        let mouse = mouse_position();
        cam.corner.0 += mouse.0 - cam.grab_loc.unwrap().0;
        cam.corner.1 += mouse.1 - cam.grab_loc.unwrap().1;
    }
    if is_mouse_button_down(MouseButton::Right) {
        cam.grab_loc = Some(mouse_position());
    } else {
        cam.grab_loc = None;
    }
}
