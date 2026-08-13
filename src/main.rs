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

        if let Some(mouse_button) = game.get_pressed_mouse_button() {
            game.process_grid_click(mouse_button)
        }

        if game.state == GameState::Active {
            game.ui.update()
        };
        game.ui.draw_counters();

        // combobox is drawn here
        if game.config.diff_changed() {
            game.config.update();

            game = Game::new(game.config)
        }

        game.board.draw_grid();

        if game.ui.process_smiley_click() {

            game = Game::new(game.config)
        };
        game.ui.draw_smiley_rect(&game.state);

        next_frame().await
    }
}

pub struct Game {
    config: GameConfig,
    ui: Ui,
    board: Board,
    start_time: f64,
    state: GameState
}

impl Game {
    pub fn new(config: GameConfig) -> Self {
        let config = config;
        let ui = Ui::new(&config);
        let board = Board::new(&config);
        let start_time = 0.0;
        let state = GameState::Active;

        Self {
            config,
            ui,
            board,
            start_time,
            state
        }
    }

    pub fn get_pressed_mouse_button(&self) -> Option<MouseButton> {
        if is_mouse_button_pressed(MouseButton::Left) {
            return Some(MouseButton::Left);
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            return Some(MouseButton::Right);
        }
        if is_mouse_button_pressed(MouseButton::Middle) {
            return Some(MouseButton::Middle);
        }
        None
    }

    pub fn process_grid_click(&mut self, mouse_button: MouseButton) {

        if self.state != GameState::Active {
            return
        }

        let mouse_pos = mouse_position();

        if let Some(clicked) = self.board.get_cell_index(mouse_pos) {

            match mouse_button {
                MouseButton::Left => {
                    self.ui.init_timer();
                    if self.start_time == 0.0 {
                        self.start_time = macroquad::time::get_time();
                    }
                    let current_cell = self.board.grid[clicked].clone();

                    match current_cell {
                        // 1. Unflagged Mine: BOOM. Game Over.
                        Cell::Hidden(true, None) => {
                            self.state = GameState::Lost;
                            
                            // 2. Now we can safely mutate because the borrow is gone!
                            self.board.grid[clicked] = Cell::Mine(true);

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
                            self.board.reveal_cells(clicked); 
                        },
                        // 3. Flagged cells, Revealed cells, or Game Over states: Do nothing
                        _ => {}
                    }
                },
                MouseButton::Right => {
                    match self.board.grid[clicked] {
                        Cell::Hidden(is_mine, None) => {
                            self.board.grid[clicked] = Cell::Hidden(is_mine, Some(self.board.flag_char));
                            self.ui.flags_left -= 1;
                        },
                        Cell::Hidden(is_mine, Some(_)) => {
                            self.board.grid[clicked] = Cell::Hidden(is_mine, None);
                            self.ui.flags_left += 1;
                        },
                        _ => {} // Do nothing if already revealed
                    }
                },
                _ => ()
            }
        }
    }

    pub fn is_won(&mut self) {
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
