
#[derive (Debug)]
struct No<T> {
    elem: T,
    prox: Option<Box<No<T>>>
}

#[derive (Debug)]
struct Lista<T> {
    cab: Option<Box<No<T>>>
}

impl <T>Lista<T>{

    fn new() -> Self{
        Lista{cab:None}
    }



    fn inserir_cauda(&mut self, e:T){
        let mut atual = &mut self.cab;
        while atual.is_some(){
            atual =  &mut atual.as_mut().unwrap().prox;
        }
        *atual = Some(Box::new(No {elem:e, prox:None})); 
    }

    fn remover_cauda(&mut self){
        if self.cab.is_none(){
            return
        }
        if self.cab.as_mut().unwrap().prox.is_none(){
            self.cab.take();
            return;
        }

        let mut atual = &mut self.cab;

      
        while atual.as_mut().unwrap().prox.as_mut().unwrap().prox.is_some(){
            atual = &mut atual.as_mut().unwrap().prox;
        }
    
        atual.as_mut().unwrap().prox.take();
    }


    fn inserir_ordenado(&mut self, e:T){
        
    }




}

fn main() {

   
    let mut l:Lista<u32> = Lista::new();

    l.inserir_cauda(1);
    l.inserir_cauda(2);
    l.inserir_cauda(5);
    l.inserir_cauda(32);

    println!("{l:?}");

    l.remover_cauda();
    println!("{l:?}");
     
    l.remover_cauda();
    println!("{l:?}");
    

     l.remover_cauda();
    println!("{l:?}");

     l.remover_cauda();
    println!("{l:?}");

     l.remover_cauda();
    println!("{l:?}");

}
