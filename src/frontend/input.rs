pub trait Input {
    fn new() -> Self;
    fn input_loop(&mut self) -> (Option<Chip8KeyCode>, bool);
    fn decode_input(input: Chip8KeyCode) -> usize {
        match input {
            Chip8KeyCode::One => 1,
            Chip8KeyCode::Two => 2,
            Chip8KeyCode::Three => 3,
            Chip8KeyCode::C => 0xc,
            Chip8KeyCode::Four => 4,
            Chip8KeyCode::Five => 5,
            Chip8KeyCode::Six => 6,
            Chip8KeyCode::D => 0xd,
            Chip8KeyCode::Seven => 7,
            Chip8KeyCode::Eight => 8,
            Chip8KeyCode::Nine => 9,
            Chip8KeyCode::E => 0xe,
            Chip8KeyCode::A => 0xa,
            Chip8KeyCode::Zero => 0,
            Chip8KeyCode::B => 0xb,
            Chip8KeyCode::F => 0xf,
            Chip8KeyCode::Exit => 0,
        }
    }
}

pub enum Chip8KeyCode {
    One,
    Two,
    Three,
    C,
    Four,
    Five,
    Six,
    D,
    Seven,
    Eight,
    Nine,
    E,
    A,
    Zero,
    B,
    F,
    Exit
}