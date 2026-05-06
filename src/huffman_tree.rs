use std::collections::HashMap;
use std::fmt;
#[derive(Debug)]
#[derive(Clone)]
pub enum Node{
    Freq(FreqNode),
    Int(Internal),
}
#[derive(Clone)]

#[derive(Debug)]
pub struct FreqNode{
    pub freq: f32,
    pub byte:u8,
}
impl FreqNode{
    pub fn new(byte:u8)->Self{
        FreqNode{
            freq:0.0,
            byte,
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]

pub struct Internal{
    pub freq:f32,
    pub left: Box<Node>,
    pub right: Box<Node>,
}
impl Internal{
    pub fn new()->Self{
        Internal{
            freq:0.0,
            left: Box::new(Node::Freq(FreqNode::new(0))),
            right: Box::new(Node::Freq(FreqNode::new(0))),
        }
    }
}
impl Node{
    pub fn get_freq(&self)->f32{
        match self{
            Node::Freq(f) => f.freq,
            Node::Int(i)=>i.freq,
        }
    }
    pub fn build_dict(&self, seq:&mut String, code:Vec<u8>,  node_dict:&mut HashMap<u8,Vec<u8>>){
        match self{
            Node::Freq(f)=>{
                seq.push('1');
                seq.push(f.byte as char);
                node_dict.insert(f.byte ,code);},
            Node::Int(i)=>{
                seq.push('0');
                let left=&*i.left;
                let right=&*i.right;
                let mut left_code=code.clone();
                left_code.push(0);
                let mut right_code=code.clone();
                right_code.push(1);
                left.build_dict(seq,left_code,node_dict);
                right.build_dict(seq,right_code,node_dict); }
        }
    }
}
impl fmt::Display for Node{
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
        match self{
            Node::Freq(fah)=>write!(f,"f{}", fah.freq),
            Node::Int(i)=>write!(f,"i{}",i.freq),
        };
        Ok(())
    }
}
#[derive(Debug)]
pub struct MinHeap{
    tree: Vec<Node>
}
impl MinHeap{
    pub fn new()->MinHeap{
        let tree=Vec::new();
        MinHeap{tree }
    }

    pub fn push (&mut self, node:Node){
        self.tree.push(node);

        let mut idx=self.tree.len()-1;
        if idx==0{
            return;
        }
        while (idx>0) && (self.tree[(idx-1)/2].get_freq()>self.tree[idx].get_freq()){
            self.tree.swap(idx,(idx-1)/2);
            idx=(idx-1)/2;
        }
    }
    pub fn pop(&mut self)->Option<Node>{

        let len=self.tree.len();
        if len==0{
            return None;
        }
        self.tree.swap(0,len-1);
        let popped=self.tree.pop();
        let mut idx=0;
        loop{
            let len=self.tree.len();
            if idx>=len{
                break;
            }
            let left=2*idx+1;
            let right=2*idx+2;
            let mut smallest=idx;
            if left<len && self.tree[left].get_freq()<self.tree[smallest].get_freq(){
                smallest=left
            }
            else if right<len && self.tree[right].get_freq()<self.tree[smallest].get_freq(){
                smallest=right
            }
            else{
                break;
            }
            if right < len && self.tree[right].get_freq()<self.tree[left].get_freq(){
                smallest=right;
            }
            self.tree.swap(idx, smallest);
            idx=smallest;

        }
        popped

    }

}
impl fmt::Display for MinHeap{
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
        for node in self.tree.iter(){
            write!(f,"{}", node);
        }
        Ok(())
    }
}

pub fn huffman_seq(byte_dict: HashMap<u8,f32> )-> (String, HashMap<u8,Vec<u8>>){
    let mut byteheap=MinHeap::new();
    for (byte,freq) in byte_dict{
        let node=Node::Freq(FreqNode{
            freq,
            byte,
        });
        byteheap.push(node);
    }
    let mut seq=String::new();
    let mut code=Vec::new();
    let mut node_dict=HashMap::new();
    if byteheap.tree.len()==1{
        code.push(0);
    }
    while byteheap.tree.len()>1{
        let pop1=byteheap.pop().unwrap();
        let pop2=byteheap.pop().unwrap();
        let intn=Node::Int(Internal{
            freq:pop1.get_freq()+pop2.get_freq(),
            left:Box::new(pop1),
            right:Box::new(pop2),
        });
        byteheap.push(intn);
    }
    let root=byteheap.pop().unwrap();

    root.build_dict(&mut seq, code,&mut node_dict);
    (seq,node_dict)
}