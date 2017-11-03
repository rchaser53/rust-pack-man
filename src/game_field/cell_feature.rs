use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{render, video, rect};

use game_field::cell_status::{CellStatus};
use constants::{BackgroundColor};

const ITEM_RADIUS: i16 = 10;
const CELL_PADDING: i32 = 15;

pub trait DrawMyself {
    fn draw(&self, cell_status: &CellStatus, renderer: &mut render::Canvas<video::Window>) {
        renderer.set_draw_color(cell_status.background_color);

        let rect = rect::Rect::new(cell_status.x, cell_status.y, cell_status.width, cell_status.height);
        let _ = renderer.fill_rect(rect);

        self.draw_unique_feature(cell_status, renderer);
    }

    #[allow(unused)]
    fn draw_unique_feature(&self, cell_status: &CellStatus, renderer: &mut render::Canvas<video::Window>) {}
}

pub struct NormalFeature {}
impl DrawMyself for NormalFeature {}

#[allow(unused)]
pub struct DamageFeature {}
impl DrawMyself for DamageFeature {}

pub struct BlockFeature {}
impl DrawMyself for BlockFeature {}

pub struct WallFeature {}
impl DrawMyself for WallFeature {}

pub struct ItemFeature {}
impl DrawMyself for ItemFeature {
    fn draw_unique_feature(&self, cell_status: &CellStatus, renderer: &mut render::Canvas<video::Window>) {
        if !cell_status.exist_item { return; }
        let _ = renderer.filled_circle((cell_status.x + CELL_PADDING) as i16,
                  (cell_status.y + CELL_PADDING) as i16, ITEM_RADIUS, BackgroundColor::Yellow.value());
    }
}