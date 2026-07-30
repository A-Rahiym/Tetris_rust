# Tetris Implementation Plan

Status: **Playable render achieved (board + active piece visible). Game logic not yet interactive.**

---

## Current State (what already works)

- **Data model complete:** `Piece`, `Shape`, `Rotation`, `Position`, `PieceKind`,
  `Cell`, `Board`, `Game`, `Score`, `Timing`, `GameState`, `GameLevel`,
  `GameSpeed`, `PlayerAction`.
- **Piece logic:** `Piece::new`, `Piece::rotate_clockwise`,
  `Piece::rotate_counter_clockwise` (rotation recomputes cached `shape`).
- **Shape data:** `shape.rs` defines all 7 tetromino orientations for every rotation.
- **Rendering:** `Renderer::draw` renders the board grid (empty cells = DARKGRAY,
  filled = piece color) and the active piece. `draw_next_piece`, `draw_score`,
  `draw_level` are stubs.
- **Wiring:** `main.rs` creates `Game::new()` and loops `Renderer::draw(&game)`.
- **Compiles** (only unused-variable warnings).

## Open bugs / gaps

- `handle_input`, `update_gravity`, `lock_piece`, `check_for_completed_rows`,
  `spawn_piece`, `check_game_over`, `update_score` are all empty stubs.
- No collision check exists yet.
- `Game::new` hardcodes `T` and `L` pieces (no random bag).
- Renderer still has `active_pieces`/`next_pieces` unused warnings.

---

## Implementation Stages (in execution order)

### Stage 1 — Compile-blockers (DONE)
- [x] Remove stray `Renderer` struct + unused `Game` import in `board.rs`.
- [x] Remove nonexistent `pub mod input;` in `input/mod.rs`.
- [x] Fix `Renderer` calls (`active_pieces`, `Self::piece_color`).

### Stage 2 — Constructors / Game setup (DONE)
- [x] `Board::new()` -> 10x20 grid of `Cell::Empty`.
- [x] `Game::new()` wiring `Board`, two `Piece`s, state, score, timing, speed, level.

### Stage 3 — Collision / validation (NEXT)
**Goal:** a single source of truth for "can this piece be here?".

Add to `board.rs`:
```rust
impl Board {
    pub fn is_valid(&self, piece: &Piece) -> bool {
        for row in 0..4 {
            for col in 0..4 {
                if !piece.shape.cells[row][col] { continue; }
                let x = piece.position.x + col as i32;
                let y = piece.position.y + row as i32;
                // out of horizontal/vertical bounds
                if x < 0 || x >= 10 || y < 0 || y >= 20 { return false; }
                // overlap with locked cells
                if let Cell::Filled(_) = self.cells[y as usize][x as usize] {
                    return false;
                }
            }
        }
        true
    }
}
```
- Note `y < 0` is allowed during spawn (piece enters from top) — clamp only top
  overflow check to `y >= 20`; allow negative y so pieces can spawn partially
  off-screen. Decide: simplest correct version rejects `y < 0` too; spawn at
  `y = 0` to avoid issues.
- `Piece` needs `#[derive(Copy, Clone)]` OR be passed by `&` (we use `&Piece`,
  already fine).

### Stage 4 — Movement + rotation (input)
**Goal:** player controls move and rotate the active piece, validated.

In `game.rs` `handle_input`:
- `MoveLeft`: `try_offset(-1, 0)`
- `MoveRight`: `try_offset(1, 0)`
- `SoftDrop`: `try_offset(0, 1)` (also award 1 point)
- `HardDrop`: loop `try_offset(0, 1)` until invalid, then `lock_piece()`
- `Rotate`: clone active piece, call `rotate_clockwise()`, if `board.is_valid`
  keep it; else try simple wall-kick offsets (see Stage 12)
- `Pause`: toggle `GameState::Running <-> Paused`

Helper `try_offset(dx, dy)`:
```rust
fn try_offset(&mut self, dx: i32, dy: i32) -> bool {
    let mut p = self.active_pieces.clone();
    p.position.x += dx;
    p.position.y += dy;
    if self.board.is_valid(&p) {
        self.active_pieces = p;
        true
    } else { false }
}
```

