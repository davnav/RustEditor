/// examples/raw.rs

extern crate libc;
extern crate termios;

use std::io;
use std::io::Read;
use std::io::Write;
use libc::{winsize, STDIN_FILENO, STDOUT_FILENO, TIOCGWINSZ};
use termios::{tcsetattr, Termios,TCSANOW, TCSAFLUSH, VMIN, VTIME};
use termios::{CS8, OPOST};
use termios::{BRKINT, ICRNL, INPCK, ISTRIP, IXON};
use termios::{ECHO, ICANON, IEXTEN, ISIG};


fn main() {

    let stdin = 0;
    let stdout = 0;
    let mut termios = Termios::from_fd(stdin).unwrap();	
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO );
    new_termios.c_oflag &= !(OPOST);	
//    let mut termios_stdout = Termios::from(stdout).unwrap();
    io::stdout().write(b"\x1b[2J");
    println!();
    io::stdout().write(b"\x1b[H");
 
    tcsetattr(stdin,TCSANOW, &mut new_termios).unwrap();
    let mut reader = io::stdin();
    let mut buffer = [0;1];
//    println!("Hit a key");
   
    while buffer[0] != 113{
	
    	reader.read_exact(&mut buffer).unwrap();
	println!("{:?}\r\n",buffer);
    } 	

	 
    tcsetattr(stdin,TCSANOW, &mut termios).unwrap();
    
   
}

