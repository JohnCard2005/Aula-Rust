use std::io;

fn main() {
        
    let mut produtoPastel: i32 = 6;
    let mut produtoCoxinha: i32 = 5;
    let mut produtoEmpada: i32 = 4;


    loop {

    println!("-------------MENU-------------");
    println!("1 - Ver produtos");
    println!("2 - Reduzir quantidade de produto");
    println!("0 - Sair");

    let mut opcao = String::new();
    io::stdin().read_line(&mut opcao).unwrap();

    match opcao.trim() {
        "1" => {
            println!("Produtos disponíveis:");
            println!("Pastel - R$ {}", produtoPastel);
            println!("Coxinha - R$ {}", produtoCoxinha);
            println!("Empada - R$ {}", produtoEmpada);
        }
        "2" => {
            println!("Digite o nome do produto que deseja reduzir");
            let mut produto = String::new();
            io::stdin().read_line(&mut produto).unwrap();

            match produto.trim() {
                "Pastel" => {
                    println!("Digite a quantidade que deseja reduzir do Pastel:");
                    let mut quantidade = String::new();
                    io::stdin().read_line(&mut quantidade).unwrap();

                    let nova_quantidade: i32 = quantidade.trim().parse().unwrap();
                    if nova_quantidade <= produtoPastel {
                        produtoPastel = produtoPastel - nova_quantidade;
                        println!("Nova quantidade de Pastel: {}", produtoPastel);
                    } else {
                        println!("Quantidade inválida! A quantidade de Pastel não pode ser negativa.");
                    }
                }
                "Coxinha" => {
                    println!("Digite a quantidade que deseja reduzir da Coxinha:");
                    let mut quantidade = String::new();
                    io::stdin().read_line(&mut quantidade).unwrap();

                    let nova_quantidade: i32 = quantidade.trim().parse().unwrap();

                    if nova_quantidade <= produtoCoxinha {
                        produtoCoxinha = produtoCoxinha - nova_quantidade;
                    println!("Nova quantidade de Coxinha: {}", produtoCoxinha);
                } else {
                    println!("Quantidade inválida! A quantidade de Coxinha não pode ser negativa.");}
                    }
                    
                "Empada" => {
                    println!("Digite a quantidade que deseja reduzir da Empada:");
                    let mut quantidade = String::new();
                    io::stdin().read_line(&mut quantidade).unwrap();

                    let nova_quantidade: i32 = quantidade.trim().parse().unwrap();
                    if nova_quantidade <= produtoEmpada {
                    produtoEmpada = produtoEmpada - nova_quantidade;
                    println!("Nova quantidade de Empada: {}", produtoEmpada);
                } else {
                    println!("Quantidade inválida! A quantidade de Empada não pode ser negativa.");
                }
                }
                 _ => {
                        println!("Opção inválida!");
                    }
            }
        }
        "0" => {
            println!("Saindo do programa...");
            break;
        } _ => {
        println!("Opção inválida!");

    }
    } 
}
}