Wire `main.rs` to read keys -> `PlayerAction` via a new `input` module
(`Input::poll() -> Option<PlayerAction>` mapping macroquad key codes).

### Stage 5 — Gravity
**Goal:** piece auto-falls on a timer.

- Add `accumulated: Duration` to `Timing` (or track in `Game`).
- In `update_gravity`: if elapsed >= `tick_duration`, `try_offset(0, 1)`;
  if it fails (can't move down) -> `lock_piece()`.
- `tick_duration` should derive from `GameSpeed`/`GameLevel` later.

### Stage 6 — Lock piece
**Goal:** write the active piece into the board.

`lock_piece`:
```rust
fn lock_piece(&mut self) {
    let p = &self.active_pieces;
    for row in 0..4 {
        for col in 0..4 {
            if !p.shape.cells[row][col] { continue; }
            let x = p.position.x + col as i32;
            let y = p.position.y + row as i32;
            if (0..10).contains(&x) && (0..20).contains(&y) {
                self.board.cells[y as usize][x as usize] = Cell::Filled(p.kind);
            }
        }
    }
}
```

### Stage 7 — Line clear
**Goal:** remove full rows, collapse board, update score.

`check_for_completed_rows`:
- Scan rows 0..20; a row is full if no `Cell::Empty`.
- Collect full row indices, count `cleared`.
- Remove those rows, push empty rows at top (shift down).
- Route `cleared` count into `Score` (e.g. 1=100, 2=300, 3=500, 4=800).

### Stage 8 — Spawn + game over
**Goal:** continuous play, end condition.

- `spawn_piece`: `active_pieces = next_pieces.clone()`; generate new
  `next_pieces` (random kind, `Position{x:3, y:0}`, `North`); if
  `!board.is_valid(&active_pieces)` -> `state = GameState::GameOver`.
- Implement a 7-bag randomizer (optional but recommended) for fair piece order.

### Stage 9 — Score / level / speed
**Goal:** progression.

- `Score::add_lines(cleared)` updates points.
- Level up every 10 cleared lines (`GameLevel`).
- `GameSpeed.gravity_seconds` decreases with level; map to `Timing.tick_duration`.

### Stage 10 — Renderer completion
**Goal:** full HUD.

- `draw_active_piece` already works (verify).
- `draw_next_piece`: render `game.next_pieces` in a side panel (offset coords).
- `draw_score`: `draw_text` points at a fixed HUD location.
- `draw_level`: `draw_text` level.
- Optional: draw board border / ghost piece (semi-transparent landing preview).

### Stage 11 — Wire main loop
**Goal:** final integration.

`main.rs`:
```rust
loop {
    clear_background(BLACK);
    if game.state == GameState::Running {
        let action = input::poll();
        game.update(action);
    }
    Renderer::draw(&game);
    if game.state == GameState::GameOver {
        draw_text("GAME OVER", ...);
    }
    next_frame().await
}
```

### Stage 12 — Polish (optional)
- Next-piece preview panel (Stage 10).
- Pause screen (`GameState::Paused`).
- Hold piece (store one swapped piece, swap on key).
- Wall kicks: on invalid rotate, try offsets `[(-1,0),(1,0),(0,-1),(-2,0),(2,0)]`
  before rejecting.
- Ghost piece: project active piece down to landing row, draw faint.
- 7-bag randomizer for spawn fairness.
- Sound / line-clear flash / smooth gravity (sub-cell position).

---

## Suggested commit granularity (going forward)
Each stage = 1 commit with `feat(...)` / `fix(...)` prefix, e.g.:
- `feat(board): add is_valid collision check`
- `feat(game): implement movement and rotation via try_offset`
- `feat(game): add gravity tick`
- `feat(game): lock piece into board`
- `feat(game): clear completed rows and score`
- `feat(game): spawn next piece and detect game over`
- `feat(renderer): draw HUD score/level/next`
- `feat(main): integrate input polling and loop`

## Verification checklist per stage
- `cargo build` clean (fix warnings as they appear).
- `cargo run` and visually confirm the specific behavior of that stage.
- For collision/movement: temporarily allow `is_key_pressed` debug print to
  confirm actions reach `handle_input`.
