/// examples/raw.rs

extern crate libc;
extern crate termios;
use std::fs::File;
use std::io::prelude::*;
use std::str;
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
	  

fn editor_drawrows(r:u16){
   
   let mut i = 0;
   for i in 0..r{
	std::io::stdout().write(b"~\r\n");
   }

    std::io::stdout().write(b"\x1b[H");
}

fn editor_read_key()-> std::vec::Vec<u8>{
		
        let mut reader = std::io::stdin();
	let mut buffer = vec![0;1]; 
    	reader.read(&mut buffer);
	buffer


}
	
	
fn editor_process_key(){
     let mut text_rows = String::new();	
     let mut c = editor_read_key();
	
//	println!("{}",c);
//     let mut s = match str::from_utf8(&mut c){
//	Ok(v) => { if  v != "q"{
//			 println!("{}", v)
//		 }},
//	Err(e) => panic!("invalid UTF-8 :{}",e),
//     };
     while c[0] as char != 'q' {
	text_rows.push(c[0] as char);
	c = editor_read_key();
//	s = match str::from_utf8(&c){
//		Ok(v) => v,	
//		Err(e) => panic!("invalid UTF-8:{}",e),
//	};
	print!("{}",c[0] as char);
	std::io::stdout().flush();
//	println!("{}\r",text_rows);
    }
   editor_file_write(text_rows); 
//	println!("{}\r",text_rows);
//        editor_read_key();
    editor_exit();     
}

fn editor_file_write(text1:String) -> std::io::Result<()>{
	let mut file = File::create("foo1.txt")?;
	file.write_all(text1.as_bytes())?;
	Ok(())
}

fn init_screen(termios:&termios::Termios){
     
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO );
    new_termios.c_oflag &= !(OPOST);	
//    let mut termios_stdout = Termios::from(stdout).unwrap();
    tcsetattr(0,TCSANOW, &mut new_termios).unwrap();
    
    editor_refreshscreen();
    let screen_size = get_window_size();	
    match screen_size{
	Ok((r,c)) => editor_drawrows(r),
	Err(e) =>  println!("{}",e),
     
    };

}

fn editor_exit(){
					 	
	std::io::stdout().write(b"\x1b[H");
	std::io::stdout().write(b"\x1b[2J");
}


fn main() {

    let stdin = 0;
    let stdout = 0;
    let mut termios = Termios::from_fd(stdin).unwrap();	
    init_screen(&termios);	
    editor_process_key();
    editor_exit();
    
    tcsetattr(stdin,TCSANOW, &mut termios).unwrap();
}

