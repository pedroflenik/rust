use rand::Rng;

fn eh_primo_forca_bruta(n: u128) -> bool{
    if n <= 1{
        return false;
    }
    if n == 2{
        return true;
    }
    if n % 2 == 0{
        return false;
    }else{
        for i in (3..n/2).step_by(2){
            if n % i == 0 {
                return false;
            }
        }
    }
    true
}

/// N -> Valor a ser testado | k -> numero repetiçoes
fn eh_primo_probabilistico(n: u128, k: u128) -> bool {
    if n <= 3 { return n > 1; }
    
    let mut rng = rand::thread_rng();
    let exp = (n - 1) as u32;
    
    for _i in 0..k {
        let x: u128 = rng.gen_range(2..=n - 2);
        
        let mut acumulado: u128 = 1;
        for _k in 0..exp {
            acumulado = (acumulado * x) % n;
        }
        
        if acumulado != 1 {
            return false;
        }
    }
    true
}

fn main(){
    let teste: u128 = 71 ;
    println!("{}", eh_primo_forca_bruta(teste));
    println!("{}", eh_primo_probabilistico(teste, 10));
}