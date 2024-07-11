use std::io; // Bioblioteca de  Input 

fn convert_to_int(data_input: & String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    // O trim serve para cortar a String Exemplo: Hello/tworld só q com a barra invertida!
    //parse converte para i32
    //unwrap desenvelopa e ajuda a pegar erros
    x
}


fn main() {
    // let name: &str = "Guilherme"; // &str significa String
    // let age: i32 = 17; // i32 se refere ao numero de bits que vai até 128 bits.  Numero INTEIRO!
    // let f: f32 = 0.424; // f32 se refere ao numero flutuante que vai até 64 bits
    // let b: bool = true; // Bollean padrão sem muita inerencia k é sim ou não 
    // println!("Hello {}!", name);
    // println!("My age is {}", age);

    let mut number1 = String::new();
    println!("Digite um número inteiro: ");
    io::stdin().read_line(&mut number1).expect("Erro ao ler number1");
    
    let mut number2 = String::new();
    println!("Digite um número inteiro: ");
    io::stdin().read_line(&mut number2).expect("Erro ao ler number2");

    if convert_to_int(&number1) > convert_to_int(&number2){
        println!("O primeiro número é maior que o segundo!");
    } else {
        println!("O segundo número é maior que o primeiro!");
    }

}
