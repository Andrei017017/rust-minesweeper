use macroquad::prelude::*;

mod game_config;
mod board;
mod types;
mod constants;
mod ui;

use ui::Ui;
use board::Board;
use game_config::GameConfig;
use types::{GameState, Cell};
use constants::{OFFSET_X, OFFSET_Y};

#[macroquad::main(window_conf)]
async fn main() {

    let mut game = Game::new(GameConfig::new());

    loop {
        clear_background(LIGHTGRAY);

        game.process_grid_click();
        game.check_win();

        if game.state == GameState::Active {
            game.ui.update()
        };
        game.ui.draw_counters();

        // combobox is drawn here
        if game.config.diff_changed() {
            game.config.update();

            game = Game::new(game.config)
        }

        game.board.draw_grid(&game.pressed_cells);

        if game.ui.process_smiley_click() {

            game = Game::new(game.config)
        };
        game.ui.draw_smiley_rect(&game.state, &game.pressed_cells);

        next_frame().await
    }
}

pub struct Game {
    config: GameConfig,
    ui: Ui,
    board: Board,
    state: GameState,
    pressed_cells: Vec<usize>,
    is_chording: bool
}

impl Game {
    pub fn new(config: GameConfig) -> Self {
        let config = config;
        let ui = Ui::new(&config);
        let board = Board::new(&config);
        let state = GameState::Active;
        let pressed_cells = Vec::new();
        let is_chording = false;

        Self {
            config,
            ui,
            board,
            state,
            pressed_cells,
            is_chording
        }
    }

    pub fn process_grid_click(&mut self) {

        if self.state != GameState::Active {
            return
        }

        let mouse_pos = mouse_position();
        self.pressed_cells.clear();

        // DOWN Mouse buttons logic
        // Check for Chording: Middle button OR (Left + Right)
        if is_mouse_button_down(MouseButton::Middle) || (is_mouse_button_down(MouseButton::Left) && is_mouse_button_down(MouseButton::Right)) {
                self.is_chording = true;
            }

        if let Some(current_index) = self.board.get_cell_index(mouse_pos) {

            if self.is_chording {
                // Visuals only: push the center cell and all its neighbors
                self.pressed_cells.push(current_index);
                self.pressed_cells.extend(self.board.get_neighbors(current_index));
            } else if is_mouse_button_down(MouseButton::Left) {
                self.pressed_cells.push(current_index);
            } else {
                // Mouse is off the grid, nothing should look pushed
                self.pressed_cells.clear();
            }
        };
        
        if self.is_chording && (is_mouse_button_released(MouseButton::Middle) ||
        (!is_mouse_button_down(MouseButton::Left) && !is_mouse_button_down(MouseButton::Right))) {
            if let Some(released_index) = self.board.get_cell_index(mouse_pos) {
                self.process_chord_logic(released_index);
                self.is_chording = false
            }
        }
        else if is_mouse_button_released(MouseButton::Left) {
            if let Some(released_index) = self.board.get_cell_index(mouse_pos) {

                self.ui.init_timer();
                let current_cell = self.board.grid[released_index].clone();

                match current_cell {
                    // 1. Unflagged Mine: BOOM. Game Over.
                    Cell::Hidden(true, None) => {
                        self.state = GameState::Lost;
                        
                        // 2. Now we can safely mutate because the borrow is gone!
                        self.board.grid[released_index] = Cell::Mine(true);

                        // 3. Reveal the rest of the board
                        for cell in self.board.grid.iter_mut() {
                            match cell {
                                // Unflagged mine -> Reveal it
                                Cell::Hidden(true, None) => {
                                    *cell = Cell::Mine(false);
                                }
                                // Safe cell with a flag -> Mark as wrong
                                Cell::Hidden(false, Some(_)) => {
                                    *cell = Cell::WrongFlag;
                                }
                                // Correctly flagged mine OR already revealed -> Do nothing
                                _ => {}
                            }
                        }
                    },
                    // 2. Unflagged Safe Cell: Reveal it (and cascade if empty)
                    Cell::Hidden(false, None) => { 
                        self.board.reveal_cells(released_index); 
                    },
                    // 3. Flagged cells, Revealed cells, or Game Over states: Do nothing
                    _ => {}
                }
            };
        }
        else if is_mouse_button_pressed(MouseButton::Right) && !self.is_chording {
            if let Some(released_index) = self.board.get_cell_index(mouse_pos) {
                match self.board.grid[released_index] {
                    Cell::Hidden(is_mine, None) => {
                        self.board.grid[released_index] = Cell::Hidden(is_mine, Some(self.board.flag_char));
                        self.ui.flags_left -= 1;
                    },
                    Cell::Hidden(is_mine, Some(_)) => {
                        self.board.grid[released_index] = Cell::Hidden(is_mine, None);
                        self.ui.flags_left += 1;
                    },
                    _ => {} // Do nothing if already revealed
                }
            }    
        }
    }

    pub fn process_chord_logic(&mut self, released_index: usize) {

        if let Cell::Revealed(Some(num)) = self.board.grid[released_index] {
            let neighbours = self.board.get_neighbors(released_index);

            // 1. Count how many flagged cells are around this number
            let mut flags = 0;
            for &neighbor_index in &neighbours {
                if let Cell::Hidden(_, Some(_)) = self.board.grid[neighbor_index] {
                    flags += 1;
                }
            }

            // 2. If the flags match the number, reveal the unflagged neighbors
            if flags == num {
                for &neighbor_index in &neighbours {
                    // Clone to satisfy the borrow checker, just like before
                    let current_cell = self.board.grid[neighbor_index].clone();

                    match current_cell {
                        // BOOM! Hit an unflagged mine.
                        Cell::Hidden(true, None) => {
                            self.state = GameState::Lost;
                            
                            // Mark the specific mine we hit as the exploded one
                            self.board.grid[neighbor_index] = Cell::Mine(true);

                            // Reveal the rest of the board (reuse your existing logic!)
                            for cell in self.board.grid.iter_mut() {
                                match cell {
                                    Cell::Hidden(true, None) => *cell = Cell::Mine(false),
                                    Cell::Hidden(false, Some(_)) => *cell = Cell::WrongFlag,
                                    _ => {}
                                }
                            }
                            
                            // CRITICAL: Stop processing further neighbors immediately!
                            return; 
                        },
                        // Safe cell: Reveal it (this automatically handles cascading if it's a 0!)
                        Cell::Hidden(false, None) => {
                            self.board.reveal_cells(neighbor_index);
                        },
                        // Already revealed or correctly flagged: Do nothing
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn check_win(&mut self) {
        // WIN CONDITION CHECK
        let total_safe_cells = (self.board.rows * self.board.cols) - self.board.mines;
        if self.board.revealed == total_safe_cells {
            self.state = GameState::Won;
        }
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Minesweeper".to_string(),
        window_width: 180 + OFFSET_X as i32 * 2,
        window_height: 180 + OFFSET_Y as i32 + OFFSET_X as i32,
        window_resizable: false,
        ..Default::default()
    }
}
