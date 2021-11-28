




use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;




fn main() -> io::Result<()> {
    
    println!("Dime tu nombre perreque: ");
    
    let mut input = String::new();
    let ok = io::stdin().read_line(&mut input).expect("No se pudo leer la entrada");
    
    if ok == 0 {
        return Ok(());
    } 
      
    let path: &str = "src/lines.txt";

    let mut output: File = File::create(path)?;
    write!(output,"{}", input.trim());
 
    let input: File = File::open(path)?;
    let buffered: BufReader<File> = BufReader::new(input);
 
    for line in buffered.lines().map(|x| x.unwrap()) {
        // line: String     x:Result<String, Error>
        println!("{}", line);
    }
 
    Ok(())
}