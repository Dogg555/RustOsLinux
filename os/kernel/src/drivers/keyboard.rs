use x86::io::inb;

pub fn handle_interrupt() {
    let scancode = unsafe { inb(0x60) };
    println!("kbd scancode: 0x{:02x}", scancode);
}
