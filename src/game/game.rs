use std::time::Duration;

use macroquad::prelude::*;
use ::rand::Rng;

use crate::board::board::{Board, BOARD_WIDTH, BOARD_HEIGHT};
use crate::board::cell::Cell;
use crate::game::level::GameLevel;
use crate::game::speed::GameSpeed;
use crate::game::state::GameState;
use crate::input::actions::PlayerAction;
use crate::piece::kind::PieceKind;
use crate::piece::piece::Piece;
use crate::piece::position::Position;
use crate::piece::rotation::Rotation;
use crate::score::score::Score;
use crate::timing::timing::Timing;

#[allow(dead_code) ]
#[derive(Debug)]
pub struct Game {
    pub active_pieces: Piece,
    pub next_pieces: Piece,
    pub state: GameState,
    pub board: Board,

    pub score: Score,
    pub timing: Timing,

    pub speed: GameSpeed,
    pub level: GameLevel,

    pub lines_cleared: usize,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            active_pieces: Piece::new(
                PieceKind::I,
                Position { x: 3, y: 8 },
                Rotation::North,
            ),
            next_pieces: Piece::new(
                PieceKind::L,
                Position { x: 2, y: 7 },
                Rotation::East,
            ),
            state: GameState::Running,
            board: Board::new(),
            score: Score { points: 0 },
            timing: Timing {
                tick_duration: std::time::Duration::from_millis(500),
                elapsed: std::time::Duration::ZERO,
            },
            speed: GameSpeed {
                gravity_seconds: 1,
            },
            level: GameLevel { level: 1 },
            lines_cleared: 0,
        };

        game.apply_speed();
        game
    }

    pub fn update(&mut self, action: Option<PlayerAction>) {
        self.handle_input(action);

        self.update_gravity();

        self.check_for_completed_rows();

        self.update_score();

        self.spawn_piece();

        self.check_game_over();
    }

    fn handle_input(&mut self, action: Option<PlayerAction>) {
        match action {
            Some(PlayerAction::MoveLeft) => {
                self.try_offset(-1, 0);
            }
            Some(PlayerAction::MoveRight) => {
                self.try_offset(1, 0);
            }
            Some(PlayerAction::Rotate) => {
                self.rotate_active();
            }
            Some(PlayerAction::SoftDrop) => {
                self.try_offset(0, 1);
            }
            Some(PlayerAction::HardDrop) => {
                self.hard_drop();
            }
            Some(PlayerAction::Pause) => {
                self.state = match self.state {
                    GameState::Running => GameState::Paused,
                    GameState::Paused => GameState::Running,
                    GameState::GameOver => GameState::GameOver,
                };
            }
            None => {}
        }
    }

    fn try_offset(&mut self, dx: i32, dy: i32) -> bool {
        let mut candidate = self.active_pieces.clone();
        candidate.position.x += dx;
        candidate.position.y += dy;

        if self.board.is_valid(&candidate) {
            self.active_pieces = candidate;
            true
        } else {
            false
        }
    }

    fn rotate_active(&mut self) {
        let mut candidate = self.active_pieces.clone();
        candidate.rotate_clockwise();

        if self.board.is_valid(&candidate) {
            self.active_pieces = candidate;
        }
    }

    fn hard_drop(&mut self) {
        while self.try_offset(0, 1) {}
        self.lock_piece();
    }



    fn update_score(&mut self) {
        let lines = self.lines_cleared;
        if lines == 0 {
            return;
        }

        let points: u32 = match lines {
            1 => 100,
            2 => 300,
            3 => 500,
            _ => 800,
        };
        self.score.points += points;
        self.lines_cleared = 0;

        self.level.level += (lines / 10) as u32;
        self.apply_speed();
    }

    fn apply_speed(&mut self) {
        let base = Duration::from_millis(500);
        let step = Duration::from_millis(25);
        let level = self.level.level.max(1);

        let tick = base.saturating_sub(step.saturating_mul(level - 1));
        self.timing.tick_duration = tick.max(Duration::from_millis(60));
    }

    fn update_gravity(&mut self) {
        if self.state != GameState::Running {
            return;
        }

        self.timing.elapsed += Duration::from_secs_f32(get_frame_time());

        if self.timing.elapsed >= self.timing.tick_duration {
            self.timing.elapsed = Duration::ZERO;
            if !self.try_offset(0, 1) {
                self.lock_piece();
            }
        }
    }

    fn lock_piece(&mut self) {
        let piece = &self.active_pieces;
        for row in 0..4 {
            for col in 0..4 {
                if !piece.shape.cells[row][col] {
                    continue;
                }
                let x = piece.position.x + col as i32;
                let y = piece.position.y + row as i32;
                if x >= 0 && x < BOARD_WIDTH as i32 && y >= 0 && y < BOARD_HEIGHT as i32 {
                    self.board.cells[y as usize][x as usize] = Cell::Filled(piece.kind);
                }
            }
        }
    }

    fn check_for_completed_rows(&mut self) {
        let full: Vec<usize> = (0..BOARD_HEIGHT)
            .filter(|&row| {
                self.board.cells[row]
                    .iter()
                    .all(|cell| matches!(cell, Cell::Filled(_)))
            })
            .collect();

        if full.is_empty() {
            return;
        }

        let mut collapsed = [[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT];
        let mut write = BOARD_HEIGHT - 1;

        for read in (0..BOARD_HEIGHT).rev() {
            if full.contains(&read) {
                continue;
            }
            collapsed[write] = self.board.cells[read];
            write = write.wrapping_sub(1);
        }

        self.board.cells = collapsed;
        self.lines_cleared = full.len();
    }

    fn spawn_piece(&mut self) {
        if self.board.is_valid(&self.active_pieces) {
            return;
        }

        self.active_pieces = self.next_pieces.clone();
        self.active_pieces.position = Position { x: 3, y: 0 };

        self.next_pieces = Piece::new(
            Self::random_kind(),
            Position { x: 3, y: 0 },
            Rotation::North,
        );
    }

    fn random_kind() -> PieceKind {
        const KINDS: [PieceKind; 7] = [
            PieceKind::I,
            PieceKind::O,
            PieceKind::T,
            PieceKind::S,
            PieceKind::Z,
            PieceKind::J,
            PieceKind::L,
        ];
        let idx = ::rand::rng().random_range(0..7);
        KINDS[idx as usize]
    }

    fn check_game_over(&mut self) {
        if !self.board.is_valid(&self.active_pieces) {
            self.state = GameState::GameOver;
        }
    }
}