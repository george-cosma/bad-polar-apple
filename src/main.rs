mod sequence;
mod transformer;
mod line_sequence;

use std::thread;

use sequence::Sequence;
use transformer::transform_sequence;

fn main() {
    // let mut frame_seq = Sequence::new("frames/frame.", ".png", 487, 488); //6572);
    let threads = thread::available_parallelism().unwrap().get();
    let frames = 6572;
    let frames_per_thread = frames / threads;

    let mut handles = Vec::new();
    
    for i in 0..(threads - 1) {
        let handle = thread::spawn(move || {
            let mut frame_seq = Sequence::new("frames/frame.", ".png", 
            i * frames_per_thread + 1, (i + 1) * frames_per_thread);
            transform_sequence(&mut frame_seq);
        });

        handles.push(handle);
    }

    let handle = thread::spawn(move || {
        let mut frame_seq = Sequence::new("frames/frame.", ".png", (threads - 1) * frames_per_thread + 1, frames);
        transform_sequence(&mut frame_seq);
    });

    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }
}
