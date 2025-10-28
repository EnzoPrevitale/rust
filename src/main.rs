struct Pessoa {
    nome: String,
    idade: u32,
}

impl Pessoa {
    fn apresentar(&self) {
        println!("Meu nome é {} e tenho {} anos.", self.nome, self.idade);
    }
}

fn novo(nome: String, idade: u32) -> Pessoa {
    return Pessoa { nome: nome, idade: idade };
}

fn main() {
    let pessoa: Pessoa = novo("Enzo".to_string(), 17);
    pessoa.apresentar();
}

