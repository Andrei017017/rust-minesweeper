use ::rand::rng;
use ::rand::seq::index::sample;
use macroquad::prelude::*;

use crate::types::Cell;
use crate::game_config::GameConfig;
use crate::constants::{DIRECTIONS, CELL_SIZE, OFFSET_X, OFFSET_Y, THICKNESS};

pub struct Board {
    pub rows: usize,
    pub cols: usize,
    pub mines: usize,
    pub revealed: usize,
    pub grid: Vec<Cell>,
    pub flag_char: char,
}

impl Board {

    pub fn new(game_config: &GameConfig) -> Self {
        let rows = game_config.rows;
        let cols = game_config.cols;
        let mines = game_config.mines;
        let flag_char = 'µ';
        let revealed = 0;
        let mut grid = vec![Cell::Hidden(false, None); rows*cols];

        let mut rng = rng();
        let indices = sample(&mut rng, rows*cols, mines);
        
        for index in indices {
            grid[index] = Cell::Hidden(true, None);
        }

        Self {
            rows,
            cols,
            mines,
            revealed,
            grid,
            flag_char
        }
    }

    pub fn draw_grid(&self, pressed_cells: &Vec<usize>) {
        for i in 0..self.rows*self.cols {
            
            let row = (i / self.cols) as f32;
            let col = (i % self.cols) as f32;

            let primary_color;
            let secondary_color;
            let mut center_face =  LIGHTGRAY;

            let mut text_color = BLACK;
            let mut text = String::default();
            let mut flag_exists = false;
            let mut wrong_flag_exists = false;
            let mut mine_exists = false;

            match self.grid[i] {
                Cell::Hidden(_, option) => {
                    if option.is_some() {
                        // It has a flag. Always drawn not pushed
                        flag_exists = true;
                        primary_color = WHITE;
                        secondary_color = DARKGRAY;
                    } else {
                        // No flag. Drawn pushed if current i in pressed_cells
                        let is_pushed = pressed_cells.contains(&i);
                        
                        primary_color = if is_pushed { DARKGRAY } else { WHITE };
                        secondary_color = if is_pushed { WHITE } else { DARKGRAY };
                    }
                },
                Cell::Revealed(None) => {
                    primary_color = DARKGRAY;
                    secondary_color =  WHITE;
                },
                Cell::Revealed(Some(num)) => {
                    primary_color = DARKGRAY;
                    secondary_color = WHITE;
                    text = num.to_string();

                    text_color = match num {
                        1 => BLUE,
                        2 => DARKGREEN,
                        3 => RED,
                        4 => PURPLE,
                        5 => MAROON,
                        6 => MAGENTA,
                        7 => BLACK,
                        8 => BROWN,
                        _ => BLANK
                    }
                },
                Cell::Mine(is_clicked) => {
                    primary_color = WHITE;
                    secondary_color = DARKGRAY;
                    mine_exists = true;
                    
                    if is_clicked {
                        center_face = RED
                    }
                },
                Cell::WrongFlag => {
                    primary_color = WHITE;
                    secondary_color = DARKGRAY;
                    // Adding a cross over the flag
                    wrong_flag_exists = true;
                }
            };

            let x = OFFSET_X + col * CELL_SIZE;
            let y = OFFSET_Y + row * CELL_SIZE;
            let t = THICKNESS; // 2.0

            // Cell drawing consists of four edges and a center rectangle:
            // 1. Top edge (White highlight)
            draw_rectangle(x, y, CELL_SIZE - THICKNESS, t, primary_color);
            // 2. Left edge (White highlight)
            draw_rectangle(x, y, t, CELL_SIZE - THICKNESS, primary_color);

            // 3. Bottom edge (Dark shadow)
            draw_rectangle(x + THICKNESS, y + CELL_SIZE - t, CELL_SIZE - THICKNESS, t, secondary_color);
            // 4. Right edge (Dark shadow)
            draw_rectangle(x + CELL_SIZE - t, y + THICKNESS, t, CELL_SIZE - THICKNESS, secondary_color);

            // 5. Center face (Light gray)
            draw_rectangle(x + t, y + t, CELL_SIZE - 2.0 * t, CELL_SIZE - 2.0 * t, center_face);

            if !text.is_empty() {
                let text_dimensions = measure_text(&text, None, 18, 1.);
                let rect_center = vec2(
                    OFFSET_X + col * CELL_SIZE + CELL_SIZE / 2.,
                    OFFSET_Y + row * CELL_SIZE + CELL_SIZE / 2.
                ); 
                
                draw_text(
                    &text,
                    rect_center.x - text_dimensions.width / 2.,
                    rect_center.y - text_dimensions.height / 2. + text_dimensions.offset_y,
                    18.,
                    text_color
                );
            }

            if flag_exists {
                self.draw_flag(i);
            }

            if wrong_flag_exists {
                self.draw_flag(i);
                self.draw_wrong_flag(i);
            }
            if mine_exists {
                self.draw_mine(i)
            }
        }
    }

