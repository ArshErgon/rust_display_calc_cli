fn main() {
    // args is talking the cli argument in vector
    let args: Vec<String> = std::env::args().skip(1).collect();
    let input: String = args.join(" ");
    let characters: Vec<&str> = input.split_whitespace().collect();
    // characters remove any white-spaces

    // shunting algorithm to convert numercial to polish notation
    let (number_vector, sya_number) = shunting_yard_algorithm(characters);

    let msg = format!("Numerical: {input} \nShunting Algorithm: {sya_number}");
    println!("{msg}");

    reverse_polish_notation(number_vector);
    // you can understand pretty much here.
}

fn reverse_polish_notation(number_vector: Vec<&str>) {
    let mut stack: Vec<String> = Vec::new();
    let mut steps: i32 = 0;
    println!("\tWay to solve");
    for elements in number_vector {
        match elements {
            "+" | "-" | "*" | "/" => {
                let (num_one, num_two) = (stack.pop().unwrap(), stack.pop().unwrap());
                let num_one: i128 = num_one.parse().expect("num one failed");
                let num_two: i128 = num_two.parse().expect("num two failed");

                match elements {
                    "+" => {
                        // let sum = (num_one.parse::<i32>().unwrap() + num_two.parse::<i32>().unwrap()).to_string().clone();
                        let running_sum = (num_two + num_one).to_string();
                        steps += 1;
                        let msg = format!(
                            "Step: {steps} -> {num_two} {elements} {num_one} = {running_sum}"
                        );
                        stack.push(running_sum);
                        println!("{msg}");
                    }

                    "-" => {
                        let running_sum = (num_two - num_one).to_string();
                        steps += 1;
                        let msg = format!(
                            "Step: {steps} -> {num_two} {elements} {num_one} = {running_sum}"
                        );
                        stack.push(running_sum);
                        println!("{msg}");
                    }

                    "*" => {
                        let running_sum = (num_two * num_one).to_string();
                        steps += 1;
                        let msg = format!(
                            "Step: {steps} -> {num_two} {elements} {num_one} = {running_sum}"
                        );
                        stack.push(running_sum);
                        println!("{msg}");
                    }

                    "/" => {
                        if num_two == 0 {
                            println!("Can't divied by zero");
                            break;
                        } else {
                            let running_sum = (num_one / num_two).to_string();
                            steps += 1;
                            let msg = format!(
                                "Step: {steps} -> {num_two} {elements} {num_one} = {running_sum}"
                            );
                            stack.push(running_sum);
                            println!("{msg}");
                        }
                    }

                    &_ => continue,
                }
            }
            &_ => stack.push(elements.to_string()),
        }
    }
    if stack.len() > 0 {
        println!("Final Answer: {}", stack[0]);
    } else {
        println!("Process failed or 0 answer");
    }
}

fn shunting_yard_algorithm(args: Vec<&str>) -> (Vec<&str>, String) {
    let mut stack = Vec::new();
    let mut queue = Vec::new();

    // loop in args, which is the cli input
    // loop it last
    // if some a match
    // push the operator in the stack
    // if found an opening bracket
    // pop every operator from the stack and add it to the queue
    for element in args {
        match element {
            "+" | "-" | "/" | "*" => stack.push(element),
            "(" => continue,
            ")" => {
                while stack.len() > 0 {
                    queue.push(stack.pop().unwrap())
                }
            }
            _ => queue.push(element),
            // if something other then operators, push here, means numbers
        }
    }
    // to empty the stack for the operators
    // if some operator left this line will handle that error.
    while stack.len() > 0 {
        queue.push(stack.pop().unwrap());
    }

    let name = queue.join(" ");
    (queue, name)
}
