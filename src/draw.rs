use crate::puyo::*;

pub fn draw_field_with_tumo(field: &Field, puyos: &Puyos) {
    println!("\x1b[H");
    for i in 0..FIELD_HEIGHT {
        for j in 0..FIELD_WIDTH {
            if i == puyos.puyo_1.1.x && j == puyos.puyo_1.1.y {
                print!(" {}", puyos.puyo_1.0);
            } else if i == puyos.puyo_2.1.x && j == puyos.puyo_2.1.y {
                print!(" {}", puyos.puyo_2.0);
            } else if i == 4 && j == 3 && field[i][j] == FIELD_SPACE {
                print!(" x");
            } else if field[i][j] == FIELD_NULL {
                print!("  ");
            } else if field[i][j] == FIELD_WALL {
                print!("[]");
            } else if field[i][j] == FIELD_SPACE {
                print!(" .")
            } else {
                print!(" {}", field[i][j]);
            }
        }
        println!();
    }
    println!();
}

pub fn draw_field_without_tumo(field: &Field) {
    println!("\x1b[H");
    for i in 0..FIELD_HEIGHT {
        for j in 0..FIELD_WIDTH {
            if i == 4 && j == 3 && field[i][j] == FIELD_SPACE {
                print!(" x");
            } else if field[i][j] == FIELD_NULL {
                print!("  ");
            } else if field[i][j] == FIELD_WALL {
                print!("[]");
            } else if field[i][j] == FIELD_SPACE {
                print!(" .")
            } else {
                print!(" {}", field[i][j]);
            }
        }
        println!();
    }
    println!();
}
