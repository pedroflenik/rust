/*
Implemente em Rust métodos que:
• Insira um elemento (obedecendo a organização de árvore binária de pesquisa, elementos repetidos não devem ser inseridos); X
• Retorne e remova o maior elemento (se houver); X
• Remova um elemento passado como parâmetro; X
• Forneça também uma implementações para a trait Display; X
• Forneça um iterador (trait Iterator) que percorra a árvore em ordem sem consumi-la
*/

use core::{fmt, panic};

pub struct Arvore<T:Ord + Copy> {
    raiz : Option<Box<No<T>>>
}

pub struct No <T:Ord + Copy> {
    elem: T,
    esq: Option<Box<No<T>>>,
    dir: Option<Box<No<T>>>,
}

pub struct ArvoreIte<'a, T>
where
    T: Ord + Copy,
{
    pilha: Vec<&'a No<T>>,
}
impl<'a, T: Ord + Copy> ArvoreIte<'a, T> {
    fn empilha_toda_esquerda(&mut self, mut atual: Option<&'a No<T>>) {
        while atual.is_some() {
            let n = atual.unwrap();
            self.pilha.push(n);
            atual = n.esq.as_deref();
        }
    }
}
impl<'a, T: Ord + Copy> Iterator for ArvoreIte<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(no) = self.pilha.pop() {
            self.empilha_toda_esquerda(no.dir.as_deref());
            Some(&no.elem)
        } else {
            None
        }
    }
}


impl<T: Ord + Copy + fmt::Display> fmt::Display for Arvore<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.raiz{
            None => write!(f,"ÁRVORE VAZIA"),
            Some(no) => write!(f, "[ {} ]",no)
        }
    }
}

impl<T: Ord + Copy + fmt::Display> fmt::Display for No<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, " {} ", self.elem);

        if let Some(ref esq) = self.esq {
            write!(f, "[ E:{} ] ", esq); 
        }
        
        if let Some(ref dir) = self.dir {
            write!(f, " [ D:{} ]", dir);
        }
        
        Ok(())
    }
}

impl<T: Ord + Copy> Arvore<T> {    

    pub fn iter(&self) -> ArvoreIte<'_, T> {
        let mut iter = ArvoreIte { pilha: Vec::new() };
        iter.empilha_toda_esquerda(self.raiz.as_deref());
        iter
    }

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
            if atual.is_none() {
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
    // testes
    let mut minha_arvore: Arvore<i32> = Arvore { raiz: None };

    minha_arvore.inserir_elem(10);
    minha_arvore.inserir_elem(5);
    minha_arvore.inserir_elem(15);
    minha_arvore.inserir_elem(3);
    minha_arvore.inserir_elem(7);

    println!("Árvore inicial:");
    println!("{}", minha_arvore);
    println!();

    println!("tirando o elemento 5");
    minha_arvore.remover_elemento(5);
    println!("{}", minha_arvore);
    println!();

    println!("tirando o maior");
    minha_arvore.remover_maior();
    println!("{}", minha_arvore);


    println!("Testando iterador:");
    let iterr = minha_arvore.iter();
    for valor in iterr {
        print!("{} ", valor);
    }

    println!();
    println!();

}