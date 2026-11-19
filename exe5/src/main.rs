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
2 - Dividor restante entre threads de forma igualitaria e estatica
3 - Juntar respostas

*/


use std::env;
use bitvec::prelude::*;
use core::num;
use std::thread;
use bitvec::access::BitSafeUsize;
use std::time::Instant;
use std::fs::File;
use std::io::{BufWriter, Write};

fn salvar_csv(caminho: &str, vetor_arquivo: &[(usize, usize, f64)]) -> std::io::Result<()> {
    let arquivo = File::create(caminho)?;
    let mut escritor = BufWriter::new(arquivo);

    writeln!(escritor, "threads,run,real_seconds")?;

    for &(threads, run, real_seconds) in vetor_arquivo {
        writeln!(escritor, "{},{},{:.3}", threads, run, real_seconds)?;
    }

    escritor.flush()?;
    Ok(())
}

fn calcular_primos_thread<T: BitStore>(inicio: usize,v: &mut BitSlice<T, Lsb0>,primos_base: &BitSlice,
){
    let fim = inicio + v.len();

    for (i, b) in primos_base.iter().enumerate() {
        if i < 2 {
            continue;
        }
        
        if *b {
            let quadrado = i * i;
            if quadrado >= fim {
                break; 
            }

            let pm: usize = inicio.div_ceil(i) * i;
            let comeco_li = pm.max(quadrado);

            for j in (comeco_li..fim).step_by(i) {
                v.set(j - inicio, false);
            }
        }
    }
}

fn calcular_primos_serial(v: &mut BitSlice) {
    let tamanho = v.len();
    
    if tamanho > 0 {
         v.set(0, false); 
    }
    if tamanho > 1 { 
        v.set(1, false); 
    }

    let limite = (tamanho as f64).sqrt() as usize;

    for i in 2..=limite {
        if v[i]==true {
            for j in ((i * i)..tamanho).step_by(i) {
                v.set(j, false);
            }
        }
    }
}




fn main() {
    const N: usize = 4_294_967_295;
    //let args: Vec<String> = env::args().collect();
    //let mut num_threads: usize = args.get(1).expect("faltou informar o número de threads").parse().expect("O n de threads deve ser um inteiro positivo");

    let max_threads = thread::available_parallelism().map(|n| n.get()).unwrap_or(12);
    let mut vetor_arquivo: Vec<(usize, usize, f64)> = Vec::new();

    for num_threads in 1..=max_threads{
        
        for j in 1..=10{
            let inicio = Instant::now();

            let limite = (N as f64).sqrt() as usize;

            let mut numeros_base = bitvec![1; limite];
            calcular_primos_serial(&mut numeros_base);

            let mut bits = bitvec![1; N];
            bits.set(0, false);
            bits.set(1, false);


            let parcela = N/ num_threads;
            let resto = N % num_threads;
            
            let primos_base = &numeros_base[..];
            thread::scope(|s| {
                let (_, mut resto_bits): (&mut BitSlice<BitSafeUsize>, &mut BitSlice<BitSafeUsize>) =bits.split_at_mut(0);
                let mut inicio = 0;

                for id in 0..num_threads {
                    let tam: usize = parcela + if id < resto {
                        1 
                    } else { 
                        0 
                    };
                    let (chunk, prox) = resto_bits.split_at_mut(tam);
                    resto_bits = prox;

                    s.spawn(move || {
                        calcular_primos_thread(inicio, chunk, primos_base);
                    });

                    inicio += tam;
                }

            });
            vetor_arquivo.push((num_threads,j,inicio.elapsed().as_secs_f64()));
        }
    }
     println!("acabou salvando arquivo");
    salvar_csv("stored_time.txt", &vetor_arquivo);
   
}