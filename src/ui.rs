use macroquad::prelude::*;

use crate::game_config::GameConfig;
use crate::types::GameState;
use crate::constants::{CELL_SIZE, OFFSET_X, OFFSET_Y, FONT_BYTES, THICKNESS};

pub struct Ui {
    smiley: Rect,
    pub flags_left: i32,
    flags_text: String,
    start_time: f64,
    timer_text: String,
    font: Font,
    font_size: u16,
    text_color: Color,
    pressed: bool
}

impl Ui {

    pub fn new(config: &GameConfig) -> Self {

        let grid_center_x = OFFSET_X + (config.cols as f32 * CELL_SIZE) / 2.0;
        let grid_center_y = OFFSET_Y / 2.0;

        let smiley = Rect::new(
            grid_center_x - 20.0,
            grid_center_y - 20.0,
            40.0,
            40.0
        );

        let font = load_ttf_font_from_bytes(FONT_BYTES).expect("Failed to load embedded font");

        let flags_left = config.mines as i32;
        let flags_text = "000".to_string();
        let start_time = 0.0;
        let timer_text = "000".to_string();
        let font_size = 20;
        let text_color = RED;
        let pressed = false;


        Self {
            smiley,
            flags_left,
            flags_text,
            start_time,
            timer_text,
            font,
            font_size,
            text_color,
            pressed
        }
    }

    pub fn init_timer(&mut self) {
        if self.start_time == 0.0 {
            self.start_time = macroquad::time::get_time();
        };
    }

    pub fn update(&mut self) {
        if self.start_time != 0.0 {
            let elapsed = if self.start_time > 0.0 { ((macroquad::time::get_time() - self.start_time) as u32).min(999) } else { 0 };
            self.timer_text = format!("{:03}", elapsed);
        }
        self.flags_text = self.format_flags()
    }

    pub fn format_flags(&mut self) -> String {
        if self.flags_left >= 0 {
            format!("{:03}", self.flags_left)
        } else {
            let abs_val = self.flags_left.abs();
            if abs_val < 10 {
                format!("0-{}", abs_val)
            } else {
                format!("-{}", abs_val)
            }
        }
    }

    pub fn draw_counters(&self) {
        let middle = screen_width() / 2.0;

        // Left counter - how many flags left
        draw_rectangle(middle - 90., 33., 50., 35., BLACK);
        draw_text_ex(
            &self.flags_text,
            middle - 84.,
            60.,
            TextParams {
                font: Some(&self.font),
                font_size: self.font_size,
                color: self.text_color,
                ..Default::default()
            }
        );
        draw_rectangle(middle + 40., 33., 50., 35., BLACK);
        draw_text_ex(
            &self.timer_text,
            middle + 46.0,
            60.,
            TextParams {
                font: Some(&self.font),
                font_size: 20,
                color: RED,
                ..Default::default()
            }
        );
    }

    pub fn process_smiley_click(&mut self) -> bool {

        let mouse_pos: Vec2 = mouse_position().into();
        let is_hovered = self.smiley.contains(mouse_pos);

        // 1. CAPTURE: Did the mouse button JUST go down?
        if is_mouse_button_pressed(MouseButton::Left) {
            // Remember if we clicked ON the smiley
            self.pressed = is_hovered;
        }

        // 2. RELEASE: Did the mouse button JUST go up?
        if is_mouse_button_released(MouseButton::Left) {
            // If we captured it initially, AND we are releasing while still hovering...
            if self.pressed && is_hovered {
                self.pressed = false; // Reset
                return true; // CONFIRMED CLICK!
            }
            // Otherwise, cancel the click
            self.pressed = false;
        }

        return false;
    }

    pub fn draw_smiley_rect(&self, state: &GameState) {
        
            let x = self.smiley.x;
            let y = self.smiley.y;
            let w = self.smiley.w;
            let h = self.smiley.h;
            let t = THICKNESS; // 2.0
            let primary_color;
            let secondary_color;

            let mouse_pos= mouse_position().into();
            let is_hovered = self.smiley.contains(mouse_pos);

            if self.pressed && is_hovered {
                primary_color = DARKGRAY;
                secondary_color = WHITE;
            } else {
                primary_color = WHITE;
                secondary_color = DARKGRAY;
            }

            // Cell drawing consists of four edges and a center rectangle:
            // 1. Top edge (White highlight)
            draw_rectangle(x, y, w - t, t, primary_color);
            // 2. Left edge (White highlight)
            draw_rectangle(x, y, t, h - t, primary_color);

            // 3. Bottom edge (Dark shadow)
            draw_rectangle(x + t, y + h - t, w - t, t, secondary_color);
            // 4. Right edge (Dark shadow)
            draw_rectangle(x + w - t, y + t, t, h - t, secondary_color);

            // 5. Center face (Light gray)
            draw_rectangle(x + t, y + t, w - 2.0 * t, h - 2.0 * t, LIGHTGRAY);

        match state {
            GameState::Active => {
                if self.pressed && is_hovered {
                    self.draw_scared_face();
                } else {
                    self.draw_smiling_face();
                }
            },
            GameState::Won => {
                self.draw_cool_face();
            },
            GameState::Lost => {
                self.draw_dead_face();
            }
        }
    }

