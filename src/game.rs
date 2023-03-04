use crate::puyo::*;
use crate::union_find::*;

pub fn check_vanishing_puyo(field: &mut Field) -> bool {
    // Given the game board of the field, eliminate the connected Puyos of 4 or more by matching them.
    let mut check = false;
    let mut uf = UnionFind::new();
    for i in (1..FIELD_HEIGHT - 1).rev() {
        for j in 1..FIELD_WIDTH - 1 {
            if field[i][j] == FIELD_NULL || field[i][j] == FIELD_SPACE || field[i][j] == FIELD_WALL
            {
                continue;
            }
            if field[i][j] == field[i + 1][j] {
                uf.unite(
                    &mut Position { x: i, y: j },
                    &mut Position { x: i + 1, y: j },
                );
            }
            if field[i][j] == field[i - 1][j] {
                uf.unite(
                    &mut Position { x: i, y: j },
                    &mut Position { x: i - 1, y: j },
                );
            }
            if field[i][j] == field[i][j + 1] {
                uf.unite(
                    &mut Position { x: i, y: j },
                    &mut Position { x: i, y: j + 1 },
                );
            }
            if field[i][j] == field[i][j - 1] {
                uf.unite(
                    &mut Position { x: i, y: j },
                    &mut Position { x: i, y: j - 1 },
                );
            }
        }
    }
    for i in (1..FIELD_HEIGHT - 1).rev() {
        for j in 1..FIELD_WIDTH - 1 {
            if uf.size(Position { x: i, y: j }) >= 4 {
                // ここで直接消すのでスコア計算機能を実装するとしたらここ。
                field[i][j] = FIELD_SPACE;
                check = true;
            }
        }
    }

    check
}

pub fn fall_floating_puyos_first(field: &mut Field, puyos: &Puyos) {
    // This function drops the floating Puyos and organizes the game board. It returns a boolean value indicating whether the game board has changed or not.
    let puyo1_pos: Position = puyos.puyo_1.1;
    let puyo2_pos: Position = puyos.puyo_2.1;
    if puyo1_pos.y == puyo2_pos.y {
        if puyo1_pos.x < puyo2_pos.x {
            // puyo2が下の方にあるので先に落とす。
            for i in (3..FIELD_HEIGHT).rev() {
                if field[i][puyo2_pos.y] == FIELD_SPACE {
                    field[i][puyo2_pos.y] = puyos.puyo_2.0;
                    break;
                }
            }
            for i in (3..FIELD_HEIGHT).rev() {
                if field[i][puyo1_pos.y] == FIELD_SPACE {
                    field[i][puyo1_pos.y] = puyos.puyo_1.0;
                    break;
                }
            }
        } else {
            for i in (3..FIELD_HEIGHT).rev() {
                if field[i][puyo1_pos.y] == FIELD_SPACE {
                    field[i][puyo1_pos.y] = puyos.puyo_1.0;
                    break;
                }
            }
            for i in (3..FIELD_HEIGHT).rev() {
                if field[i][puyo2_pos.y] == FIELD_SPACE {
                    field[i][puyo2_pos.y] = puyos.puyo_2.0;
                    break;
                }
            }
        }
    } else {
        for i in (3..FIELD_HEIGHT).rev() {
            if field[i][puyo1_pos.y] == FIELD_SPACE {
                field[i][puyo1_pos.y] = puyos.puyo_1.0;
                break;
            }
        }
        for i in (3..FIELD_HEIGHT).rev() {
            if field[i][puyo2_pos.y] == FIELD_SPACE {
                field[i][puyo2_pos.y] = puyos.puyo_2.0;
                break;
            }
        }
    }
}

pub fn fall_floating_puyos_second(field: &mut Field) -> bool {
    let mut check: bool = false;
    for j in 1..FIELD_WIDTH - 1 {
        let mut k = FIELD_HEIGHT - 2;
        for i in (0..FIELD_HEIGHT - 1).rev() {
            if field[i][j] == FIELD_NULL {
                continue;
            }
            if field[i][j] != FIELD_SPACE {
                field[k][j] = field[i][j];
                if i != k {
                    field[i][j] = FIELD_SPACE;
                    check = true;
                }
                k -= 1;
            }
        }
    }
    return check;
}

pub fn check_end(field: &Field) -> bool {
    if field[4][3] != FIELD_SPACE {
        return true;
    }
    false
}

pub fn is_collision(puyo_pos: Position) -> bool {
    // 左の判定
    if puyo_pos.y == 0 {
        return true;
    }
    // 右の判定
    if puyo_pos.y == (FIELD_WIDTH - 1) {
        return true;
    }
    false
}

pub fn rotate_right(puyos: &mut Puyos) {
    if puyos.puyo_2.1.x == 0 {
        puyos.puyo_2.1.x += 1;
        puyos.puyo_2.1.y += 1;
    } else if puyos.puyo_2.1.x == 2 {
        puyos.puyo_2.1.x -= 1;
        puyos.puyo_2.1.y -= 1;
    } else if puyos.puyo_1.1.y < puyos.puyo_2.1.y {
        puyos.puyo_2.1.x += 1;
        puyos.puyo_2.1.y -= 1;
    } else {
        puyos.puyo_2.1.x -= 1;
        puyos.puyo_2.1.y += 1;
    }
    if puyos.puyo_2.1.y == 0 {
        puyos.puyo_1.1.y += 1;
        puyos.puyo_2.1.y += 1;
    }
    if puyos.puyo_2.1.y == FIELD_WIDTH - 1 {
        puyos.puyo_1.1.y -= 1;
        puyos.puyo_2.1.y -= 1;
    }
}

pub fn rotate_left(puyos: &mut Puyos) {
    if puyos.puyo_2.1.x == 0 {
        puyos.puyo_2.1.x += 1;
        puyos.puyo_2.1.y -= 1;
    } else if puyos.puyo_2.1.x == 2 {
        puyos.puyo_2.1.x -= 1;
        puyos.puyo_2.1.y += 1;
    } else if puyos.puyo_1.1.y < puyos.puyo_2.1.y {
        puyos.puyo_2.1.x -= 1;
        puyos.puyo_2.1.y -= 1;
    } else {
        puyos.puyo_2.1.x += 1;
        puyos.puyo_2.1.y += 1;
    }
    if puyos.puyo_2.1.y == 0 {
        puyos.puyo_1.1.y += 1;
        puyos.puyo_2.1.y += 1;
    }
    if puyos.puyo_2.1.y == FIELD_WIDTH - 1 {
        puyos.puyo_1.1.y -= 1;
        puyos.puyo_2.1.y -= 1;
    }
}
