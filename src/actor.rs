use std::any::Any;
use std::collections::HashSet;

pub use crate::pt2d::*;


pub trait Actor {
    fn act(&mut self, arena: &mut ArenaStatus, others: &[&mut Box<dyn Actor>]);
    fn collide(&mut self, other: &dyn Actor, arena: &mut ArenaStatus);
    fn pos(&self) -> Pt;
    fn size(&self) -> Pt;
    fn sprite(&self) -> Option<Pt>;
    fn alive(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
}

pub struct ArenaStatus {
    spawned: Vec<Box<dyn Actor>>,
    size: Pt,
    count: i32,
    curr_keys: String,
    prev_keys: String
}
impl ArenaStatus {
    pub fn spawn(&mut self, b: Box<dyn Actor>) { self.spawned.push(b); }
    pub fn size(&self) -> Pt { self.size }
    pub fn count(&self) -> i32 { self.count }
    pub fn current_keys(&self) -> Vec<&str> { self.curr_keys.split(",").collect() }
    pub fn previous_keys(&self) -> Vec<&str> { self.prev_keys.split(",").collect() }
}


pub struct Arena {
    status: ArenaStatus,
    actors: Vec<Box<dyn Actor>>
}
impl Arena {
    pub fn new(size: Pt) -> Arena {
        Arena{
            status: ArenaStatus {
                spawned: vec![],
                size: size,
                count: 0,
                curr_keys: String::new(),
                prev_keys: String::new()
            },
            actors: vec![],
        }
    }
    pub fn check_collision(b1: &dyn Actor, b2: &dyn Actor) -> bool {
        let (tl1, br1) = (b1.pos(), b1.pos() + b1.size());
        let (tl2, br2) = (b2.pos(), b2.pos() + b2.size());
        (b1 as *const dyn Actor) != (b2 as *const dyn Actor)
            && tl2.x < br1.x && tl1.x < br2.x
            && tl2.y < br1.y && tl1.y < br2.y
    }
    pub fn tick(&mut self, keys: String) {
        self.status.prev_keys = self.status.curr_keys.to_string();
        self.status.curr_keys = keys;

        for i in 0..self.actors.len() {
            let (left, right) = self.actors.split_at_mut(i);
            let (middle, right) = right.split_at_mut(1);
            let b = &mut middle[0];
            let others = left.iter_mut().chain(right.iter_mut());
            b.act(&mut self.status, &others.collect::<Vec<_>>()[..]);
        }

        /*
        for i in (1..self.actors.len()).rev() {
            let (left, right) = self.actors.split_at_mut(i);
            let b1 = &mut right[0];
            for b2 in left.iter_mut() {
                if Arena::check_collision(&**b1, &**b2) {
                    b1.collide(&mut **b2, &mut self.status);
                    b2.collide(&mut **b1, &mut self.status);
                }
            }
        }
        */

        // divide the arena in tiles, for efficient collision detection
        let tile = pt(40, 40);
        let n = self.size() / tile;
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
        for i in (1..self.actors.len()).rev() {
            let (left, right) = self.actors.split_at_mut(i);
            let b1 = &mut right[0];
            let (tl, br) = (b1.pos() / tile, (b1.pos() + b1.size()) / tile);
            let mut neighs = HashSet::<usize>::new();
            for x in tl.x..=br.x {
                for y in tl.y..=br.y {
                    if 0 <= x && x < n.x &&  0 <= y && y < n.y {
                        neighs.extend(&cells[(y * n.x + x) as usize]);
                    }
                }
            }
            for j in neighs.into_iter().filter(|&j| j < i) {
                let b2 = &mut left[j];
                if Arena::check_collision(&**b1, &**b2) {
                    b1.collide(&mut **b2, &mut self.status);
                    b2.collide(&mut **b1, &mut self.status);
                }
            }
        }

        self.status.count += 1;
        self.actors.append(&mut self.status.spawned);
        self.status.spawned.clear();
        self.actors.retain(|b| b.alive());
    }
    pub fn spawn(&mut self, b: Box<dyn Actor>) { self.status.spawn(b); }
    pub fn actors(&self) -> &Vec<Box<dyn Actor>> { &self.actors }
    pub fn size(&self) -> Pt { self.status.size() }
    pub fn count(&self) -> i32 { self.status.count() }
}

//pub trait Actor { fn clone_dyn(&self) -> Box<dyn Actor>; }
//impl Actor for Ball { fn clone_dyn... { Box::new(self.clone()) } }
//impl Clone for Box<dyn Actor> { fn clone(&self) -> Self { self.clone_dyn() } }

