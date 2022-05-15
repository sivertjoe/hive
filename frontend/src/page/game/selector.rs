use super::util::*;

pub struct Selector {
    pub index: usize,
    pub max: usize,
    pub dir: Option<Key>,
}

impl Selector {
    pub fn get_and_update_index(&mut self, key: Key) -> Option<usize> {
        if self.max == 0 {
            return match key {
                Key::Left => match self.dir {
                    None => {
                        self.dir = Some(Key::Left);
                        Some(0)
                    }
                    Some(Key::Left) => None,
                    Some(Key::Right) => {
                        self.dir = Some(Key::Left);
                        Some(0)
                    }
                },

                Key::Right => match self.dir {
                    None => None,
                    Some(Key::Left) => {
                        self.dir = Some(Key::Right);
                        Some(0)
                    }
                    Some(Key::Right) => None,
                },
            };
        }


        match key {
            Key::Left => match self.dir {
                None => {
                    if self.index == self.max {
                        self.dir = Some(key);
                        Some(self.index)
                    } else {
                        // Some(self.index)
                        None
                    }
                }
                Some(Key::Left) => {
                    self.index -= 1;
                    if self.index == 0 {
                        self.dir = None;
                    }
                    Some(self.index)
                }
                Some(Key::Right) => {
                    self.dir = Some(Key::Left);
                    if self.index == 0 {
                        self.dir = None;
                    }
                    Some(self.index)
                }
            },

            Key::Right => match self.dir {
                Some(Key::Left) => {
                    self.dir = Some(Key::Right);
                    if self.index == self.max {
                        self.dir = None;
                    }
                    Some(self.index)
                }
                Some(Key::Right) => {
                    self.index += 1;
                    self.dir = Some(Key::Right);
                    if self.index == self.max {
                        self.dir = None;
                    }
                    Some(self.index)
                }
                None => {
                    if self.index == 0 {
                        self.dir = Some(Key::Right);
                        Some(self.index)
                    } else {
                        None
                    }
                }
            },
        }
    }
}
