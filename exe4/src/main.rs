
/*
TODOs:
1 - Pegar argumentos da linha de commando
2-  Leitura de arquivos
3 - xxxx
*/


enum Huffman {
    Leaf {c:char, freq:u32},
    Node {freq:u32,
    left :Box<Huffman>,
    right:Box<Huffman>}
}


fn main() {
    println!("Hello, world!");
}
