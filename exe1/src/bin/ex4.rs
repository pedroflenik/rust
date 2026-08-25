
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

fn eh_primo_probabilistico(n: u128) -> bool{
    true
}


fn main(){
    
    println!("{}", eh_primo_forca_bruta(23));
}