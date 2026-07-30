use crate::board::board::Board;
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
                PieceKind::T,
                Position { x: 3, y: 0 },
                Rotation::North,
            ),
            next_pieces: Piece::new(
                PieceKind::L,
                Position { x: 3, y: 0 },
                Rotation::North,
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

        // Move Left

        // Move Right

        // Rotate

        // Soft Drop

        // Hard Drop
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

        // Copy active piece
        // into board cells
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