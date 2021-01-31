extern crate ada;

use minifb::{Key, Window, WindowOptions};

use ada::Canvas;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            topmost: true,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~30 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(33333)));

    // create canvas
    let mut cbuffer: Vec<u8> = vec![0; 4 * WIDTH * HEIGHT];
    let _canvas = Canvas::new(WIDTH, HEIGHT, &mut cbuffer[..]);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, pix) in buffer.iter_mut().enumerate() {
            let r = 255;
            let g = 255;
            let b = 0;
            if i > 10 && i < 30 {
                *pix = (r << 16) | (g << 8) | b;
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
