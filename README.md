# Run Locally
```git
$ cargo run "1 + 1 + 1" 
$ cargo run "( 1 + 2 ) + 4"
give a proper space between operators and integers or else it will failed
```

## Examples
```git
$ cargo run 1 + 1 + 1
Numerical: 1 + 1 + 1 
Shunting Algorithm: 1 1 1 + +
	Way to solve
Step: 1 -> 1 + 1 = 2
Step: 2 -> 2 + 1 = 3
Final Answer: 3


$ cargo run "2 * ( 3 + 4 ) - 5"
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/cli_display_cal '2 * ( 3 + 4 ) - 5'`
Numerical: 2 * ( 3 + 4 ) - 5 
Shunting Algorithm: 2 3 4 + * 5 -
	Way to solve
Step: 1 -> 4 + 3 = 7
Step: 2 -> 7 * 2 = 14
Step: 3 -> 5 - 14 = 9
Final Answer: 9

```

# to Run it on Windows or Linux without installing rust
> goto the exe folder, download the exe file for your OS and follow these steps

```git 

for linux
$ ./cli_display_cal "1 + 1 + 1"

for windows
$ ./cli_display_cal "1 + 1 + 1"  # or .\cli_display_cal.exe "1 + 1 + 1"      on Windows


```

make sure to change the expersion



### its still under process
> will be improving it more and will create a docker file for it also.
