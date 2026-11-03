
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
use std::collections::VecDeque;
use std::ptr::hash;
use std::string;
enum Huffman {
    Leaf {c:char, freq:u32},
    Node {freq:u32,
    left :Box<Huffman>,
    right:Box<Huffman>}
}

impl Huffman {
    fn freq(&self) -> u32 {
        match self {
            Huffman::Leaf { freq, .. } => *freq,
            Huffman::Node { freq, .. } => *freq,
        }
    }
}

fn pegar_proximo_menor(fila_folhas: &mut VecDeque<Box<Huffman>>,fila_pais: &mut VecDeque<Box<Huffman>>,) -> Box<Huffman> {
    match (fila_folhas.front(), fila_pais.front()) {
        (Some(n1), Some(n2)) => {
            if n1.freq() <= n2.freq() {
                fila_folhas.pop_front().unwrap()
            } else {
                fila_pais.pop_front().unwrap()
            }
        }
        (Some(_), None) => fila_folhas.pop_front().unwrap(),
        (None, Some(_)) => fila_pais.pop_front().unwrap(),
        (None, None) => panic!("erro"),
    }
}

fn get_string_from_arvore(arvore: Option<Box<Huffman>>) -> String{
    
    let s = String::new();

    while arvore.is_some(){
        match arvore{
            
        }
    }
    s
}

fn criar_tabela_char_cod(arvore: Option<Box<Huffman>>)-> HashMap<char,String>{
    let mut hash_map: HashMap<char,String> = HashMap::new();


    if let Some(no) = arvore{
        match *no{
            Huffman::Leaf { c, freq } => {

            }
            Huffman::Node { freq, left, right } =>{

            }
        }
    }
    


    hash_map
}

fn criar_tabela_cod_char(arvore: Option<Box<Huffman>>) -> HashMap<String, char>{
  let mut hash_map: HashMap<String, char> = HashMap::new();






    hash_map
}

fn percorrer_arvore_string(){
    let a =0;
}

fn percorrer_arvore_folha(){
    let a =0;
}


fn criar_arvore(frequencia: &HashMap<char, u32>) -> Option<Box<Huffman>> {
    if frequencia.is_empty() {
        panic!("Erro frequencia vazio");
    }

    let mut folhas: Vec<Box<Huffman>> = frequencia
        .iter()
        .map(|(&c, &freq)| Box::new(Huffman::Leaf { c, freq }))
        .collect();

    folhas.sort_by(|a, b| a.freq().cmp(&b.freq()));

    let mut fila_folhas = VecDeque::from(folhas);
    let mut fila_pais: VecDeque<Box<Huffman>> = VecDeque::new();

    while fila_folhas.len() + fila_pais.len() > 1 {
        let esq = pegar_proximo_menor(&mut fila_folhas, &mut fila_pais);
        let dir = pegar_proximo_menor(&mut fila_folhas, &mut fila_pais);

        let soma = esq.freq() + dir.freq();

        let pai = Box::new(Huffman::Node {
            freq: soma,
            left: esq,
            right: dir,
        });

        fila_pais.push_back(pai);
    }

    if !fila_pais.is_empty() {
        fila_pais.pop_front()
    } else {
        fila_folhas.pop_front()
    }
}

fn leitura_arquivo_normal(nome_arquivo: &str) -> Result<String,io::Error>{
    println!("criando arvore");
    let mut conteudo: String = String::new();
    let mut arquivo = File::open(nome_arquivo)?;
    arquivo.read_to_string(&mut conteudo)?;
    Ok(conteudo)
}



fn frequencia(string_arquivo: &str) -> Result<HashMap<char, u32>, String>{
    let mut mapa_frequencia: HashMap<char,u32> = HashMap::new();

    if string_arquivo.is_empty() {
        return Err(String::from("A string está azia!"));
    }

    for c in string_arquivo.chars(){
        match mapa_frequencia.get(&c){
            None => {
                mapa_frequencia.insert(c, 1);
            }
            Some(k) =>{
                mapa_frequencia.insert(c, k+1);
            }
        }
    }


    Ok(mapa_frequencia)
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


    let frequencia = match frequencia(&string_arquivo){
           Ok(contents) => {contents}
        Err(e) => {
            eprintln!("Erro na funcao frequencia: {}", e);
            HashMap::new()
        }
    };

    let arvore = criar_arvore(&frequencia);


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



}
