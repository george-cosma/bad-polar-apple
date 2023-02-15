pub struct Sequence<'a> {
    pub prefix: &'a str,
    pub suffix: &'a str,
    start: usize,
    end: usize,
    current: usize,
}

impl Sequence<'_> {
    pub fn new<'a>(prefix: &'a str, suffix: &'a str, start: usize, end: usize) -> Sequence<'a> {
        Sequence {
            prefix: prefix,
            suffix: suffix,
            start: start,
            end: end,
            current: start,
        }
    }
    pub fn next(&mut self) -> Option<usize> {
        if self.current == self.end + 1 {
            return None;
        }

        let output = self.current;
        self.current += 1;
        return Some(output);
    }

    pub fn restart(&mut self) {
        self.current = self.start;
    }
}
