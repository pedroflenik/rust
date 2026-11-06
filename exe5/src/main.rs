/*
Implemente um programa multithread que calcule todos os números primos no intervalo de 2 a n, inclusive.
Sendo o valor máximo de n 4.294.967.295.

Construa um gráfico comparando os tempos de execução das versões sequencial e multithread do algoritmo, variando o valor de n e o número de threads utilizadas. 
Apresente também o speedup obtido em cada configuração e analise os resultados.

O número de threads deve atingir: thread::available_parallelism().

A geração de todos os casos deve ser automatizada.

Olhar a biblioteca bitvec.

 */



use bitvec::prelude::*;

fn calcular_primos_serial(v:&mut BitSlice,  n:usize){
    v.set(0, false);
    v.set(1, false);

    let limite = (n as f64).sqrt() as usize;

    for i in 2..limite{
        if v[i] == true{
            for j in ((i*i)..n).step_by(i){
                v.set(j, false);
            }
        }
    }
}



fn main() {
    // let n = 4294967295
    // let n = 2294967295;
    
    const N:usize = 100;
    let mut bits = bitvec![1; N];

    calcular_primos_serial(&mut bits, N);

    for i in 0..bits.len(){
        if bits[i] == true{
            print!("{} ",i);
        }
    }


    // let bits: BitArr!(for 100, in u8) = BitArray::ZERO;

    // for b in bits{
    //     print!("B = {}", b);
    // }

}
