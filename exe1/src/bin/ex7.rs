use std::{collections::HashMap, hash::Hash};

fn maior_frequencia_char(s: &str) -> char{
    
    let mut cont:HashMap<char, u16>= HashMap::new();
    
    for c in s.chars() {
        *cont.entry(c).or_insert(0) += 1;
    }

    let mut maior:u16 = 0;
    let mut  res:char = 'a';

    for (chave, valor) in cont{
        if valor >= maior{
            maior = valor;
            res = chave;
        }
    }
    
    res
}


fn main(){
    let  test = "testess".to_string();

    println!("{:?}", maior_frequencia_char(&test));
}