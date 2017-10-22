use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{render, video, rect, pixels};

use game_field::field_cell::{CellStatus, CellType};
use constants::{BackgroundColor};

const ITEM_RADIUS: i16 = 10;
const CELL_PADDING: i32 = 15;

pub fn draw_myself(cell_status: &CellStatus, renderer: &mut render::Canvas<video::Window>) {
    let color = convert_cell_type_to_background_color(cell_status.cell_type);
    renderer.set_draw_color(color as pixels::Color);

    let rect = rect::Rect::new(cell_status.x, cell_status.y, cell_status.width, cell_status.height);
    let _ = renderer.fill_rect(rect);
}

pub fn convert_cell_type_to_background_color(cell_type: CellType) -> pixels::Color {
  match cell_type {
    CellType::Normal => BackgroundColor::Black.value(),
    CellType::Block => BackgroundColor::Aqua.value(),
    CellType::Damage => BackgroundColor::Aqua.value(),
    CellType::Wall => BackgroundColor::Gray.value(),
    CellType::Item => BackgroundColor::Black.value()
  }
}

pub struct Item {}
impl Item {
    fn draw_unique_feature(cell_status: &CellStatus, renderer: &mut render::Canvas<video::Window>) {
        if !cell_status.exist_item { return; }
        let _ = renderer.filled_circle((cell_status.x + CELL_PADDING) as i16,
                  (cell_status.y + CELL_PADDING) as i16, ITEM_RADIUS, BackgroundColor::Yellow.value());
    }
}

pub struct CellFeature {}
impl CellFeature {
    pub fn draw(cell_status: &CellStatus, renderer: &mut render::Canvas<video::Window>) {
        draw_myself(cell_status, renderer);

        match cell_status.cell_type {
            CellType::Item => { Item::draw_unique_feature(cell_status, renderer); },
            _ => {}
        }
    }
}