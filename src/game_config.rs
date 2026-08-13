use macroquad::ui::{root_ui, widgets::ComboBox};
use macroquad::window::request_new_screen_size;
use macroquad::hash;

use crate::types::Difficulty;
use crate::constants::{CELL_SIZE, OFFSET_X, OFFSET_Y};

#[derive(Clone)]
pub struct GameConfig {
    pub difficulty: Difficulty,
    pub rows: usize,
    pub cols: usize,
    pub mines: usize,
    pub difficulty_index: usize, 
}

impl GameConfig {
    pub fn new() -> Self {
        let difficulty = Difficulty::Beginner;
        let rows = 9;
        let cols = 9;
        let mines = 10;

        Self {
            difficulty,
            rows,
            cols,
            mines,
            difficulty_index: 0,
        }
    }
}

impl GameConfig {
    pub fn diff_changed(&mut self) -> bool {

        ComboBox::new(hash!("selector"), &["Beginner", "Intermediate", "Expert"])
            .label("Difficulty")
            .ui(&mut *root_ui(), &mut self.difficulty_index);
    
        let new_difficulty = match self.difficulty_index {
            0 => Difficulty::Beginner,
            1 => Difficulty::Intermediate,
            2 => Difficulty::Expert,
            _ => Difficulty::Beginner,
        };

        if self.difficulty != new_difficulty {
            self.difficulty = new_difficulty;
            true
        } else {
            false
        }
    }

    pub fn update(&mut self) {
        let new_config = match self.difficulty {
            Difficulty::Beginner => GameConfig {
                difficulty: Difficulty::Beginner,
                rows: 9,
                cols: 9,
                mines: 10,
//                window_size: vec2(200.0, 300.0),
                difficulty_index: 0,
            },
            Difficulty::Intermediate => GameConfig {
                difficulty: Difficulty::Intermediate,
                rows: 16,
                cols: 16,
                mines: 40,
                difficulty_index: 1,
            },
            Difficulty::Expert => GameConfig {
                difficulty: Difficulty::Expert,
                rows: 16,
                cols: 30,
                mines: 99,
                difficulty_index: 2,
            },
        };
        
        *self = new_config;

        let window_width = self.cols as f32 * CELL_SIZE + OFFSET_X * 2.0;
        let window_height = self.rows as f32 * CELL_SIZE + OFFSET_X + OFFSET_Y;

        request_new_screen_size(window_width, window_height);
    }
}