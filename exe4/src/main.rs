
use core::panic;
/*
TODOs:
1 - Pegar argumentos da linha de commando
2-  Leitura de arquivos
3 - xxxx
*/
use std::env::{self, args};


enum Huffman {
    Leaf {c:char, freq:u32},
    Node {freq:u32,
    left :Box<Huffman>,
    right:Box<Huffman>}
}




fn main() {

    let args: Vec<String> =  env::args().collect();
    
    if args.len() > 3 {
        panic!("Argumento desconhecido"); 
    }

    if args[1] == "-c"{
        println!("Compactar: arquivo: {}", args[2]);    
    }else if args[1] == "-d"{
        println!("Descompactar: arquivo: {}", args[2]);    
    }else{
        panic!("Argumento desconhecido") // TODO: Melhorar mensagem com usage
    }

    
    println!("Args = {:?} ", args);
    


}
