

fn eh_palindromo(s: String) -> bool{
    let mut pilha: Vec<char> = Vec::new();
    
    
    for c in s.chars(){
        pilha.push(c);
    }
    
        
    
    for c in s.chars(){
        if c != pilha.pop().expect("Pilha == none"){
            return false;
        }
    }
    
    true
}


fn main(){

    let _string_teste: String = "ovo".to_string();
    let _string_comp: String = "ovo".to_string();
    println!("{} -- {}",_string_comp ,eh_palindromo(_string_teste));
}