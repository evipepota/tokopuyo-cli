extern crate rand;

use std::{thread, time};

use getch_rs::{Getch, Key};
use rand::Rng;

const PUYO_COLOR: usize = 4;
const PUYO_RED: i32 = 1;
const PUYO_BLUE: i32 = 2;
const PUYO_YELLOW: i32 = 3;
const PUYO_GREEN: i32 = 4;
const FIELD_WALL: i32 = 5;
const FIELD_SPACE: i32 = 0;
const FIELD_NULL: i32 = -1;
const FIELD_WIDTH: usize = 6 + 2;
const FIELD_HEIGHT: usize = 12 + 2 + 3;
type Field = [[i32; FIELD_WIDTH]; FIELD_HEIGHT];

struct UnionFind {
    par: Field,
    siz: [[usize; FIELD_WIDTH]; FIELD_HEIGHT],
}

impl UnionFind {
    fn new() -> Self {
        let mut par: Field = [[0; FIELD_WIDTH]; FIELD_HEIGHT];
        for i in 0..FIELD_HEIGHT {
            for j in 0..FIELD_WIDTH {
                par[i][j] = (i * FIELD_WIDTH + j) as i32;
            }
        }
        UnionFind {
            par,
            siz: [[1; FIELD_WIDTH]; FIELD_HEIGHT],
        }
    }

    fn root(&mut self, x: usize, y: usize) -> i32 {
        if self.par[x][y] == (x * FIELD_WIDTH + y) as i32 {
            return (x * FIELD_WIDTH + y) as i32;
        }
        self.par[x][y] = self.root(
            self.par[x][y] as usize / FIELD_WIDTH,
            self.par[x][y] as usize % FIELD_WIDTH,
        );
        self.par[x][y]
    }

    fn issame(&mut self, pos1: Position, pos2: Position) -> bool {
        self.root(pos1.x, pos1.y) == self.root(pos2.x, pos2.y)
    }

    fn unite(&mut self, parent: Position, child: Position) -> bool {
        let mut parent2 = self.root(parent.x, parent.y);
        let mut child2 = self.root(child.x, child.y);

        if parent2 == child2 {
            return false;
        }

        self.par[child2 as usize / FIELD_WIDTH][child2 as usize % FIELD_WIDTH] = parent2;
        self.siz[parent2 as usize / FIELD_WIDTH][parent2 as usize % FIELD_WIDTH] +=
            self.siz[child.x][child.y];
        true
    }

    fn size(&mut self, pos: Position) -> usize {
        let root = self.root(pos.x, pos.y);
        self.siz[root as usize / FIELD_WIDTH][root as usize % FIELD_WIDTH]
    }
}

