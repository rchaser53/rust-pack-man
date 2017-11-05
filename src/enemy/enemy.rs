use enemy::enemy_feature::{
  EnemyAction,
  NormalFeature
};
use enemy::enemy_status::{
  EnemyStatus,
  EnemyStatusFactory,
  NormalEnemyStatusFactory
};

pub struct Enemy {
  pub status: EnemyStatus,
  pub feature: Box<EnemyAction> 
}

impl Enemy {
  pub fn new(status: EnemyStatus, feature: Box<EnemyAction>) -> Enemy {
    Enemy {
      status: status,
      feature: feature
    }
  }
}

pub struct EnemyCreater {}
impl EnemyCreater {
  pub fn create_enemy(row_def: &str, row_index: usize) -> Vec<Enemy> {
    let mut enemies: Vec<Enemy> = Vec::new();
    let cell_defs: Vec<char> = row_def.chars().collect();
    
    enemies = cell_defs.iter().enumerate().fold(enemies, |mut stack, (cell_index, map_char)| {
      match *map_char {
        '1' => { 
          stack.push( Enemy::new(Box::new(NormalEnemyStatusFactory{}).create_enemy_status(row_index, cell_index),
                      Box::new(NormalFeature {})));
         },
        _ => {}
      };
      stack
    });

    enemies
  }
}