/*
Implemente um programa multithread que calcule todos os números primos no intervalo de 2 a n, inclusive.
Sendo o valor máximo de n 4.294.967.295.

Construa um gráfico comparando os tempos de execução das versões sequencial e multithread do algoritmo, variando o valor de n e o número de threads utilizadas. 
Apresente também o speedup obtido em cada configuração e analise os resultados.

O número de threads deve atingir: thread::available_parallelism().

A geração de todos os casos deve ser automatizada.

Olhar a biblioteca bitvec.

Como paralelizar
1 - Resolver loop externo de forma serial
2 - Dividor restante entre threads de forma igualitaria
3 - Juntar respostas



*/



use bitvec::prelude::*;
use std::thread;

fn calcular_primos(v: &mut BitSlice, inicio: usize, fim:usize, primos_base: &mut BitSlice){
    let mut ultimo_verificado: usize = 0;

    for (i, b) in primos_base.iter().enumerate(){
        if *b{
            for j in ((i * i)..fim).step_by(i) {
                v.set(inicio + j, false);
            }
        }
    }
}

fn calcular_primos_serial(v:&mut BitSlice,  n:usize, limite: usize){
    v.set(0, false);
    v.set(1, false);

    

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

    const N:usize = 4294967295;

    let limite = (N as f64).sqrt() as usize;
    let mut bits = bitvec![1; N];

    let mut numeros_base = bitvec![1; limite];

    let limite_primeiro_calculo = (limite as f64).sqrt() as usize;
    calcular_primos_serial(&mut b, limite_primeiro_calculo, limite);


    for i in 0..bits.len(){
        if bits[i] == true{
            print!("{} ",i);
        }
    }



    // printar bits
    // let bits: BitArr!(for 100, in u8) = BitArray::ZERO;

    // for b in bits{
    //     print!("B = {}", b);
    // }

}
