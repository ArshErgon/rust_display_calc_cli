mod algorithms;
mod startup;

fn main() {
    startup::starting_up();
    // args is talking the cli argument in vector
    let args: Vec<String> = std::env::args().skip(1).collect();
    let input: String = args.join(" ");
    let characters: Vec<&str> = input.split_whitespace().collect();
    // characters remove any white-spaces

    // shunting algorithm to convert numercial to polish notation
    let (number_vector, sya_number) = algorithms::shunting_yard_algorithm(characters);

    let msg = format!(" Numerical: {input} \n Shunting Algorithm: {sya_number}\n");
    println!("{msg}");

    algorithms::reverse_polish_notation(number_vector);
}
