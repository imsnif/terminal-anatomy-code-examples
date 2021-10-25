use nix::sys::termios;
use std::io::{self, Read};
use std::io::prelude::*;

fn set_raw_mode() {
    let mut tio = termios::tcgetattr(0).expect("could not get terminal attribute");
    termios::cfmakeraw(&mut tio);
    match termios::tcsetattr(0, termios::SetArg::TCSANOW, &tio) {
        Ok(_) => {}
        Err(e) => panic!("error {:?}", e),
    };
}

fn main() {
    set_raw_mode();
    let mut buffer = [0; 1];
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        print!("\n\r");
        print!("Would you like to quit [y/n]? ");
        io::stdout().lock().flush().unwrap();
        handle.read_exact(&mut buffer).unwrap();
        if buffer == [121] { // y
            break;
        }
    }
}
