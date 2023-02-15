/// Generates a sequence of (x,y) coordinates that represent a line between pixel (x0, y0) to (x1,y1), in that order.
/// 
/// For this implementation, we use [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm).
pub fn plot_line(x0: u32, y0: u32, x1: u32, y1: u32) -> Vec<(u32, u32)> {
    if y1.abs_diff(y0) < x1.abs_diff(x0) {
        if x0 > x1 {
            let mut out = plot_line_low(x1, y1, x0, y0);
            out.reverse();
            return out;
        } else {
            plot_line_low(x0, y0, x1, y1)
        }
    } else {
        if y0 > y1 {
            let mut out = plot_line_high(x1, y1, x0, y0);
            out.reverse();
            return out;
        } else {
            plot_line_high(x0, y0, x1, y1)
        }
    }
}

fn plot_line_low(x0: u32, y0: u32, x1: u32, y1: u32) -> Vec<(u32, u32)> {
    let dx = (x1 - x0) as i32;
    let mut dy = y1 as i32 - y0 as i32;
    let mut delta = (2 * dy) - dx;
    let mut y = y0 as i32;
    let mut yi = 1i32;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut vec = Vec::new();
    for x in x0..=x1 {
        vec.push((x as u32, y as u32));
        if delta > 0 {
            y = y + yi;
            delta = delta + (2 * (dy - dx));
        } else {
            delta = delta + 2 * dy;
        }
    }
    return vec;
}

fn plot_line_high(x0: u32, y0: u32, x1: u32, y1: u32) -> Vec<(u32, u32)> {
    let mut dx = x1 as i32 - x0 as i32;
    let dy = (y1 - y0) as i32;
    let mut xi = 1i32;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }

    let mut delta = (2 * dx) - dy;
    let mut x = x0 as i32;

    let mut vec = Vec::new();
    for y in y0..y1 {
        vec.push((x as u32, y as u32));
        if delta > 0 {
            x = x + xi;
            delta = delta + (2 * (dx - dy))
        } else {
            delta = delta + 2 * dx
        }
    }

    return vec;
}
