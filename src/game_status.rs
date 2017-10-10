pub struct GameStatus {
    pub is_pause: bool,
    pub is_clear: bool,
    pub score: i16,
}

impl GameStatus {
  pub fn new() -> GameStatus {
    return GameStatus {
      is_pause: false,
      is_clear: false,
      score: 0
    };
  }
}