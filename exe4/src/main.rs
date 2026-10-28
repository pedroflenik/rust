
use core::panic;
/*
TODOs:
1 - Pegar argumentos da linha de commando
2-  Leitura de arquivos
3 - xxxx

Funções necessárias
- Função para ler arquivo normal
- Função para ler arquivo compactado
- Função para escrever arquivo normal
- Função para escrever arquivo compactado
- Função para criar a árvore de huffman

*/
use std::env::{self, args};
use std::fs::{self, File};
use std::collections::HashMap;
use std::hash::Hash;
use std::io::{self, Read};

enum Huffman {
    Leaf {c:char, freq:u32},
    Node {freq:u32,
    left :Box<Huffman>,
    right:Box<Huffman>}
}



fn leitura_arquivo_normal(nome_arquivo: &str) -> Result<String,io::Error>{
    let mut conteudo: String = String::new();
    let mut arquivo = File::open(nome_arquivo)?;
    arquivo.read_to_string(&mut conteudo)?;
    Ok(conteudo)
}


fn compactar(nome_arquivo: &str){
    println!("Compactando arquivo: {}", nome_arquivo); 

    let string_arquivo = match leitura_arquivo_normal(nome_arquivo) {
        Ok(contents) => {contents}
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            String::new()
        }
    };


}

fn main() {

    let args: Vec<String> =  env::args().collect();
    

    if args.len() > 3 {
        panic!("Argumento desconhecido"); 
    }
    let conteudo_arquivo = fs::read_to_string(&args[2]);
    if args[1] == "-c"{
        compactar(&args[2]);   
    }else if args[1] == "-d"{
        println!("Descompactar: arquivo: {}", args[2]);    
    }else{
        panic!("Argumento desconhecido") // TODO: Melhorar mensagem com usage
    }

    
    println!("Args = {:?} ", args);
    
    println!("Conteudo arquivo = {:?}", conteudo_arquivo)


}
