use sdl2::pixels::Color;
use sdl2::pixels::Palette;
use sdl2::pixels::PixelFormat;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::video::WindowContext;

use crate::tetris_piece::TetrisPieceType;
use crate::GameBoard;
use crate::TetrisPiece;

pub struct TetrisGameGraphics<'a> {
    canvas: &'a mut Canvas<Window>,
    sprite_width_mult: u32,
    sprite_height_mult: u32,
}

impl<'a> TetrisGameGraphics<'_> {
    pub fn new(
        canvas: &'a mut Canvas<Window>,
        sprite_width_mult: u32,
        sprite_height_mult: u32,
    ) -> TetrisGameGraphics<'a> {
        TetrisGameGraphics {
            canvas,
            sprite_width_mult,
            sprite_height_mult,
        }
    }

    pub fn draw_background(&mut self, elements: Vec<&Surface>) {
        for (_i, element) in elements.iter().enumerate() {
            self.canvas
                .copy(
                    &element.as_texture(&self.canvas.texture_creator()).unwrap(),
                    None,
                    None,
                )
                .unwrap();
        }
    }

    pub fn draw_game_board(
        &mut self,
        x_offset: i32,
        y_offset: i32,
        board: &GameBoard,
        sprite_sheet: &SpriteSheet,
    ) {
        for y in 0..board.height {
            for x in 0..board.width {
                let piece_id = board.grid[(x + y * board.width) as usize];
                if piece_id > 0 {
                    let sprite = &sprite_sheet.sprites[(piece_id - 1) as usize];
                    let dest_rect = Rect::new(
                        (x * 16 + x_offset) * self.sprite_width_mult as i32,
                        (y * 16 + y_offset) * self.sprite_height_mult as i32,
                        16 * self.sprite_width_mult,
                        16 * self.sprite_height_mult,
                    );
                    self.canvas.copy(&sprite.texture, None, dest_rect).unwrap();
                };
            }
        }
    }

    pub fn draw_piece(
        &mut self,
        x_offset: i32,
        y_offset: i32,
        piece: &TetrisPiece,
        sprite_sheet: &SpriteSheet,
    ) {
        for y in 0..5 {
            for x in 0..5 {
                let piece_id = piece.grid[(x + y * 5) as usize];
                if piece_id > 0 {
                    let sprite = &sprite_sheet.sprites[(piece_id - 1) as usize];
                    let dest_rect = Rect::new(
                        (x * 16 + x_offset) * self.sprite_width_mult as i32,
                        (y * 16 + y_offset) * self.sprite_height_mult as i32,
                        16 * self.sprite_width_mult,
                        16 * self.sprite_height_mult,
                    );
                    self.canvas.copy(&sprite.texture, None, dest_rect).unwrap();
                }
            }
        }
    }

    pub fn draw_hold_piece(
        &mut self,
        piece: &TetrisPiece,
        sprite_sheet: &SpriteSheet,
    ) {
        let mut fudge_x = 8;
        let mut fudge_y = 0;
        if piece.piece_type == TetrisPieceType::I {
            fudge_x = 16;
            fudge_y = 8;
        }
        if piece.piece_type == TetrisPieceType::O {
            fudge_x = 16;
            fudge_y = 0;
        }
        self.draw_piece(3 * 16 - fudge_x, 3 * 16 - fudge_y, &piece, &sprite_sheet);
    }

    pub fn draw_next_piece(&mut self, piece: &TetrisPiece, sprite_sheet: &SpriteSheet) {
      let mut fudge_x = 8;
      let mut fudge_y = 0;
      if piece.piece_type == TetrisPieceType::I {
          fudge_x = 16;
          fudge_y = 8;
      }
      if piece.piece_type == TetrisPieceType::O {
          fudge_x = 16;
          fudge_y = 0;
      }
      self.draw_piece(23 * 16 - fudge_x, 3 * 16 - fudge_y, &piece, &sprite_sheet);

    }

    pub fn draw_shine_effect(
        &mut self,
        x_offset: i32,
        y_offset: i32,
        piece: &TetrisPiece,
        sprite_sheet: &SpriteSheet,
        frame: i32,
    ) {
        for y in 0..5 {
            for x in 0..5 {
                let piece_id = piece.grid[(x + y * 5) as usize];
                if piece_id > 0 {
                    let sprite = &sprite_sheet.sprites[frame as usize];
                    let dest_rect = Rect::new(
                        (x * 16 + x_offset) * self.sprite_width_mult as i32,
                        (y * 16 + y_offset) * self.sprite_height_mult as i32,
                        16 * self.sprite_width_mult,
                        16 * self.sprite_height_mult,
                    );
                    self.canvas.copy(&sprite.texture, None, dest_rect).unwrap();
                }
            }
        }
    }

    pub fn draw_score(
        &mut self,
        x_offset: i32,
        y_offset: i32,
        score: i32,
        sprite_sheet: &SpriteSheet,
    ) {
        let score_str = format!("{:010}", score);
        for (i, c) in score_str.chars().enumerate() {
            let mut sprite_index = c.to_digit(10).unwrap() + 25;
            if c.to_digit(10).unwrap() == 0 {
                sprite_index = 35;
            }
            let sprite = &sprite_sheet.sprites[sprite_index as usize];
            let dest_rect = Rect::new(
                (x_offset + i as i32 * 6) * self.sprite_width_mult as i32,
                (y_offset + 6) * self.sprite_height_mult as i32,
                6 * self.sprite_width_mult,
                6 * self.sprite_height_mult,
            );
            self.canvas.copy(&sprite.texture, None, dest_rect).unwrap();
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}

pub struct Sprite<'a> {
    pub texture: sdl2::render::Texture<'a>,
    pub width: u32,
    pub height: u32,
}

