const PUYO_COLOR: usize = 4;
const PUYO_RED: i32 = 1;
const PUYO_BLUE: i32 = 2;
const PUYO_YELLOW: i32 = 3;
const PUYO_GREEN: i32 = 4;
const FIELD_WALL: i32 = 5;
const FIELD_SPACE: i32 = 0;
const FIELD_WIDTH: usize = 6 + 2;
const FIELD_HEIGHT: usize = 12 + 2;
type Field = [[i32; FIELD_WIDTH]; FIELD_HEIGHT];

fn check_vanishing_puyo(field: &mut Field) -> bool {
    // Given the game board of the field, eliminate the connected Puyos of 4 or more by matching them.
    todo!()
}

fn fall_floating_puyos(field: &mut Field) -> bool {
    // This function drops the floating Puyos and organizes the game board. It returns a boolean value indicating whether the game board has changed or not.
    todo!()
}

fn check_end(field: Field) -> bool {
    todo!()
}

fn main() {
    let field: Field = [
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

    for i in 0..FIELD_HEIGHT {
        for j in 0..FIELD_WIDTH {
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

    loop {
        if check_end(field) {
            break;
        }
        // Generate and Manipulate puyos at this location.

        loop {
            let mut check_loop_end: bool = true;
            check_loop_end &= !fall_floating_puyos(&mut field_buf);
            check_loop_end &= !check_vanishing_puyo(&mut field_buf);
            if check_loop_end {
                break;
            }
        }
    }
}
