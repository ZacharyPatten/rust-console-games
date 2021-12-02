use std::io;
use rand::prelude::*;
use std::io::prelude::*;

fn main()
{
    let mut random = rand::thread_rng();
    let mut wins   : u32 = 0;
    let mut draws  : u32 = 0;
    let mut losses : u32 = 0;
    while wins < u32::max_value() && draws < u32::max_value() && losses < u32::max_value()
    {
        clear();
        println!("Rock, Paper, Scissors");
        println!();
        let mut valid_input = false;
        let mut input = "default";
        let mut input_string = String::new();
        while !valid_input
        {
            print!("Choose [r]ock, [p]aper, [s]cissors, or [e]xit: ");
            flush();
            input_string = String::new();
            match io::stdin().read_line(&mut input_string)
            {
                Err(error) => { println!("error: {}", error); flush(); return; }
                _ => {},
            }
            input = input_string.trim();
            match input
            {
                "rock"     | "r" => { input = "rock";     valid_input = true; },
                "paper"    | "p" => { input = "paper";    valid_input = true; },
                "scissors" | "s" => { input = "scissors"; valid_input = true; },
                "exit"     | "e" => { clear(); return; },
                _ => { println!("Invalid input. Try again..."); flush(); valid_input = false; },
            }
        }
        let computer_move = random.gen_range(0..3);
        let computer_move_str = match computer_move
        {
            0 => "rock",
            1 => "paper",
            2 => "scissors",

            _ => { clear(); println!("error"); flush(); return; },
        };
        match (input, computer_move)
        {
            ("r", 0) | ("rock",     0) |
            ("p", 1) | ("paper",    1) |
            ("s", 2) | ("scissors", 2) => { draws = draws + 1; println!("Draw. You played {} and the computer played {}.", input, computer_move_str); },

            ("r", 1) | ("rock",     1) |
            ("p", 2) | ("paper",    2) |
            ("s", 0) | ("scissors", 0) => { losses = losses + 1; println!("Lose. You played {} and the computer played {}.", input, computer_move_str); },

            ("r", 2) | ("rock",     2) |
            ("p", 0) | ("paper",    0) |
            ("s", 1) | ("scissors", 1) => { wins = wins + 1; println!("Win. You played {} and the computer played {}.", input, computer_move_str); },

            _ => { clear(); println!("error"); flush(); return; },
        }
        println!();
        println!("wins:   {}", wins);
        println!("losses: {}", losses);
        println!("draws:  {}", draws);
        println!();
        print!("Press Enter To Continue...");
        flush();
        match io::stdin().read_line(&mut input_string)
        {
            Err(error) => { println!("error: {}", error); flush(); return; }
            _ => {},
        }
    }
}

fn clear()
{
    print!("\x1B[2J\x1B[1;1H");
    flush();
}

fn flush()
{
    match io::stdout().flush() { _ => {} };
}