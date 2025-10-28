use std::{io::{self, Write}};

struct ContaBancaria {
    titular: String,
    saldo: f64,
}

impl ContaBancaria {
    fn depositar(&mut self, valor: f64) {
        self.saldo += valor;
        println!("R${} depositados. Saldo: R${}.", valor, self.saldo);
    }

    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor;
        println!("R${} sacados. Saldo: R${}.", valor, self.saldo);
    }

    fn mostrar_saldo(&self) {
        println!("Saldo da conta {}: R${}.", self.titular, self.saldo);
    }
}

//Obs: Na prática, seria muito mais fácil utilizar uma função para input.
fn main() {
    print!("Digite o seu nome: ");

    io::stdout()
        .flush()
        .unwrap();

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler input.");
    
    let nome: String = input;

    let mut conta: ContaBancaria = ContaBancaria{
        titular: nome,
        saldo: 0.0,
    };

    loop {
        println!("--- BANCO ---\nTitular: {}\nSaldo: {}", conta.titular, conta.saldo);

        // Opções
        print!("Escolha uma ação: \n
        [1] Depositar\n
        [2] Sacar\n
        [3] Mostrar Saldo\n
        [4] Sair \n
        Digite: ");

        io::stdout()
            .flush()
            .unwrap();

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler input.");

        let opcao: u8 = input
                            .trim()
                            .parse()
                            .expect("Valor inválido.");

        let acao = if opcao == 1 {
            Ok("Depositar")
        } else if opcao == 2 {
            Ok("Sacar")
        } else if opcao == 3 {
            Ok("Mostrar saldo")
        } else if opcao == 4 {
            break
        }
         else {
            Err("Opção inválida.")
        };

        if acao == Ok("Depositar") {
            print!("Digite o valor que deseja depositar: ");

            io::stdout()
                .flush()
                .unwrap();

            let mut input: String = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Erro ao ler input.");

            let valor: f64 = input
                                .trim()
                                .parse()
                                .expect("Valor inválido.");

            conta.depositar(valor);
        }

        else if acao == Ok("Sacar") {
            print!("Digite o valor que deseja sacar: ");

            io::stdout()
                .flush()
                .unwrap();

            let mut input: String = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Erro ao ler input.");

            let valor: f64 = input
                                .trim()
                                .parse()
                                .expect("Valor inválido.");

            conta.sacar(valor);
            }

        else if acao == Ok("Mostrar saldo") {conta.mostrar_saldo()}
            }
}

