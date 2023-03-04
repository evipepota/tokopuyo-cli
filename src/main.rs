use getch_rs::{Getch, Key};
use std::{thread, time};

mod draw;
mod game;
mod puyo;
mod union_find;

use draw::*;
use game::*;
use puyo::*;

fn main() {
    let mut field_buf = FIELD;

    let g = Getch::new();

    println!("\x1b[2J\x1b[H\x1b[?25l");

    loop {
        if check_end(&field_buf) {
            println!("\x1b[?25h");
            return;
        }

        let mut puyos = Puyos::new();

        loop {
            draw_field_with_tumo(&field_buf, &puyos);
            match g.getch() {
                Ok(Key::Char('a')) => {
                    // is_collisionがfalseならばpos.yを-1する。
                    let mut new_puyo1_pos = puyos.puyo_1.1;
                    let mut new_puyo2_pos = puyos.puyo_2.1;
                    new_puyo1_pos.y -= 1;
                    new_puyo2_pos.y -= 1;
                    if !is_collision(new_puyo1_pos) && !is_collision(new_puyo2_pos) {
                        puyos.puyo_1.1 = new_puyo1_pos;
                        puyos.puyo_2.1 = new_puyo2_pos;
                        draw_field_with_tumo(&field_buf, &puyos);
                    }
                }
                Ok(Key::Char('d')) => {
                    // is_collisionがfalseならばpos.yを+1する。
                    let mut new_puyo1_pos = puyos.puyo_1.1;
                    let mut new_puyo2_pos = puyos.puyo_2.1;
                    new_puyo1_pos.y += 1;
                    new_puyo2_pos.y += 1;
                    if !is_collision(new_puyo1_pos) && !is_collision(new_puyo2_pos) {
                        puyos.puyo_1.1 = new_puyo1_pos;
                        puyos.puyo_2.1 = new_puyo2_pos;
                        draw_field_with_tumo(&field_buf, &puyos);
                    }
                }
                Ok(Key::Left) => {
                    rotate_left(&mut puyos);
                    draw_field_with_tumo(&field_buf, &puyos);
                }
                Ok(Key::Right) => {
                    rotate_right(&mut puyos);
                    draw_field_with_tumo(&field_buf, &puyos);
                }
                Ok(Key::Down) => {
                    fall_floating_puyos_first(&mut field_buf, &puyos);
                    draw_field_without_tumo(&field_buf);
                    break;
                }
                Ok(Key::Char('s')) => {
                    fall_floating_puyos_first(&mut field_buf, &puyos);
                    draw_field_without_tumo(&field_buf);
                    break;
                }
                Ok(Key::Esc) => {
                    println!("\x1b[?25h");
                    return;
                }
                _ => (),
            }
        }

        loop {
            let mut check_loop_end: bool = true;
            check_loop_end &= !check_vanishing_puyo(&mut field_buf);
            thread::sleep(time::Duration::from_millis(500));
            check_loop_end &= !fall_floating_puyos_second(&mut field_buf);
            draw_field_without_tumo(&field_buf);
            if check_loop_end {
                break;
            }
            thread::sleep(time::Duration::from_millis(300));
        }
    }
}
