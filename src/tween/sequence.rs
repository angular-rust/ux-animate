// Utils
pub struct Sequence(usize);

impl Sequence {
    pub fn next_id(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}
