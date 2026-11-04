
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


fn criar_arvore(frequencia: &HashMap<char, u32>) -> Option<Box<Huffman>> {
    if frequencia.is_empty() {
        panic!("Erro frequencia vazio");
    }

    let mut folhas: Vec<Box<Huffman>> = frequencia
        .iter()
        .map(|(&c, &freq)| Box::new(Huffman::Leaf { c, freq }))
        .collect();

    folhas.sort_by(|a, b| {
        match (a.as_ref(), b.as_ref()) {
            (Huffman::Leaf { c: c1, freq: f1 }, Huffman::Leaf { c: c2, freq: f2 }) => {
            if f1 == f2 {
                ((*c1) as u32).cmp(&((*c2) as u32))
            }else{
                f1.cmp(f2)
            }
            }
            _ => a.freq().cmp(&b.freq()),
        }
    });

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

fn leitura_arquivo_compactado(nome_arquivo: &str) -> Result<(u32, HashMap<char, u32>, Vec<u8>), io::Error> {
    let mut arquivo = File::open(nome_arquivo)?;

    let mut buf_n = [0u8; 2];
    arquivo.read_exact(&mut buf_n)?;
    let n = u16::from_le_bytes(buf_n) as u32;

    let mut buf_t = [0u8; 4];
    arquivo.read_exact(&mut buf_t)?;
    let t = u32::from_le_bytes(buf_t);

    let mut mapa_frequencia = HashMap::new();

    for _ in 0..n {
        let mut buf_char = [0u8; 4];
        arquivo.read_exact(&mut buf_char)?;
        let codigo_u32 = u32::from_le_bytes(buf_char);
        let c = char::from_u32(codigo_u32).unwrap_or('?');

        let mut buf_freq = [0u8; 4];
        arquivo.read_exact(&mut buf_freq)?;
        let freq = u32::from_le_bytes(buf_freq);

        mapa_frequencia.insert(c, freq);
    }

    let mut bytes_bits = Vec::new();
    arquivo.read_to_end(&mut bytes_bits)?;

    Ok((t, mapa_frequencia, bytes_bits))
}

fn descompactar(nome_arquivo: &str){
    println!("Descompactando arquivo: {}", nome_arquivo);
    let (t, frequencia, bytes_bits) = match leitura_arquivo_compactado(nome_arquivo) {
        Ok(dados) => dados,
        Err(e) => {
            panic!("Erro ao ler arquivo compactado: {}", e);
        }
    };

    let arvore = criar_arvore(&frequencia);

    
    let tabela_cod_char = criar_tabela_cod_char(arvore);

    let texto = decodificar_com_tabela(&tabela_cod_char, &bytes_bits, t);

    let arquivo_saida = "arquivo_descompactado.txt";
    if let Err(e) = fs::write(arquivo_saida, texto) {
        eprintln!("Erro ao salvar arquivo descompactado: {}", e);
    } else {
        println!("Arquivo descompactado com sucesso em: {}", arquivo_saida);
    }
}


fn decodificar_com_tabela(tabela: &HashMap<String, char>, bytes_bits: &[u8], t: u32) -> String {
    let mut resultado = String::new();
    let mut codigo_atual = String::new();
    let mut caracteres_lidos = 0;

    'fora: for &byte in bytes_bits {

        for i in (0..8).rev() {
            if caracteres_lidos >= t {
                break 'fora; 
            }

            let bit = (byte >> i) & 1;

            if bit == 0 {
                codigo_atual.push('0');
            } else {
                codigo_atual.push('1');
            }

            if let Some(&c) = tabela.get(&codigo_atual) {
                resultado.push(c);
                codigo_atual.clear(); 
                caracteres_lidos += 1;
            }
        }
    }

    resultado
}


fn main() {
    // Passar o arquivo temp/arquivo ou /temp/arquivo * cargo run -- [opcao] [arquivo]
    let args: Vec<String> =  env::args().collect();


    if args.len() > 3 {
        panic!("Argumento desconhecido. Uso [opcao] [arquivo]"); 
    }

    let conteudo_arquivo = fs::read_to_string(&args[2]);
    if args[1] == "-c"{
        compactar(&args[2]);   
    }else if args[1] == "-d"{
        descompactar(&args[2]);    
    }else{
        panic!("Argumento desconhecido") // TODO: Melhorar mensagem com usage
    }

}
