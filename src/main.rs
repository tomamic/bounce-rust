pub mod actor;
pub mod bounce;
pub mod pt2d;
pub mod rand;


fn main() {
    let mut game = bounce::BounceGame::new(pt2d::pt(480, 360), 3, 2);
    for _ in 0..100 {
        game.tick(String::new());
        for b in game.actors() {
            println!("{:?}", b.pos());
        }
        println!();
    }
}
