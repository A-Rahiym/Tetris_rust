use crate :: piece :: piece :: Piece;
use crate:: game :: state :: GameState;
use crate::board::board::Board;
use crate::score::score::Score;
use crate::timing::timing::Timing;
use crate::game::level::GameLevel;
use crate::game::speed::GameSpeed;


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
