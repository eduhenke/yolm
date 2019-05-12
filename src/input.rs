// taken from https://github.com/conradkdotcom/rpassword/blob/master/src/lib.rs
// :x

use libc::{c_int, isatty, tcgetattr, tcsetattr, TCSANOW, ECHO, ECHONL};
use std::io::{BufReader, BufRead};
use std::io;
use std::os::unix::io::AsRawFd;
use std::fs::File;

/// Turns a C function return into an IO Result
fn io_result(ret: c_int) -> io::Result<()> {
    match ret {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error()),
    }
}

pub fn read_password_from_tty() -> io::Result<String> {
    let (tty_f, mut tty_rd) = {
        let tty = File::open("/dev/tty")?;
        (tty.as_raw_fd(), BufReader::new(tty))
    };

    let is_tty = unsafe { isatty(tty_f) } == 1;
    if !is_tty {
        panic!("Program should be run in a TTY");
    }

    // we'll want to hide the password as it is typed by the user
    // Make two copies of the terminal settings. The first one will be modified
    // and the second one will act as a backup for when we want to set the
    // terminal back to its original state.
    let mut term = unsafe { ::std::mem::uninitialized() };
    let mut term_orig = unsafe { ::std::mem::uninitialized() };
    io_result(unsafe { tcgetattr(tty_f, &mut term) })?;
    io_result(unsafe { tcgetattr(tty_f, &mut term_orig) })?;

    // Hide the password. This is what makes this function useful.
    term.c_lflag &= !ECHO;

    // But don't hide the NL character when the user hits ENTER.
    term.c_lflag |= ECHONL;

    // Save the settings for now.
    io_result(unsafe { tcsetattr(tty_f, TCSANOW, &term) })?;

    // Read the password.
    let mut password = String::new();
    tty_rd.read_line(&mut password)?;

    // Reset the terminal
    io_result(unsafe { tcsetattr(tty_f, TCSANOW, &term_orig) })?;

    // Remove the newline
    password.pop();

    Ok(password)
}