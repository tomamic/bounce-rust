use wasm_bindgen::prelude::*;

use crate::pt2d::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: String);
    pub fn set_color(r: i32, g: i32, b: i32);
    pub fn clear_canvas();
    pub fn update_canvas();
    pub fn current_keys() -> String;
    pub fn previous_keys() -> String;
    pub fn mouse_clicked() -> bool;
    pub fn load_image(src: String) -> String;
    pub fn main_loop(fps: i32);
    pub fn close_canvas();

    fn js_init_canvas(w: i32, h: i32);
    fn js_fill_circle(x: i32, y: i32, r: i32);
    fn js_fill_rect(x: i32, y: i32, w: i32, h: i32);
    fn js_draw_text(txt: String, x: i32, y: i32, size: i32, baseline: String, align: String);
    fn js_draw_image(src: String, x: i32, y: i32, xc: i32, yc: i32, wc: i32, hc: i32);
    fn js_mouse_x() -> i32;
    fn js_mouse_y() -> i32;
}

pub fn init_canvas(size: Pt) {
    js_init_canvas(size.x, size.y);
}
pub fn fill_circle(pos: Pt, r: i32) {
    js_fill_circle(pos.x, pos.y, r);
}
pub fn fill_rect(pos: Pt, size: Pt) {
    js_fill_rect(pos.x, pos.y, size.x, size.y);
}
pub fn draw_image(src: String, pos: Pt) {
    js_draw_image(src, pos.x, pos.y, 0, 0, 0, 0);
}
pub fn draw_image_clip(src: String, pos: Pt, clip: Pt, size: Pt) {
    js_draw_image(src, pos.x, pos.y, clip.x, clip.y, size.x, size.y);
}
pub fn draw_text(txt: String, pos: Pt, size: i32) {
    js_draw_text(txt, pos.x, pos.y, size, String::from("top"), String::from("left"));
}
pub fn draw_text_centered(txt: String, pos: Pt, size: i32) {
    js_draw_text(txt, pos.x, pos.y, size, String::from("middle"), String::from("center"));
}
pub fn mouse_pos() -> Pt {
    pt(js_mouse_x(), js_mouse_y())
}
