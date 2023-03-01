const PUYO_COLOR: i32 = 4;
const PUYO_RED: i32 = 1;
const PUYO_BLUE: i32 = 2;
const PUYO_YELLOW: i32 = 3;
const PUYO_GREEN: i32 = 4;
const FIELD_WALL: i32 = 5;
const FIELD_SPACE: i32 = 0;

fn main() {
    let field = [
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

    let mut field_buf = field;

    for i in 0..14 {
        for j in 0..8 {
            if field[i][j] == FIELD_WALL {
                print!("[]");
            } else if field[i][j] == FIELD_SPACE {
                print!(" .")
            } else {
                print!(" {}", field[i][j]);
            }
        }
        println!();
    }
}
