mod sequence;
mod transformer;
mod line_sequence;

use sequence::Sequence;
use transformer::transform_sequence;

fn main() {
    let mut frame_seq = Sequence::new("frames/frame.", ".png", 1, 6572);

    transform_sequence(&mut frame_seq);
}
