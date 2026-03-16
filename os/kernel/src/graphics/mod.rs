#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct Window {
    pub id: usize,
    pub bounds: Rect,
    pub z: usize,
}

pub struct Compositor {
    windows: [Option<Window>; 16],
    next_id: usize,
}

impl Compositor {
    pub const fn new() -> Self {
        Self {
            windows: [None; 16],
            next_id: 1,
        }
    }

    pub fn create_window(&mut self, bounds: Rect) -> Option<usize> {
        let slot = self.windows.iter().position(Option::is_none)?;
        let id = self.next_id;
        self.next_id += 1;
        self.windows[slot] = Some(Window {
            id,
            bounds,
            z: slot,
        });
        Some(id)
    }
}
