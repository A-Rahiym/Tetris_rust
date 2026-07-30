use crate::board::board::Board;
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

#[allow(dead_code)]
pub struct Game {
    pub active_pieces: Piece,
    pub next_pieces: Piece,
    pub state: GameState,
    pub board: Board,

    pub score: Score,
    pub timing: Timing,

    pub speed: GameSpeed,
    pub level: GameLevel,
}

impl Game {
    pub fn new() -> Self {
        Self {
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
            },
            speed: GameSpeed {
                gravity_seconds: 1,
            },
            level: GameLevel { level: 1 },
        }
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

        // Has enough time elapsed?

        // Try moving down

        // If collision

        // Lock Piece
    }

    fn update_gravity(&mut self) {

        // Has enough time elapsed?

        // Try moving down

        // If collision

        // Lock Piece
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
                if (0..10).contains(&x) && (0..20).contains(&y) {
                    self.board.cells[y as usize][x as usize] = Cell::Filled(piece.kind);
                }
            }
        }
    }

    fn check_for_completed_rows(&mut self) {

        // Scan every row

        // Remove completed ones

        // Collapse board
    }

    fn spawn_piece(&mut self) {

        // Next Piece

        // becomes Active Piece

        // Generate another Next Piece
    }

    fn check_game_over(&mut self) {

        // Can new piece fit?

        // If not

        // state = GameOver
    }
}