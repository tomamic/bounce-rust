use std::cell::RefCell;

#[derive(Copy, Clone)]
pub struct Rng {
    random: u32
}
impl Rng {
    pub fn from_addr() -> Rng {
        let num = vec![1, 2, 3];
        let address = &num as *const Vec<i32>;
        Rng{random: address as u32}
    }
    pub fn from_time() -> Rng {
        use std::time::SystemTime as st;
        let now = st::now().duration_since(st::UNIX_EPOCH).unwrap();
        Rng{random: now.as_millis() as u32}
    }
    pub fn randint(&mut self, nmin: i32, nmax: i32) -> i32 {
        // From "Xorshift RNGs" by George Marsaglia
        self.random ^= self.random << 13;
        self.random ^= self.random >> 17;
        self.random ^= self.random << 5;
        nmin + (self.random % (nmax + 1 - nmin) as u32) as i32
    }
}

thread_local! {
    static RNG: RefCell<Rng> = RefCell::new(Rng::from_addr());
}

pub fn randint(nmin: i32, nmax: i32) -> i32 {
    let mut result = 0;
    RNG.with(|rng| {
        result = rng.borrow_mut().randint(nmin, nmax);
    });
    result
}
