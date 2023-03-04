use rand::Rng;

pub const PUYO_COLOR: usize = 4;
pub const PUYO_RED: i32 = 1;
pub const PUYO_BLUE: i32 = 2;
pub const PUYO_YELLOW: i32 = 3;
pub const PUYO_GREEN: i32 = 4;
pub const FIELD_WALL: i32 = 5;
pub const FIELD_SPACE: i32 = 0;
pub const FIELD_NULL: i32 = -1;
pub const FIELD_WIDTH: usize = 6 + 2;
pub const FIELD_HEIGHT: usize = 12 + 2 + 3;
pub type Field = [[i32; FIELD_WIDTH]; FIELD_HEIGHT];

pub const FIELD: Field = [
    [
        FIELD_NULL, FIELD_NULL, FIELD_NULL, FIELD_NULL, FIELD_NULL, FIELD_NULL, FIELD_NULL,
        FIELD_NULL,
    ],
    [
        FIELD_NULL, FIELD_NULL, FIELD_NULL, FIELD_NULL, FIELD_NULL, FIELD_NULL, FIELD_NULL,
        FIELD_NULL,
    ],
    [
        FIELD_NULL, FIELD_NULL, FIELD_NULL, FIELD_NULL, FIELD_NULL, FIELD_NULL, FIELD_NULL,
        FIELD_NULL,
    ],
    [
        FIELD_WALL,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_WALL,
    ],
    [
        FIELD_WALL,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_WALL,
    ],
    [
        FIELD_WALL,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_WALL,
    ],
    [
        FIELD_WALL,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_WALL,
    ],
    [
        FIELD_WALL,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_WALL,
    ],
    [
        FIELD_WALL,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_WALL,
    ],
    [
        FIELD_WALL,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_WALL,
    ],
    [
        FIELD_WALL,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_WALL,
    ],
    [
        FIELD_WALL,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_WALL,
    ],
    [
        FIELD_WALL,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_WALL,
    ],
    [
        FIELD_WALL,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_WALL,
    ],
    [
        FIELD_WALL,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_WALL,
    ],
    [
        FIELD_WALL,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_SPACE,
        FIELD_WALL,
    ],
    [
        FIELD_WALL, FIELD_WALL, FIELD_WALL, FIELD_WALL, FIELD_WALL, FIELD_WALL, FIELD_WALL,
        FIELD_WALL,
    ],
];

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Puyos {
    pub puyo_1: (i32, Position),
    pub puyo_2: (i32, Position),
}

impl Puyos {
    pub fn new() -> Self {
        let (puyo_1, puyo_2) = Self::generate_puyo();
        let start_1: Position = Position { x: 1, y: 3 };
        let start_2: Position = Position { x: 2, y: 3 };
        return Puyos {
            puyo_1: (puyo_1, start_1),
            puyo_2: (puyo_2, start_2),
        };
    }

    fn generate_puyo() -> (i32, i32) {
        let mut rng = rand::thread_rng();
        let puyo_1 = match rng.gen_range(0..PUYO_COLOR) {
            0 => PUYO_RED,
            1 => PUYO_BLUE,
            2 => PUYO_YELLOW,
            _ => PUYO_GREEN,
        };
        let puyo_2 = match rng.gen_range(0..PUYO_COLOR) {
            0 => PUYO_RED,
            1 => PUYO_BLUE,
            2 => PUYO_YELLOW,
            _ => PUYO_GREEN,
        };
        return (puyo_1, puyo_2);
    }
}
