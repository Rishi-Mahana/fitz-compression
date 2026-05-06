use crate::huffman_tree::*;
use std::collections::VecDeque;
use std::fs;


pub fn decode(input: &str, output: &str){
    let vec_bytes=fs::read(input).expect("Failed to read file");
    let identifier=(vec_bytes[0], vec_bytes[1],vec_bytes[2],vec_bytes[3]);
    if identifier!=(70,73,84,90){
        eprintln!("Not encoded using FITZ encoding");
        return;
    }
    let padding=vec_bytes[6];
    let be_seq_len1=vec_bytes[4];
    let be_seq_len2=vec_bytes[5];
    let seq_len:u16=u16::from_be_bytes([be_seq_len1, be_seq_len2]);
    let mut seq=build_sequence(&vec_bytes,seq_len);
    let mut root=build_tree(&mut seq);
    let decoded=byte_decoding(vec_bytes, seq_len,padding,root);
    fs::write(output, decoded).expect("Failed to write");


}
pub fn build_sequence(bytes:&Vec<u8>, seq_len:u16)->VecDeque<u8>{
    let mut count=7;
    let mut seq=VecDeque::new();
    while count<seq_len+7{
        seq.push_back(bytes[count as usize]);
        count+=1;

    }
    seq
}
pub fn build_tree(seq: &mut VecDeque<u8>) -> Node {
    let node_type = seq.pop_front().unwrap();
    if node_type == b'0' {
        let left = build_tree(seq);
        let right = build_tree(seq);
        Node::Int(Internal {
            freq: 0.0,
            left: Box::new(left),
            right: Box::new(right),
        })
    } else {
        let byte = seq.pop_front().unwrap();
        Node::Freq(FreqNode { freq: 0.0, byte })
    }
}
pub fn byte_decoding(vec_bytes:Vec<u8>,seq_len:u16, padding:u8, root:Node)->String{
    let total_bits=(vec_bytes.len()-7-(seq_len as usize))*8-(padding as usize);
    let mut byte_idx=7+seq_len;
    let mut bit_idx=0;
    let mut byte_count=7;
    let mut curr_byte=vec_bytes[byte_idx as usize];
    let mut curr_node=&root;
    let mut decoded=String::new();
    match curr_node{
        Node::Freq(f)=>{
            let byte=f.byte;
            let decoded=(byte as char).to_string().repeat(total_bits);
            return decoded;
        },
        _=>{},
    };
    while bit_idx<total_bits{
        if byte_count==-1{
            byte_idx+=1;
            curr_byte=vec_bytes[byte_idx as usize];
            byte_count=7;
        }
        let curr_bit=(curr_byte>>byte_count as u32)&1;
        match curr_node{
            Node::Int(i)=>{
            if curr_bit==1{
                let next_node=&*i.right;
                match next_node{
                    Node::Int(_)=>{curr_node=next_node;},
                    Node::Freq(f)=>{decoded.push(f.byte as char); curr_node=&root}
                }
            }
            else{
                let next_node=&*i.left;
                match next_node{
                    Node::Int(_)=>{curr_node=next_node;},
                    Node::Freq(f)=>{decoded.push(f.byte as char); curr_node=&root;}
                }
            }
            }
            Node::Freq(_)=>{panic!("Current node in decoding traversal is a frequency node");

            }
        }

        bit_idx+=1;
        byte_count-=1;
    };
    decoded
}