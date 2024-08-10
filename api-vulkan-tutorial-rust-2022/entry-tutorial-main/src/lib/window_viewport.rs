pub const WINDOW_VIEWPORT_LOGICAL_NUMBER_MAX: usize = 4;


pub struct WindowViewportLogicalNumber(usize);

impl WindowViewportLogicalNumber {
    pub fn new(number: usize) -> Self {
        Self(number)
    }

    pub fn as_raw(&self) -> usize {
        self.0
    }

    pub fn increase_one(&mut self) -> () {
        match self.0 < WINDOW_VIEWPORT_LOGICAL_NUMBER_MAX {
            true => self.0 += 1,
            false => (),
        }
    }

    pub fn decrease_one(&mut self) -> () {
        match 1 < self.0 {
            true => self.0 -= 1,
            false => (),
        }
    }
}

pub struct WindowViewportLogicalIndex(usize);

impl WindowViewportLogicalIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn as_raw(&self) -> usize {
        self.0
    }
}