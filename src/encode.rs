use std::collections::HashMap;
use std::fs::{write, File};
use crate::huffman_tree::huffman_seq;
use crate::file_handling::{file_handling, write_to_output};
use std::io::{BufReader, BufWriter, Write,Read};
pub fn encode(input:&str, output:&str){
    let byte_dict=file_handling(input);
    let (seq,node_dict)=huffman_seq(byte_dict);
    write_to_output(input, output,seq, node_dict);
}

