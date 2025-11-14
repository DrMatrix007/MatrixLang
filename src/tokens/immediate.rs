#[derive(Debug)]
pub enum Immediate {
    String(String),
    Number(Number),
}

#[derive(Debug)]
pub enum Number {
    I32(i32),
    F32(f32),
}
