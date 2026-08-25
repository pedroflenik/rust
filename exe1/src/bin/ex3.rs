
fn max(array:&mut [u32]) -> Option<u32> {
    let mut maior:u32;
    if array.len() == 0{
        return  None;
    }else{
        maior = array[0];
        for i in array{
            if *i >= maior{
                maior = *i;
            }
        }
    }
    Some(maior)
} 


fn main(){
    let mut array_teste : [u32;0]= [];

    println!("{:?}", array_teste);
    println!("{:?}", max(&mut array_teste));
}