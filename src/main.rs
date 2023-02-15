mod renderer;
mod sequence;
mod transformer;

mod line_sequence;

use minifb::{ScaleMode, Window, WindowOptions};
use sequence::Sequence;
use transformer::transform_sequence;
// https://github.com/emoon/rust_minifb/blob/master/examples/image.rs
fn main() {
    let mut frame_seq = Sequence::new("frames/frame.", ".png", 1, 6572);
    // Process

    // Render Output

    transform_sequence(&mut frame_seq);

    // Just use ffmpeg
    // // Make Window
    // let (width, height) = renderer::get_size();
    // let mut window = Window::new(
    //     "Image Test - Press ESC to exit",
    //     width,
    //     height,
    //     WindowOptions {
    //         resize: true,
    //         scale_mode: ScaleMode::Center,
    //         ..WindowOptions::default()
    //     },
    // )
    // .expect("Unable to open Window");

    // // Pipe to renderer
    // let mut out_seq = Sequence::new("out/frame.", ".png", 1, 6572);
    // // ~30 fps, though it won't reach that frame rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600 / 2)));
    // renderer::render_sequence(&mut window, &mut frame_seq);
}
