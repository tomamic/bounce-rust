mod actor;
mod bounce;
mod pt2d;
mod rand;


fn main() {
    let mut game = bounce::BounceGame::new(480, 360);
    for _ in 0..100 {
        game.tick(String::new());
        for b in game.actors() {
            println!("{:?}", b.pos());
        }
        println!();
    }
}
