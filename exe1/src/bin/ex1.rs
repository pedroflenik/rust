
fn celsius_to_fahrenheit(degrees_celsius: f64) -> f64 {
    (degrees_celsius * 1.8) + 32.0
} 


fn main(){
    let array_teste : [f64;5]= [0.0,100.0,3.5,40.0,30.0];

    for i in 0..5{
        println!("{} -- {}", array_teste[i], celsius_to_fahrenheit(array_teste[i]))
    }
}