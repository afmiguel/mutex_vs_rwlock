// ------------------------------------------------------------------
// TODO: Atividade (Passo 1: Mutex)
// 1. Adicione 'Mutex' a esta linha de 'use'.
//
// TODO: Atividade (Passo 2: RwLock)
// 1. Remova 'Mutex' e adicione 'RwLock' a esta linha.
// ------------------------------------------------------------------
use std::sync::{Arc, ???}; // <-- TODO 1
use std::thread;
use std::time::{Duration, Instant};

// --- PARÂMETROS DA SIMULAÇÃO ---
const NUM_READERS: u32 = 10;
const NUM_WRITERS: u32 = 2;
const READER_WORK_MS: u64 = 200;
const WRITER_WORK_MS: u64 = 100;

/// Executa a simulação.
/// Esta função será usada para testar *ambos* Mutex e RwLock.
fn run_simulation() {
    println!("\nIniciando teste...");

    // ------------------------------------------------------------------
    // TODO: Atividade (Passo 1: Mutex)
    // 1. Crie o 'data' usando: Arc::new(Mutex::new(Vec::<u32>::new()))
    //
    // TODO: Atividade (Passo 2: RwLock)
    // 1. Altere a linha para: Arc::new(RwLock::new(Vec::<u32>::new()))
    // ------------------------------------------------------------------
    let data: Arc< ??? > = ???; // <-- TODO 2

    let mut handles = vec![];
    let start = Instant::now();

    // --- Inicia as Threads Escritoras ---
    for i in 0..NUM_WRITERS {
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            // ------------------------------------------------------------------
            // TODO: Atividade (Passo 1: Mutex)
            // 1. Obtenha o lock com: data_clone.lock().unwrap()
            //
            // TODO: Atividade (Passo 2: RwLock)
            // 1. Obtenha o lock de *escrita* com: data_clone.write().unwrap()
            // ------------------------------------------------------------------
            let mut guard = ???; // <-- TODO 3 (Lock de Escrita)

            // Simula trabalho de escrita
            thread::sleep(Duration::from_millis(WRITER_WORK_MS));
            guard.push(i);

            // Lock é liberado aqui
        });
        handles.push(handle);
    }

    // --- Inicia as Threads Leitoras ---
    for _ in 0..NUM_READERS { // Usando '_' para evitar 'unused_variable'
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            // ------------------------------------------------------------------
            // TODO: Atividade (Passo 1: Mutex)
            // 1. Obtenha o lock com: data_clone.lock().unwrap()
            //
            // TODO: Atividade (Passo 2: RwLock)
            // 1. Obtenha o lock de *leitura* com: data_clone.read().unwrap()
            // ------------------------------------------------------------------
            let guard = ???; // <-- TODO 4 (Lock de Leitura/Acesso)

            // Simula trabalho de leitura
            thread::sleep(Duration::from_millis(READER_WORK_MS));

            // Realiza a leitura (apenas para simular)
            let _len = guard.len();

            // Lock é liberado aqui
        });
        handles.push(handle);
    }

    // Espera todas terminarem
    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("[Teste] Tempo total: {:?}", duration);

    // ------------------------------------------------------------------
    // TODO: Atividade (Passo 1: Mutex)
    // 1. Leia o resultado com: data.lock().unwrap().len()
    //
    // TODO: Atividade (Passo 2: RwLock)
    // 1. Leia o resultado com: data.read().unwrap().len()
    // ------------------------------------------------------------------
    let final_len = ???; // <-- TODO 5 (Leitura Final)

    println!("[Teste] Valor final dos dados (len): {}", final_len);
}

/// Executa os dois benchmarks para comparação
fn main() {
    println!("--- Comparação de Desempenho Mutex vs. RwLock ---");
    println!("Parâmetros: {} Leitores ({}ms) | {} Escritores ({}ms)\n",
             NUM_READERS, READER_WORK_MS, NUM_WRITERS, WRITER_WORK_MS);

    // --- INSTRUÇÕES DA ATIVIDADE ---
    //
    // PASSO 1: Preencha todos os 5 'TODO's com a solução para MUTEX.
    //          (Veja os TODOs 1, 2, 3, 4 e 5).
    //
    // PASSO 2: Execute o código no modo 'release' para um benchmark adequado:
    //          $ cargo run --release
    //
    // PASSO 3: Anote o "Tempo total". Este é o seu 'Resultado 1 (Mutex)'.
    //
    // PASSO 4: Agora, altere os mesmos 5 'TODO's para a solução com RWLOCK.
    //
    // PASSO 5: Execute o código novamente:
    //          $ cargo run --release
    //
    // PASSO 6: Anote o "Tempo total". Este é o 'Resultado 2 (RwLock)'.
    //
    // PASSO 7: Compare os dois resultados e explique a diferença de desempenho.
    //
    // --- FIM DAS INSTRUÇÕES ---

    // Chame a função de simulação.
    run_simulation();
}