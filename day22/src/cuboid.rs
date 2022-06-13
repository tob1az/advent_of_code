use std::cmp::{min, max};

#[derive(Clone, Copy, PartialEq)]
pub enum Switch {
    On = 1,
    Off = -1,
}

#[derive(Clone)]
pub struct Cuboid {
    pub x_min: i64,
    pub x_max: i64,
    pub y_min: i64,
    pub y_max: i64,
    pub z_min: i64,
    pub z_max: i64,
    pub switch: Switch,
}

impl Cuboid {
    pub fn volume(&self) -> i64 {
        (self.x_max - self.x_min + 1)
            * (self.y_max - self.y_min + 1)
            * (self.z_max - self.z_min + 1)
            * (self.switch as i64)
    }

    pub fn intersects_with(&self, another: &Self) -> bool {
        self.x_min <= another.x_max && self.x_max >= another.x_min && 
        self.y_min <= another.y_max && self.y_max >= another.y_min && 
        self.z_min <= another.z_max && self.z_max >= another.z_min
    }

    pub fn intersection(&self, another: &Self) -> Self {
        Self {
            x_min: max(self.x_min, another.x_min),
            x_max: min(self.x_max, another.x_max),
            y_min: max(self.y_min, another.y_min),
            y_max: min(self.y_max, another.y_max),
            z_min: max(self.z_min, another.z_min),
            z_max: min(self.z_max, another.z_max),
            switch: match another.switch {
                Switch::On => Switch::Off,
                Switch::Off => Switch::On,
            }
        }
    }
}
