fn main(){
 println!("Sequencia de fibonacci");

 let mut anterior: i32 = 0;
 let mut atual: i32 = 1;
 let mut anterior1: i32 = 0;
 let mut atual1: i32 = 1;

 for i in 0..20 {

    if anterior % 2 == 0 {
        println!("Par: {}", anterior);
    }

    let proximo = anterior + atual;
    anterior = atual;
    atual = proximo;
 }   

  for i in 0..20 {

    if anterior1 % 2 == 0 {
    } else {
        println!("Impar: {}", anterior1);
    }
    
    let proximo1 = anterior1 + atual1;
    anterior1 = atual1;
    atual1 = proximo1;
 }   
}
