use minifb::{Key, Window, WindowOptions};
use crate::gpu::{Tile, Color};
include!("gameboy_logo_buffer.rs");


const WIDTH: usize = 160;
const HEIGHT: usize = 144;

pub fn render(fbuf: &[Tile]) {
    let mut window = Window::new(
        "Rustboy",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| panic!("{}", e));

    let mut buffer = vec![0; WIDTH * HEIGHT];
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, tile) in fbuf.iter().enumerate() {
            for (j, c) in tile.pixels.iter().enumerate() {
                buffer[i*64 + j] = match c {
                    Color::White => 0x00,
                    Color::LGray => 0x55,
                    Color::DGray => 0xAA,
                    Color::Black => 0xFF,
                } 
            }
        }
        // for (i, &grey) in GAMEBOY_LOGO_SCREEN.iter().enumerate() {
        //     let rgb = (grey as u32) * 0x010101;
        //     buffer[i] = rgb;
        // } 

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
     
}
