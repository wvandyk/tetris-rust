extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadSurface};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use tetris_piece::TetrisPieceType;
use std::time::Duration;

mod game_board;
mod tetris_game_graphics;
mod tetris_piece;
use crate::game_board::GameBoard;
use crate::tetris_game_graphics::{SpriteSheet, TetrisGameGraphics};
use crate::tetris_piece::{TetrisPiece, TetrisPieceState};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let sdl_timer = sdl_context.timer()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let mut render_width = 480;
    let mut render_height = 400;
    let mut sprite_width_mult = 1;
    let mut sprite_height_mult = 1;
    let line_scores = vec![0, 100, 200, 400, 800];

    let display_mode = video_subsystem.current_display_mode(0).unwrap();
    if display_mode.w > render_width && display_mode.h > render_height {
        sprite_width_mult = 2;
        sprite_height_mult = 2;
        render_width *= 2;
        render_height *= 2;
    }

    let window = video_subsystem
        .window("Clonetris", render_width as u32, render_height as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut last_ticks: u32;

    let texture_creator = canvas.texture_creator();
    let spritesheet_surface = Surface::from_file("base_gfx.png").map_err(|e| e.to_string())?;
    let background_surface = Surface::from_file("background_gfx.png").map_err(|e| e.to_string())?;
    let foreground_surface = Surface::from_file("foreground_gfx.png").map_err(|e| e.to_string())?;
    let sprite_sheet = SpriteSheet::new_from_surface_with_rect(
        &texture_creator,
        &spritesheet_surface,
        16,
        16,
        Rect::new(0, 0, 16 * 7, 16),
    );
    let font_sheet = SpriteSheet::new_from_surface_with_rect(
        &texture_creator,
        &spritesheet_surface,
        6,
        6,
        Rect::new(0, 16, 6 * 36, 6),
    );
    let shine_effect_sheet = SpriteSheet::new_from_surface_with_rect(
        &texture_creator,
        &spritesheet_surface,
        16,
        16,
        Rect::new(0, 64, 16 * 7, 16),
    );

    let mut board: GameBoard = GameBoard::new(10, 22);
    let mut t = TetrisPiece::new_random_piece();
    let mut t_next = TetrisPiece::new_random_piece();
    let mut t_hold = TetrisPiece::new(TetrisPieceType::None);
    let mut gfx = TetrisGameGraphics::new(&mut canvas, sprite_width_mult, sprite_height_mult);
    let drop_time: u32 = 30;
    let place_time: u32 = 100;
    let mut drop_timer: u32 = drop_time;
    let mut place_timer: u32 = place_time;
    let mut score = 0;
    let mut shine_frame = 0;
    let mut delta_time: u32;
    'running: loop {
        let current_ticks: u32 = sdl_timer.ticks();
        last_ticks = current_ticks;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    t.translate(&board, -1, 0);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    t.translate(&board, 1, 0);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    t.translate(&board, 0, 1);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    t.srs_rotate(&board, true);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    t.srs_rotate(&board, false);
                }
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    if t.state == TetrisPieceState::Placing {
                        continue;
                    }
                    if t_hold.piece_type == TetrisPieceType::None {
                        t_hold = t;
                        t = t_next;
                        t_next = TetrisPiece::new_random_piece();
                    } else {
                        t_hold.y = t.y;
                        t_hold.x = t.x;
                        
                        let temp = t_hold;
                        t_hold = t;
                        for _i in 0..t_hold.rotation {
                            t_hold.rotate_ccw();
                        }
                        t = temp;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    while t.state == TetrisPieceState::Active {
                        let old_y = t.y;
                        t.translate(&board, 0, 1);
                        if t.y == old_y {
                            t.state = TetrisPieceState::Placing;
                        }
                    }
                }
                _ => {}
            }
        }

        drop_timer -= 1;
        if t.state == TetrisPieceState::Placing {
            place_timer -= 1;
        }

        if drop_timer == 0 {
            let old_y = t.y;
            t.translate(&board, 0, 1);
            if t.y == old_y {
                t.state = TetrisPieceState::Placing;
            } else {
                t.state = TetrisPieceState::Active;
                place_timer = place_time;
            }
            drop_timer = drop_time;
        }

        if place_timer == 0 && t.state == TetrisPieceState::Placing {
            t.state = TetrisPieceState::Placed;
            board.place_piece(&t);
            let full_lines = board.full_lines();

            score += line_scores[full_lines.len() as usize];

            for line in full_lines {
                for y in (0..line).rev() {
                    for x in 0..10 {
                        board.grid[(x + (y + 1) * 10) as usize] = board.grid[(x + y * 10) as usize];
                    }
                }
                for x in 0..10 {
                    board.grid[x] = 0;
                }
            }

            t = t_next;
            t_next = TetrisPiece::new_random_piece();
            if !board.test_placement(&t) {
                break 'running;
            }
            place_timer = place_time;
        }

        gfx.draw_background(vec![&background_surface]);
        gfx.draw_game_board(160, 16, &board, &sprite_sheet);
        gfx.draw_piece(160 + (&t.x * 16), 16 + (&t.y * 16), &t, &sprite_sheet);
        if &t.state == &TetrisPieceState::Placing {
            gfx.draw_shine_effect(
                160 + (&t.x * 16),
                16 + (&t.y * 16),
                &t,
                &shine_effect_sheet,
                shine_frame,
            );
        }
        
        gfx.draw_next_piece(&t_next, &sprite_sheet);
        gfx.draw_hold_piece(&t_hold, &sprite_sheet);

        gfx.draw_background(vec![&foreground_surface]);
        gfx.draw_score(23 * 16 + 3, 10 * 16 - 1, score, &font_sheet);
        gfx.present();

        let current_ticks: u32 = sdl_timer.ticks();
        delta_time = current_ticks - last_ticks;
        shine_frame = 6 - ((place_timer as i32 / 10) % 7);
        ::std::thread::sleep(Duration::new(
            0,
            (1_000_000_000u32 - (delta_time % 1_000) * 1_000_000u32) / 200,
        ));
    }

    Ok(())
}
