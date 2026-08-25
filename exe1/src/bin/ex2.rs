
fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    }else{
        fibonacci(n - 1) + fibonacci(n - 2)
    }
} 


fn main(){
    let array_teste : [u32;6]= [0,1,2,3,4,5];

    for i in 0..6{
        println!("{} -- {}", array_teste[i], fibonacci(array_teste[i]))
    }
}