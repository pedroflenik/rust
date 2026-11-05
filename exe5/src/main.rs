/*
Implemente um programa multithread que calcule todos os números primos no intervalo de 2 a n, inclusive.
Sendo o valor máximo de n 4.294.967.295.

Construa um gráfico comparando os tempos de execução das versões sequencial e multithread do algoritmo, variando o valor de n e o número de threads utilizadas. 
Apresente também o speedup obtido em cada configuração e analise os resultados.

O número de threads deve atingir: thread::available_parallelism().

A geração de todos os casos deve ser automatizada.

Olhar a biblioteca bitvec.

 */

fn calcular_primos(n:usize) -> Vec<bool>{
    let mut v: Vec<bool> = vec![true; n];
    v[0] = false;
    v[1] = false;
    for i in 2..n{
        if v[i] == true{
            
            for j in ((i*i)..n).step_by(i){
                    v[j] = false;
            }
        }
    }

    v
}



fn main() {
    let n = 2294967295;
    let k = calcular_primos(n);

    for i in 0..k.len(){
        if k[i] == true{
            print!("{} ",i);
        }
    }
}
