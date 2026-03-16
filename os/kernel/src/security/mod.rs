#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capability {
    MountFs,
    NetAdmin,
    DriverIo,
}

#[derive(Clone, Copy, Debug)]
pub struct SecurityContext {
    pub uid: u32,
    pub gid: u32,
    pub caps: [Option<Capability>; 8],
}

impl SecurityContext {
    pub const fn root() -> Self {
        Self {
            uid: 0,
            gid: 0,
            caps: [
                Some(Capability::MountFs),
                Some(Capability::NetAdmin),
                Some(Capability::DriverIo),
                None,
                None,
                None,
                None,
                None,
            ],
        }
    }

    pub fn can(&self, cap: Capability) -> bool {
        self.caps.iter().flatten().any(|c| *c == cap)
    }
}
