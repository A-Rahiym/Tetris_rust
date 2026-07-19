#[derive(Clone, Copy, Debug)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    Rotate,
    SoftDrop,
    HardDrop,
    Pause,
}