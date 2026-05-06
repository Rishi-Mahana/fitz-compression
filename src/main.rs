mod encode;
mod huffman_tree;
mod file_handling;
mod decode;
use decode::*;
use clap::Parser;
use encode::*;
#[derive(Parser)]
struct Args{
    input:String,
    output:String,
    mode:String,
}
fn input(){


}

fn main() {
    let args = Args::parse();
    
    match args.mode.as_str() {
        "encode" => {encode(&args.input,&args.output)},
        "decode" => {decode(&args.input,&args.output)},
        _ => { eprintln!("jana")},

    };
}