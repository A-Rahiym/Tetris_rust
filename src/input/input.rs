use crate::input::actions::PlayerAction;
use macroquad::prelude::*;

pub fn poll() -> Option<PlayerAction> {
    if is_key_pressed(KeyCode::Left) {
        return Some(PlayerAction::MoveLeft);
    }
    if is_key_pressed(KeyCode::Right) {
        return Some(PlayerAction::MoveRight);
    }
    if is_key_pressed(KeyCode::Up) {
        return Some(PlayerAction::Rotate);
    }
    if is_key_pressed(KeyCode::Down) {
        return Some(PlayerAction::SoftDrop);
    }
    if is_key_pressed(KeyCode::Space) {
        return Some(PlayerAction::HardDrop);
    }
    if is_key_pressed(KeyCode::P) {
        return Some(PlayerAction::Pause);
    }
    None
}
