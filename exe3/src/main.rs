/*
Implemente em Rust métodos que:


• Insira um elemento (obedecendo a organização de árvore binária de pesquisa, elementos repetidos não devem ser inseridos);
• Retorne e remova o maior elemento (se houver);
• Remova um elemento passado como parâmetro;
• Forneça também uma implementações para a trait Display;
• Forneça um iterador (trait Iterator) que percorra a árvore em ordem sem consumi-la


*/

use core::panic;

pub struct Arvore<T:Ord + Copy> {
    raiz : Option<Box<No<T>>>
}

pub struct No <T:Ord + Copy> {
    elem: T,
    esq: Option<Box<No<T>>>,
    dir: Option<Box<No<T>>>,
}


impl<T: Ord + Copy> Arvore<T> {    
    fn inserir_elem(&mut self, elem: T){
        let mut atual = &mut self.raiz;

        while !atual.is_none(){
            if atual.as_ref().unwrap().elem == elem{
                panic!("Elemento já existe na árvore");
                // TODO: Lembrar de tratar erro com Result
            }
            if atual.as_ref().unwrap().elem > elem{
                atual = &mut atual.as_mut().unwrap().esq;
            }else{
                atual = &mut atual.as_mut().unwrap().dir;
            }
        }    

        *atual =Some(Box::new(No { elem, esq: None, dir: None }))
    }

    fn remover_maior(&self){

    }

    fn remover_elemento(&self, elem: T){

    }
}
fn main(){

}