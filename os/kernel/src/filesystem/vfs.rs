use super::ramfs;

pub fn init() {
    ramfs::initialize();

    let _ = ramfs::with_fs(|fs| {
        let etc = fs.mkdir(0, "etc");
        let cfg = fs.create_file(etc, "aurora.conf");
        let _ = fs.write_file(cfg, b"kernel.log=info\nnet=enabled\n");
    });
}
