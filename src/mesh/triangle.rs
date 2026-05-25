
pub struct Triangle {
    pub v: [usize; 3],
    pub color: u32
}

impl Triangle {
    pub fn new(v0: usize, v1:usize, v2:usize, color: u32)->Self {
        Self {
            v: [v0, v1, v2],
            color
        }
    }
}