#[derive(Copy, Clone)]
pub enum Placement {
    Absent, // Not in the word
    Present, // In the word but wrong position
    Correct, // In the correct position
    Unknown, // Not yet analyzed
}

#[derive(Copy, Clone)]
pub struct Character {
    character: char,
    placement: Placement,
    index: usize,
}

impl Character {
    pub fn new(character: char, placement: Placement, index: usize) -> Self {
        Character { character, placement, index }
    }

    fn update_placement(&mut self, new_placement: Placement, new_index: usize) {
        self.placement = new_placement;
        self.index = new_index;
    }
}