    pub fn draw_mine(&self, index: usize) {
        // 1. Convert 1D index to 2D coordinates
        let row = (index / self.cols) as f32;
        let col = (index % self.cols) as f32;

        // 2. Calculate correct X and Y
        let cell_x = OFFSET_X + col * CELL_SIZE;
        let cell_y = OFFSET_Y + row * CELL_SIZE;

        // body
        draw_circle(cell_x + CELL_SIZE / 2.0, cell_y + CELL_SIZE / 2.0, CELL_SIZE * 0.2, BLACK);
        // horizontal spike
        draw_line(
            cell_x + CELL_SIZE * 0.15,
            cell_y + CELL_SIZE * 0.5,
            cell_x + CELL_SIZE * 0.85,
            cell_y + CELL_SIZE * 0.5,
            2.0,
            BLACK
        );
        // vertical spike
        draw_line(
            cell_x + CELL_SIZE * 0.5,
            cell_y + CELL_SIZE * 0.15,
            cell_x + CELL_SIZE * 0.5,
            cell_y + CELL_SIZE * 0.85,
            2.0,
            BLACK
        );
        // negative diagonal spike
        draw_line(
            cell_x + CELL_SIZE * 0.25,
            cell_y + CELL_SIZE * 0.25,
            cell_x + CELL_SIZE * 0.75,
            cell_y + CELL_SIZE * 0.75,
            2.0,
            BLACK
        );
        // positive diagonal spike
        draw_line(
            cell_x + CELL_SIZE * 0.25,
            cell_y + CELL_SIZE * 0.75,
            cell_x + CELL_SIZE * 0.75,
            cell_y + CELL_SIZE * 0.25,
            2.0,
            BLACK
        );
        // glare
        draw_rectangle(
            cell_x + CELL_SIZE * 0.4,
            cell_y + CELL_SIZE * 0.4,
            2.0,
            2.0,
            WHITE
        );
    }

    pub fn draw_flag(&self, index: usize) {
        // 1. Convert 1D index to 2D coordinates
        let row = (index / self.cols) as f32;
        let col = (index % self.cols) as f32;

        // 2. Calculate correct X and Y
        let cell_x = OFFSET_X + col * CELL_SIZE;
        let cell_y = OFFSET_Y + row * CELL_SIZE;

        // 1. Pole (Vertical line)
        draw_line(
            cell_x + CELL_SIZE * 0.5,
            cell_y + CELL_SIZE * 0.15,
            cell_x + CELL_SIZE * 0.5,
            cell_y + CELL_SIZE * 0.75,
            2.0,
            BLACK
        );

        // 2. Base Stand (Bottom steps)
        // Upper base step
        draw_line(
            cell_x + CELL_SIZE * 0.35,
            cell_y + CELL_SIZE * 0.70,
            cell_x + CELL_SIZE * 0.65,
            cell_y + CELL_SIZE * 0.70,
            2.0,
            BLACK
        );
        // Lower base step
        draw_line(
            cell_x + CELL_SIZE * 0.25,
            cell_y + CELL_SIZE * 0.75,
            cell_x + CELL_SIZE * 0.75,
            cell_y + CELL_SIZE * 0.75,
            2.0,
            BLACK
        );

        // Red Flag Triangle
        let top = vec2(cell_x + CELL_SIZE * 0.5, cell_y + CELL_SIZE * 0.15);
        let left = vec2(cell_x + CELL_SIZE * 0.15, cell_y + CELL_SIZE * 0.35);
        let bottom = vec2(cell_x + CELL_SIZE * 0.5, cell_y + CELL_SIZE * 0.55);

        draw_triangle(top, bottom, left, RED);

    }