    pub fn draw_smiling_face(&self) {
        let x = self.smiley.center().x;

        // Yellow face circle
        draw_circle(x, OFFSET_Y / 2.0, 15.0, YELLOW);
        // Black outline of the face
        draw_circle_lines(x, OFFSET_Y / 2.0, 14.0, 1.0, BLACK);
        // Mouth/smile
        draw_arc(x, OFFSET_Y / 2.0 - 3.0, 20, 10.0, 45.0, THICKNESS, 90.0, BLACK);
        // Eyes
        draw_rectangle(x - 7.0, OFFSET_Y / 2.0 - 7.0, 4.0, 4.0, BLACK);
        draw_rectangle(x + 3.0, OFFSET_Y / 2.0 - 7.0, 4.0, 4.0, BLACK);
    }

    pub fn draw_dead_face(&self) {

        let x = self.smiley.center().x;

        // Yellow face circle
        draw_circle(x, OFFSET_Y / 2.0, 15.0, YELLOW);
        // Black outline of the face
        draw_circle_lines(x, OFFSET_Y / 2.0, 14.0, 1.0, BLACK);
        // Mouth/smile
        draw_arc(x, OFFSET_Y / 2.0 + 15.0, 20, 10.0, 225.0, THICKNESS, 90.0, BLACK);

        // Left eye cross
        draw_line(
            x - 7.0,
            OFFSET_Y / 2.0 - 7.0,
            x - 3.0,
            OFFSET_Y / 2.0 - 3.0,
            THICKNESS / 2.0,
            BLACK
        );

        draw_line(
            x - 3.0,
            OFFSET_Y / 2.0 - 7.0,
            x - 7.0,
            OFFSET_Y / 2.0 - 3.0,
            THICKNESS / 2.0,
            BLACK
        );

        // Right eye cross
        draw_line(
            x + 7.0,
            OFFSET_Y / 2.0 - 7.0,
            x + 3.0,
            OFFSET_Y / 2.0 - 3.0,
            THICKNESS / 2.0,
            BLACK
        );

        draw_line(
            x + 3.0,
            OFFSET_Y / 2.0 - 7.0,
            x + 7.0,
            OFFSET_Y / 2.0 - 3.0,
            THICKNESS / 2.0,
            BLACK
        )
    }

    pub fn draw_scared_face(&self) {

        let x = self.smiley.center().x;

        // Yellow face circle
        draw_circle(x, OFFSET_Y / 2.0, 15.0, YELLOW);
        // Black outline of the face
        draw_circle_lines(x, OFFSET_Y / 2.0, 14.0, 1.0, BLACK);
        // Eyes
        draw_rectangle(x - 7.0, OFFSET_Y / 2.0 - 7.0, 4.0, 4.0, BLACK);
        draw_rectangle(x + 3.0, OFFSET_Y / 2.0 - 7.0, 4.0, 4.0, BLACK);

        // Open mouth
        draw_ellipse_lines(
            x,
            OFFSET_Y / 2.0 + 7.0,
            4.0,
            3.0,
            0.0,
            THICKNESS / 2.0,
            BLACK
        );
    }

    pub fn draw_cool_face(&self) {

        let x = self.smiley.center().x;

        // Yellow face circle
        draw_circle(x, OFFSET_Y / 2.0, 15.0, YELLOW);
        // Black outline of the face
        draw_circle_lines(x, OFFSET_Y / 2.0, 14.0, 1.0, BLACK);
        // Mouth/smile
        draw_arc(x, OFFSET_Y / 2.0 - 3.0, 20, 10.0, 45.0, THICKNESS, 90.0, BLACK);
        // Sunglasses
        // Left eye
        draw_ellipse(
            x - 5.0,
            OFFSET_Y / 2.0 - 5.0,
            4.0,
            6.0,
            0.0,
            BLACK
        );
        draw_rectangle(x - 9.0, OFFSET_Y / 2.0 - 11.0, 8.0, 5.0, YELLOW);

        // Right eye
        draw_ellipse(
            x + 5.0,
            OFFSET_Y / 2.0 - 5.0,
            4.0,
            6.0,
            0.0,
            BLACK
        );
        draw_rectangle(x + 1.0, OFFSET_Y / 2.0 - 11.0, 8.0, 5.0, YELLOW);

        // Bridge
        draw_line(x - 4.0, OFFSET_Y / 2.0 - 5.0, x + 4.0, OFFSET_Y / 2.0 - 5.0, THICKNESS, BLACK);

        // Left arm
        draw_line(x - 9.0, OFFSET_Y / 2.0 - 5.0, x - 15.0, OFFSET_Y / 2.0, 2.0, BLACK);

        // Right arm
        draw_line(x + 9.0, OFFSET_Y / 2.0 - 5.0, x + 15.0, OFFSET_Y / 2.0, 2.0, BLACK);

    }

}