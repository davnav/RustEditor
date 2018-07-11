/// examples/raw.rs

extern crate libc;
extern crate termios;

use std::io::*;
use std::io::Read;
use std::io::Write;
use libc::{winsize, STDIN_FILENO, STDOUT_FILENO, TIOCGWINSZ};
use termios::{tcsetattr, Termios,TCSANOW, TCSAFLUSH, VMIN, VTIME};
use termios::{CS8, OPOST};
use termios::{BRKINT, ICRNL, INPCK, ISTRIP, IXON};
use termios::{ECHO, ICANON, IEXTEN, ISIG};

fn editor_refreshscreen(){
	
    std::io::stdout().write(b"\x1b[2J");
//    io::stdout().write(b"\x1b[H");
}

fn get_window_size()-> std::io::Result<(u16,u16)>{
	let ws = winsize{
	 	ws_col :0,
	 	ws_row :0,
	 	ws_xpixel:0,
	 	ws_ypixel:0,
	};
	unsafe{
	   if (libc::ioctl(STDOUT_FILENO,TIOCGWINSZ,&ws) == -1 || ws.ws_col ==0){
		return Err(Error::new(ErrorKind::Other,"get_window_size failed"));
	   }
	}
       Ok((ws.ws_row,ws.ws_col))
}
	  

fn editor_drawrows(){
   
   let mut i = 0;
   for i in 0..24{
	std::io::stdout().write(b"~\r\n");
   }

    std::io::stdout().write(b"\x1b[H");
}

			 	

fn main() {

    let stdin = 0;
    let stdout = 0;
    
    let mut termios = Termios::from_fd(stdin).unwrap();	
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO );
    new_termios.c_oflag &= !(OPOST);	
//    let mut termios_stdout = Termios::from(stdout).unwrap();
    editor_refreshscreen();
    let screen_size = get_window_size();	
    match screen_size{
	Ok((r,c)) => println!("row:{},column:{}",r,c),
	Err(e) =>  println!("{}",e),
    };    
    editor_drawrows(); 
    tcsetattr(stdin,TCSANOW, &mut new_termios).unwrap();
    let mut reader = std::io::stdin();
    let mut buffer = [0;1];
//    println!("Hit a key");
   
    while buffer[0] != 113{
	
    	reader.read_exact(&mut buffer).unwrap();
	println!("{:?}\r",buffer);
    } 	

	 
    tcsetattr(stdin,TCSANOW, &mut termios).unwrap();
    
   
}