    pub fn draw_wrong_flag(&self, index: usize) {

        let row = (index / self.cols) as f32;
        let col = (index % self.cols) as f32;

        let cell_x = OFFSET_X + col * CELL_SIZE;
        let cell_y = OFFSET_Y + row * CELL_SIZE;

        draw_line(
            cell_x,
            cell_y,
            cell_x + CELL_SIZE,
            cell_y + CELL_SIZE,
            THICKNESS,
            Color::new(1.0, 0.0, 0.0, 0.5)
        );

        draw_line(
            cell_x,
            cell_y + CELL_SIZE,
            cell_x + CELL_SIZE,
            cell_y,
            THICKNESS,
            Color::new(1.0, 0.0, 0.0, 0.5)
        );

    }

    pub fn get_cell_index(&self, mouse_pos: (f32, f32)) -> Option<usize> {
        let (x, y) = mouse_pos;

        if x < OFFSET_X || 
            x > OFFSET_X + self.cols as f32 * CELL_SIZE || 
            y < OFFSET_Y || 
            y > OFFSET_Y + self.rows as f32 * CELL_SIZE 
        {
            return None;
        }

        let col = ((x - OFFSET_X) / CELL_SIZE) as usize;
        let row = ((y - OFFSET_Y) / CELL_SIZE) as usize;
        let i = col + self.cols * row;

        Some(i)
    }

    pub fn count_mines(&self, index: usize) -> u8 {
        let mut mine_count = 0;

        let row = index / self.cols;
        let col = index % self.cols;

        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                // Skip the cell itself (0, 0)
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                let neighbor_row = row as i32 + row_offset;
                let neighbor_col = col as i32 + col_offset;

                if neighbor_row >= 0 &&
                    neighbor_row < self.rows as i32 &&
                    neighbor_col >= 0 &&
                    neighbor_col < self.cols as i32 {
                    
                    let neighbor_index = (neighbor_row * self.cols as i32 + neighbor_col) as usize;

                    if let Cell::Hidden(true, _) = self.grid[neighbor_index] {
                        mine_count += 1;
                    }
                }
            }
        }

        mine_count
    }

    // should check for game.over after this:
    pub fn reveal_cells(&mut self, start_index: usize) {
        let mut stack = vec![start_index];

        while let Some(current_index) = stack.pop() {
            // CRITICAL: Prevent infinite loops. 
            // If we already revealed this cell, skip it.
            if let Cell::Revealed(_) = self.grid[current_index] {
                continue;
            };
            if let Cell::Hidden(_, Some(_)) = self.grid[current_index] {
                continue;
            };

            let count = self.count_mines(current_index);

            // Reveal the current cell
            self.grid[current_index] = if count > 0 {
                Cell::Revealed(Some(count))
            } else {
                Cell::Revealed(None)
            };
            self.revealed += 1;

            // If it's an empty cell (0 mines), explore its neighbors
            if count == 0 {
                let row = current_index / self.cols;
                let col = current_index % self.cols;

                for (r_offset, c_offset) in DIRECTIONS {
                    let n_row = row as i32 + r_offset;
                    let n_col = col as i32 + c_offset;

                    // Boundary check
                    if n_row >= 0 && n_row < self.rows as i32 
                    && n_col >= 0 && n_col < self.cols as i32 {
                        
                        let n_index = (n_row * self.cols as i32 + n_col) as usize;
                        
                        // Only push if it's still hidden
                        if let Cell::Hidden(_, _) = self.grid[n_index] {
                            stack.push(n_index);
                        }
                    }
                }
            }
        }
    }

    pub fn get_neighbors(&self, index: usize) -> Vec<usize> {
        let mut neighbors = Vec::with_capacity(8);
        let row = (index / self.cols) as i32;
        let col = (index % self.cols) as i32;

        for r_offset in -1..=1 {
            for c_offset in -1..=1 {
                if r_offset == 0 && c_offset == 0 {
                    continue; // Skip the center cell itself
                }

                let n_row = row + r_offset;
                let n_col = col + c_offset;

                if n_row >= 0 && n_row < self.rows as i32 
                && n_col >= 0 && n_col < self.cols as i32 {
                    neighbors.push((n_row * self.cols as i32 + n_col) as usize);
                }
            }
        }
        neighbors
    }
}