impl Sprite<'_> {
    pub fn new(texture: sdl2::render::Texture) -> Sprite {
        let query = texture.query();
        Sprite {
            texture,
            width: query.width,
            height: query.height,
        }
    }
}

pub struct SpriteSheet<'a> {
    pub sprites: Vec<Sprite<'a>>,
}

impl<'a> SpriteSheet<'_> {
    pub fn new_from_surface_with_rect(
        texture_creator: &'a TextureCreator<WindowContext>,
        surface: &Surface,
        sprite_width: u32,
        sprite_height: u32,
        rect: Rect,
    ) -> SpriteSheet<'a> {
        let mut sprites: Vec<Sprite> = Vec::new();
        for y in 0..rect.height() / sprite_height {
            for x in 0..rect.width() / sprite_width {
                let src_rect = Rect::new(
                    (x * sprite_width) as i32 + rect.x,
                    (y * sprite_height) as i32 + rect.y,
                    sprite_width,
                    sprite_height,
                );
                let mut sprite_surface =
                    Surface::new(sprite_width, sprite_height, PixelFormatEnum::RGBA32).unwrap();
                //sprite_surface.set_palette(&Palette::from_surface(&surface)).unwrap();
                sprite_surface
                    .set_color_key(true, Color::RGBA(246, 153, 136, 255))
                    .unwrap();
                surface.blit(src_rect, &mut sprite_surface, None).unwrap();
                let sprite_texture = sprite_surface.as_texture(&texture_creator).unwrap();
                sprites.push(Sprite::new(sprite_texture));
            }
        }
        SpriteSheet { sprites }
    }
}

pub trait GetPalette {
    fn from_surface(surface: &Surface) -> Palette;
}

impl GetPalette for Palette {
    fn from_surface(surface: &Surface) -> Palette {
        let raw_palette: PixelFormat = surface.pixel_format();
        let c = unsafe { *(*raw_palette.raw()).palette };
        let mut colors: Vec<Color> = Vec::new();
        for i in 0..c.ncolors {
            let c = unsafe { *c.colors.offset(i as isize) };
            let color: Color = Color::RGBA(c.r, c.g, c.b, c.a);
            colors.push(color);
        }
        Palette::with_colors(&colors).unwrap()
    }
}
