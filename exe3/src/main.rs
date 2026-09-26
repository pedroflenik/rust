/*
Implemente em Rust métodos que:


• Insira um elemento (obedecendo a organização de árvore binária de pesquisa, elementos repetidos não devem ser inseridos);
• Retorne e remova o maior elemento (se houver);
• Remova um elemento passado como parâmetro;
• Forneça também uma implementações para a trait Display;
• Forneça um iterador (trait Iterator) que percorra a árvore em ordem sem consumi-la


*/

use core::panic;
use std::cell::Ref;

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

    fn remover_maior(&mut self){
        if self.raiz.is_none(){
            panic!("Árvore vazia") // TODO: lembrar de tratar erro com Result
        }else{
            let mut atual = &mut self.raiz;
            while !atual.as_ref().unwrap().dir.is_none(){
                atual = &mut atual.as_mut().unwrap().dir;
            }
            let no_esq = atual.as_mut().unwrap().esq.take();

            *atual = no_esq;
        }
           
    }

    fn remover_elemento(&mut self, elem: T){
        if self.raiz.is_none(){
            panic!("Árvore vazia!")//TODO: lembrar de tratar erro com result
        }else{
            let mut atual = &mut self.raiz;
            while atual.is_some() && atual.as_ref().unwrap().elem != elem{
                if atual.as_ref().unwrap().elem < elem{
                    atual = &mut atual.as_mut().unwrap().dir;
                }else {
                    atual = &mut atual.as_mut().unwrap().esq;
                }
            }
            if(atual.is_none()){
                panic!("Elemento não existe na árvore")
            }else{
                /*
                    Tratar 3 casos
                    1 - Atual é um nó folha
                    2 - Atual tem 1 filho
                    3 - Atual tem 2 filhos
                 */

                // Caso 1
                if atual.as_ref().unwrap().esq.is_none() && atual.as_ref().unwrap().dir.is_none() {
                    atual.take();
                }else{
                    if atual.as_ref().unwrap().esq.is_none(){
                        // Caso 2 com filho a dir
                        let no_dir = atual.as_mut().unwrap().dir.take();
                        *atual = no_dir;
                    }else if atual.as_ref().unwrap().dir.is_none(){
                        // Caso 3 com filho a esq
                        let no_esq = atual.as_mut().unwrap().esq.take();
                        *atual = no_esq;
                    }else{
                        // Caso 3
                        let mut menor_no_dir = &mut atual.as_mut().unwrap().dir;
                        while menor_no_dir.as_ref().unwrap().esq.is_some(){
                            menor_no_dir = &mut menor_no_dir.as_mut().unwrap().esq;
                        }
                        let menor_valor = menor_no_dir.as_ref().unwrap().elem;
                        let filho_sucessor = menor_no_dir.as_mut().unwrap().dir.take();
                        *menor_no_dir = filho_sucessor;
                        atual.as_mut().unwrap().elem = menor_valor;
                    }
                }
            }
        }
    }


}
fn main(){

}