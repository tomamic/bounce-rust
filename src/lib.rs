use wasm_bindgen::prelude::*;
use std::cell::RefCell;

pub mod actor;
pub mod bounce;
pub mod g2d;
pub mod pt2d;
pub mod rand;

pub struct BounceGui {
    game: bounce::BounceGame
}
impl BounceGui {
    pub fn new() -> BounceGui {
        let game = bounce::BounceGame::new(pt2d::pt(480, 360), 3, 2);
        BounceGui{game}
    }
    pub fn setup(&self) {
        g2d::init_canvas(self.game.size());
        g2d::main_loop(30);
    }
    pub fn tick(&mut self) {
        self.game.tick(g2d::current_keys());  // Game logic

        g2d::clear_canvas();
        let actors = self.game.actors();
        for b in actors.iter() {
            if let Some(img) = b.sprite() {
                g2d::draw_image_clip("sprites.png".to_string(), b.pos(), img, b.size());
            } else {
                //g2d::fill_rect(b.pos(), b.size());
            }
        }
        let txt = format!("Lives: {} Time: {}",
            self.game.remaining_lives(), self.game.remaining_time());
        g2d::draw_text(txt, pt2d::pt(0, 0), 24);

        if self.game.game_over() {
            g2d::alert("Game over".to_string());
            g2d::close_canvas();
        } else if self.game.game_won() {
            g2d::alert("Game won".to_string());
            g2d::close_canvas();
        }
    }
}

thread_local! {
    static GUI: RefCell<BounceGui> = RefCell::new(BounceGui::new());
}

#[wasm_bindgen]
pub fn tick() {
    GUI.with(|g| {
        g.borrow_mut().tick();
    });
}

#[wasm_bindgen]
pub fn setup() {
    GUI.with(|g| {
        g.borrow_mut().setup();
    });
}
