# Calculator Cli
A cli calculator which uses shunting algorithm and reverse polish notation to do the required arthematic operations.

It first take the equation in a string format, remove all the white spaces and send the string into the shunting algorithm function which convert it to a polish notation, after that it send it to the reverse polish notation function to do simple arthematic operations with the help of stack.

[calculator_cli_ss.webm](https://user-images.githubusercontent.com/40994679/214606308-a70614ac-3a7e-4c7e-899d-003ef1b3cbd2.webm)


# Run Locally
```git
$ cargo run "1 + 1 + 1" 
$ cargo run "( 1 + 2 ) + 4"
give a proper space between operators and integers or else it will failed
```

## Examples
```git
$ cargo run "( 8 - 3 ) * 5"
    ______      __           __      __            
    / ____/___ _/ /______  __/ /___ _/ /_____  _____
   / /   / __ `/ / ___/ / / / / __ `/ __/ __ \/ ___/
  / /___/ /_/ / / /__/ /_/ / / /_/ / /_/ /_/ / /    
  \____/\__,_/_/\___/\__,_/_/\__,_/\__/\____/_/     
                                                   
    A calculator which follows BODMAS rule.
    

 Numerical: ( 8 - 3 ) * 5 
 Shunting Algorithm: 8 3 - 5 *

	Way to solve

Step: 1 -> Perform substraction 
 8 - 3 = 5

Step: 2 -> Perform multiplication 
 5 * 5 = 25

Final Answer: 25



$ cargo run "1 + 1 + 1"

    ______      __           __      __            
    / ____/___ _/ /______  __/ /___ _/ /_____  _____
   / /   / __ `/ / ___/ / / / / __ `/ __/ __ \/ ___/
  / /___/ /_/ / / /__/ /_/ / / /_/ / /_/ /_/ / /    
  \____/\__,_/_/\___/\__,_/_/\__,_/\__/\____/_/     
                                                   
    A calculator which follows BODMAS rule.
    

 Numerical: 1 + 1 + 1 
 Shunting Algorithm: 1 1 1 + +

	Way to solve

Step: 1 -> Perform addition 
 1 + 1 = 2

Step: 2 -> Perform addition 
 1 + 2 = 3

Final Answer: 3


```

# Dockerfile
```git
$ docker build -t myapp .
$ docker run -p 8000:8000 myapp
```