fn check_vanishing_puyo(field: &mut Field) -> bool {
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
                uf.unite(Position { x: i, y: j }, Position { x: i + 1, y: j });
            }
            if field[i][j] == field[i - 1][j] {
                uf.unite(Position { x: i, y: j }, Position { x: i - 1, y: j });
            }
            if field[i][j] == field[i][j + 1] {
                uf.unite(Position { x: i, y: j }, Position { x: i, y: j + 1 });
            }
            if field[i][j] == field[i][j - 1] {
                uf.unite(Position { x: i, y: j }, Position { x: i, y: j - 1 });
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

fn fall_floating_puyos_first(field: &mut Field, puyos: &Puyos) {
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

fn fall_floating_puyos_second(field: &mut Field) -> bool {
    let mut check: bool = false;
    for j in 1..FIELD_WIDTH {
        let mut k = FIELD_HEIGHT - 2;
        for i in (0..FIELD_HEIGHT - 2).rev() {
            if field[i][j] != FIELD_SPACE {
                if i != k {
                    check = true;
                }
                field[k][j] = field[i][j];
                k -= 1;
            }
        }
    }
    return check;
}

fn check_end(field: &Field) -> bool {
    todo!()
}

fn generate_puyo() -> (i32, i32) {
    let mut rng = rand::thread_rng();
    let puyo_1 = rng.gen_range(0..PUYO_COLOR) as i32 + 1;
    let puyo_2 = rng.gen_range(0..PUYO_COLOR) as i32 + 1;
    return (puyo_1, puyo_2);
}

fn is_collision(puyo_pos: Position) -> bool {
    // 左の判定
    if puyo_pos.y == 0 {
        return true;
    }
    // 右の判定
    if puyo_pos.y == (FIELD_WIDTH - 1) {
        return true;
    }
    // 下の判定 todo

    false
}

fn print_field(field: &Field, puyos: &Puyos) {
    println!("\x1b[H");
    for i in 0..FIELD_HEIGHT {
        for j in 0..FIELD_WIDTH {
            if i == puyos.puyo_1.1.x && j == puyos.puyo_1.1.y {
                print!(" {}", puyos.puyo_1.0);
            } else if i == puyos.puyo_2.1.x && j == puyos.puyo_2.1.y {
                print!(" {}", puyos.puyo_2.0);
            } else if i == 4 && j == 3 {
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

fn print_field_no_puyo(field: &Field) {
    println!("\x1b[H");
    for i in 0..FIELD_HEIGHT {
        for j in 0..FIELD_WIDTH {
            if i == 4 && j == 3 {
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

#[derive(Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

struct Puyos {
    puyo_1: (i32, Position),
    puyo_2: (i32, Position),
}

impl Puyos {
    fn new() -> Self {
        let (puyo_1, puyo_2) = generate_puyo();
        let start_1: Position = Position { x: 1, y: 3 };
        let start_2: Position = Position { x: 2, y: 3 };
        return Puyos {
            puyo_1: (puyo_1, start_1),
            puyo_2: (puyo_2, start_2),
        };
    }
}

fn main() {
    let field: Field = [
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

    let mut field_buf = field;

    let g = Getch::new();

    println!("\x1b[2J\x1b[H\x1b[?25l");

    loop {
        //if check_end(&field) {
        //break;
        //}
        // Generate and Manipulate puyos at this location.
        let mut puyo = Puyos::new();

        loop {
            print_field(&field_buf, &puyo);
            match g.getch() {
                Ok(Key::Left) => {
                    // is_collisionがfalseならば
                    // pos.yを-1する。
                    let mut new_puyo1_pos = puyo.puyo_1.1;
                    let mut new_puyo2_pos = puyo.puyo_2.1;
                    new_puyo1_pos.y -= 1;
                    new_puyo2_pos.y -= 1;
                    if !is_collision(new_puyo1_pos) && !is_collision(new_puyo2_pos) {
                        puyo.puyo_1.1 = new_puyo1_pos;
                        puyo.puyo_2.1 = new_puyo2_pos;
                        print_field(&field_buf, &puyo);
                    }
                }
                Ok(Key::Right) => {
                    // is_collisionがfalseならば
                    // pos.yを+1する。
                    let mut new_puyo1_pos = puyo.puyo_1.1;
                    let mut new_puyo2_pos = puyo.puyo_2.1;
                    new_puyo1_pos.y += 1;
                    new_puyo2_pos.y += 1;
                    if !is_collision(new_puyo1_pos) && !is_collision(new_puyo2_pos) {
                        puyo.puyo_1.1 = new_puyo1_pos;
                        puyo.puyo_2.1 = new_puyo2_pos;
                        print_field(&field_buf, &puyo);
                    }
                }
                Ok(Key::Char('\r')) => {
                    fall_floating_puyos_first(&mut field_buf, &puyo);
                    print_field_no_puyo(&field_buf);
                    break;
                }
                _ => (),
            }
        }

        loop {
            let mut check_loop_end: bool = true;
            check_loop_end &= !check_vanishing_puyo(&mut field_buf);
            thread::sleep(time::Duration::from_millis(500));
            check_loop_end &= !fall_floating_puyos_second(&mut field_buf);
            print_field_no_puyo(&field_buf);
            if check_loop_end {
                break;
            }
            thread::sleep(time::Duration::from_millis(300));
        }
    }
}
