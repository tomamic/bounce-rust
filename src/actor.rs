use std::any::Any;
use std::collections::HashSet;

pub use crate::pt2d::*;


pub trait Actor {
    fn act(&mut self, arena: &mut ArenaStatus);
    fn pos(&self) -> Pt;
    fn size(&self) -> Pt;
    fn sprite(&self) -> Option<Pt>;
    fn alive(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
}

pub struct ArenaStatus<'a> {
    spawned: Vec<Box<dyn Actor>>,
    collisions: Vec<&'a mut Box<dyn Actor>>,
    others: Vec<&'a mut Box<dyn Actor>>,
    size: Pt,
    count: i32,
    curr_keys: String,
    prev_keys: String
}
impl ArenaStatus<'_> {
    pub fn spawn(&mut self, b: Box<dyn Actor>) { self.spawned.push(b); }
    pub fn size(&self) -> Pt { self.size }
    pub fn count(&self) -> i32 { self.count }
    pub fn current_keys(&self) -> Vec<&str> { self.curr_keys.split(",").collect() }
    pub fn previous_keys(&self) -> Vec<&str> { self.prev_keys.split(",").collect() }
    pub fn collisions(&self) -> &Vec<&mut Box<dyn Actor>> { &self.collisions }
    pub fn others(&self) -> &Vec<&mut Box<dyn Actor>> { &self.others }
}


pub struct Arena {
    size: Pt,
    count: i32,
    prev_keys: String,
    actors: Vec<Box<dyn Actor>>
}
impl Arena {
    pub fn new(size: Pt) -> Arena {
        Arena{
            size: size,
            count: 0,
            prev_keys: String::new(),
            actors: vec![],
        }
    }
    pub fn check_collision(b1: &dyn Actor, b2: &dyn Actor) -> bool {
        let (tl1, br1) = (b1.pos(), b1.pos() + b1.size());
        let (tl2, br2) = (b2.pos(), b2.pos() + b2.size());
        !std::ptr::addr_eq(b1 as *const dyn Actor, b2 as *const dyn Actor)
            && tl2.x < br1.x && tl1.x < br2.x
            && tl2.y < br1.y && tl1.y < br2.y
    }
    pub fn tick(&mut self, keys: String) {
        // divide the arena in tiles, for efficient collision detection
        let tile = pt(40, 40);
        let n = (self.size() + tile - pt(1, 1)) / tile;  // ceil
        let mut cells: Vec<HashSet<usize>> = vec![];
        for _ in 0..n.x * n.y { cells.push(HashSet::new()); }
        for (i, b) in self.actors.iter().enumerate() {
            let (tl, br) = (b.pos() / tile, (b.pos() + b.size()) / tile);
            for x in tl.x..=br.x {
                for y in tl.y..=br.y {
                    if 0 <= x && x < n.x &&  0 <= y && y < n.y {
                        cells[(y * n.x + x) as usize].insert(i);
                    }
                }
            }
        }
        let mut collisions: Vec<HashSet<usize>> = vec![];
        for (i, b) in self.actors.iter().enumerate() {
            let (tl, br) = (b.pos() / tile, (b.pos() + b.size()) / tile);
            let mut neighs = HashSet::<usize>::new();
            for x in tl.x..=br.x {
                for y in tl.y..=br.y {
                    if 0 <= x && x < n.x &&  0 <= y && y < n.y {
                        neighs.extend(&cells[(y * n.x + x) as usize]);
                    }
                }
            }
            neighs.remove(&i);
            neighs.retain(|j| Arena::check_collision(&**b, &*self.actors[*j]));
            collisions.push(neighs);
        }

        let mut spawned: Vec<Box<dyn Actor>> = vec![];
        for i in 0..self.actors.len() {
            let mut status = ArenaStatus {
                spawned: vec![],
                collisions: vec![],
                others: vec![],
                size: self.size,
                count: self.count,
                curr_keys: keys.to_string(),
                prev_keys: self.prev_keys.to_string()
            };
            let (left, right) = self.actors.split_at_mut(i);
            let (middle, right) = right.split_at_mut(1);
            let b = &mut middle[0];
            let others = left.iter_mut().chain(right.iter_mut());
            for (j, o) in others.enumerate() {
                if (&collisions[i]).contains(&(j + (j >= i) as usize)) {
                    status.collisions.push(o);
                } else {
                    status.others.push(o);
                }
            }
            b.act(&mut status);
            spawned.append(&mut status.spawned);
        }

        self.count += 1;
        self.actors.append(&mut spawned);
        self.actors.retain(|b| b.alive());
        self.prev_keys = keys.to_string();
    }
    pub fn spawn(&mut self, b: Box<dyn Actor>) { self.actors.push(b); }
    pub fn actors(&self) -> &Vec<Box<dyn Actor>> { &self.actors }
    pub fn size(&self) -> Pt { self.size }
    pub fn count(&self) -> i32 { self.count }
}

//pub trait Actor { fn clone_dyn(&self) -> Box<dyn Actor>; }
//impl Actor for Ball { fn clone_dyn... { Box::new(self.clone()) } }
//impl Clone for Box<dyn Actor> { fn clone(&self) -> Self { self.clone_dyn() } }

