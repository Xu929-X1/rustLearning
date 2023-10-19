use std::{io::{self, stdout, Read, Error}, process};
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111 //control sets the upper 3 bits to 0
}

fn die(e: Error){
    panic!(e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        match b {
            Ok(b)=>{
                let c = b as char;
                if c.is_control(){
                    println!("{:?} \r", c);
                }else {
                    println!("{:?} ({}) \r", b, c )
                }
            }
            Err(err)=>die(err)
        }
    }
}
