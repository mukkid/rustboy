use minifb::{Key, Window, WindowOptions};
include!("gameboy_logo_buffer.rs");


const WIDTH: usize = 160;
const HEIGHT: usize = 144;

pub fn render() {
    let mut window = Window::new(
        "Rustboy",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| panic!("{}", e));

    let mut buffer = vec![0; WIDTH * HEIGHT];
    // let example = vec![0xFF; WIDTH * HEIGHT];
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, &grey) in GAMEBOY_LOGO_SCREEN.iter().enumerate() {
            let rgb = (grey as u32) * 0x010101;
            buffer[i] = rgb;
        } 

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
     
}
