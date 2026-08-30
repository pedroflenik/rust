


fn apenas_primos(v:&mut Vec<u128>) -> Vec<u128>{
    let res:&mut Vec<u128> = &mut Vec::new();
    for i in v{
        if eh_primo(*i){
            res.push(*i);
        }
    }
    res.to_vec()
}


fn eh_primo(n: u128) -> bool{
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

fn main(){
    let mut v:Vec<u128> = vec![1];
    v.splice(0..0,1..=100);
    
    println!("{:?}", apenas_primos(&mut v));
}