use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
pub struct ByteHandler{
    pub bits: u8,
    pub bytes:Vec<u8>,
    pub count:u8,
}
impl ByteHandler{
    fn new()->Self{
        ByteHandler{
            bits:0,
            bytes:Vec::new(),
            count:0,
        }
    }
    pub fn upd_padding(&mut self, padding:u8){
        self.bytes[6]=padding;
    }
    pub fn flush(&mut self)->u8{
        let padding=(8-self.count)%8;
        for _ in 0..padding{
            self.bits = (self.bits<<1) | 0;
        }
        self.bytes.push(self.bits);
        self.bits=0;
        self.count=0;
        padding
    }
    pub fn add(&mut self, bit:u8){
        if self.count==8{
            self.flush();
        }
        self.bits=(self.bits<<1) | bit;
        self.count+=1;
    }
    pub fn write_bytes(&mut self, bytes:&[u8]){
        self.bytes.extend_from_slice(bytes);
    }
}
impl fmt::Display for ByteHandler{
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        write!(f,"bits:{} \n",self.bits );
        write!(f, "count{} \n", self.count);
        for byte in &self.bytes{
            write!(f,"{}\n", byte);
        }
        Ok(())
    }
}
pub fn file_handling(input: &str)->HashMap<u8,f32>{
    let file=File::open(input).expect("Failed to read file");
    let reader=BufReader::new(file);
    let mut byte_dict: HashMap<u8,f32> =HashMap::new();
    let mut total_bytes=0.0;
    for byte in reader.bytes(){
        total_bytes+=1.0;
        let byte=byte.expect("Failed to read bytes");
        let freq = match byte_dict.get(&byte){
            Some(freq)=>freq+1.0,
            None=>1.0,
        };
        byte_dict.insert(byte,freq);
    }
    for value in byte_dict.values_mut(){
        *value=*value/total_bytes;
    }
    byte_dict
}


pub fn write_to_output(input:&str, output:&str, seq: String, node_dict: HashMap<u8, Vec<u8>>){
    let file=File::open(input).expect("Failed to read file");
    let reader=BufReader::new(file);
    //4 bytes for the identifier
    let identifier= b"FITZ";
    //2 bytes for length of the sequence
    let len : u16 =seq.len() as u16;
    let bytes_be=len.to_be_bytes();
    //1 byte for padding
    let padding:u8 = 0;
    let mut bytehandler=ByteHandler::new();
    bytehandler.write_bytes(b"FITZ");
    bytehandler.write_bytes(&bytes_be);
    bytehandler.bytes.push(padding);
    for char in seq.chars(){
        bytehandler.bytes.push(char as u8);
    }
    for byte in reader.bytes(){
        let byte=byte.expect("Failed to read bytes");
        let code=node_dict.get(&byte).unwrap();
        for char in code{
            bytehandler.add(*char);
        }

    }
    let padding=bytehandler.flush();
    bytehandler.upd_padding(padding);

    let outputfile=File::create(output).expect("Failed to create file");
    let mut writer=BufWriter::new(outputfile);
    writer.write_all(&bytehandler.bytes).expect("Failed to write bytes");
    writer.flush().unwrap();
    



}