use std::io::{self, Write};

fn input(content: &str) -> String {
    let mut inp: String = String::new();
    print!("{}", content);

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut inp)
        .expect("Input error.");

    return inp;
}

fn float(string: String) -> f64{
    return string
            .trim()
            .parse()
            .expect("Type error.");
}

enum Transacao {
    Deposito(f64),
    Saque(f64),
}

struct Conta {
    titular: String,
    saldo: f64,
}

fn atualizar_saldo(conta: &mut Conta, transacao: Transacao) -> Result<(), &str>{
    match transacao {
        Transacao::Deposito(valor) => {
            println!("R${} depositados.", valor);
            Ok(conta.saldo += valor)
        }

        Transacao::Saque(valor) => {
            if !(valor > conta.saldo) {
                println!("R${} sacados.", valor);
                Ok(conta.saldo -= valor)
            } else {
                Err("Saldo insuficiente!")
            }
        }
    }
}

fn main() {
    loop {
        println!("-=-=-= BANCO =-=-=-");

        let nome = input("Digite o seu nome: ");
        let mut conta: Conta = Conta { titular: nome, saldo: 0.0 };

        loop {
            println!("Saldo: R${}", conta.saldo);
            println!("O que deseja fazer?");
            println!("[1] Depositar");
            println!("[2] Sacar");
            println!("[3] Informações da conta");
            println!("[4] Sair");

            let opcao: u8 = input("Escolha: ")
                                .trim()
                                .parse()
                                .expect("Type error.");

            if opcao == 1 {
                let valor: f64 = float(input("Digite o valor que deseja depositar: "));
                atualizar_saldo(&mut conta, Transacao::Deposito(valor)).expect("Erro.");
            } else if opcao == 2 {
                let valor: f64 = float(input("Digite o valor que deseja sacar: "));
                atualizar_saldo(&mut conta, Transacao::Saque(valor)).expect("Erro");
            } else if opcao == 3 {
                println!("-=-=-= Informações da Conta =-=-=-");
                println!("Titular: {}", conta.titular);
                println!("Saldo: R${}", conta.saldo);
            } else {
                break;
            }
        }
    }
}

