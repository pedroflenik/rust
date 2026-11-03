
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
use std::io::ErrorKind::NotConnected;
use std::io::{self, Read, Write};
use std::collections::VecDeque;
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

fn escrever_arquivo_compactado(frequencia:HashMap<char, u32>, string_arquivo: String, hash_codigos: HashMap<char,String>)
-> std::io::Result<()>{
    let mut novo_arquivo = File::create("saida.bits")?;
    let numero_simbolos = frequencia.keys().len() as u16;
    let mut total_chars: u32 = 0;
    
    for (key, value) in &frequencia{
        total_chars = total_chars + value;
    }

    let  mut bits_str = String::new();
    for c in string_arquivo.chars(){
        if let Some(codigo) = hash_codigos.get(&c){
            bits_str.push_str(codigo);
        }
    }

    novo_arquivo.write_all(&numero_simbolos.to_le_bytes())?;
    novo_arquivo.write_all(&total_chars.to_le_bytes())?;

    for (&c, &freq) in &frequencia {

        let c_u32 = c as u32;

        novo_arquivo.write_all(&c_u32.to_le_bytes())?;
        
        novo_arquivo.write_all(&freq.to_le_bytes())?;
    }

    let mut vetor_u8: Vec<u8> = Vec::new();
    let mut atual: u8 = 0;
    let mut cont: usize = 0;


    for bit in bits_str.chars(){

        atual <<= 1;
        if bit == '1'{
            atual |= 1;
        }

        cont +=1;

        if cont ==8{
            vetor_u8.push(atual);
            atual = 0;
            cont = 0;
        }

    }

    let resto = 8 - cont;
    if resto > 0{
        atual <<= resto;
        vetor_u8.push(atual);
    }

    novo_arquivo.write_all(&vetor_u8)?;
    Ok(())
}

fn criar_tabela_char_cod(arvore: Option<Box<Huffman>>)-> HashMap<char,String>{
    let mut hash_map: HashMap<char,String> = HashMap::new();
    let mut pilha_endereco: Vec<(&Huffman, String)> = Vec::new();

    let Some(raiz) = arvore else {
        panic!("Árvore vazia!");
    };


    pilha_endereco.push((&raiz, String::new()));

    while let Some((no, codigo)) = pilha_endereco.pop() {
        match  no {
            Huffman::Leaf { c, freq } => {
                // Escrever string
                // salvar no hash map
                let codigo_final;
                if codigo.is_empty(){
                    codigo_final = String::from('0')
                }else{
                    codigo_final = codigo;
                }
                hash_map.insert(*c, codigo_final);
            }
            Huffman::Node { freq, left, right } => {
                // Guardar endereço
                //push '0' ou '1'
                let mut codigo_dir = codigo.clone();
                codigo_dir.push('1');
                pilha_endereco.push((right, codigo_dir));

                let mut codigo_esq = codigo;
                codigo_esq.push('0');
                pilha_endereco.push((left, codigo_esq));
                
            }
        }
    }

    hash_map
}

fn criar_tabela_cod_char(arvore: Option<Box<Huffman>>) -> HashMap<String, char>{
  let mut hash_map: HashMap<String, char> = HashMap::new();

    let mut pilha_endereco: Vec<(&Huffman, String)> = Vec::new();
    let mut pilha_chars: Vec<char> = Vec::new();

    let Some(raiz) = arvore else {
        panic!("Árvore vazia!");
    };

    pilha_endereco.push((&raiz, String::new()));

    while let Some((no, codigo)) = pilha_endereco.pop() {
        match  no {
            Huffman::Leaf { c, freq } => {
                // Escrever string
                // salvar no hash map
                let mut codigo_final = String::new();
                if codigo.is_empty(){
                    codigo_final = String::from('0')
                }else{
                    codigo_final = codigo;
                }
                hash_map.insert(codigo_final, *c);
            }
            Huffman::Node { freq, left, right } => {
                // Guardar endereço
                //push '0' ou '1'
                let mut codigo_dir = codigo.clone();
                codigo_dir.push('1');
                pilha_endereco.push((right, codigo_dir));

                let mut codigo_esq = codigo;
                codigo_esq.push('0');
                pilha_endereco.push((left, codigo_esq));
                
            }
        }
    }

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
    let hash_map_codigos = criar_tabela_char_cod(arvore);
    let _ = escrever_arquivo_compactado(frequencia, string_arquivo, hash_map_codigos);
}


fn main() {
    // Passar o arquivo temp/arquivo ou /temp/arquivo * cargo run -- [opcao] [arquivo]
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
