use std::io;
use rand::prelude::*;
use std::io::prelude::*;
use std::{thread, time};

fn main()
{
    let mut random = rand::thread_rng();
    let button_press_delay = time::Duration::from_millis(600);
    let animation_delay = time::Duration::from_millis(300);
    let mut playagain = true;
    while playagain
    {
        let mut pattern: Vec<u8> = Vec::new();
        let mut lose = false;
        while !lose
        {
            let random_step = random.gen_range(1..5);
            let pattern2 = &mut pattern;
            pattern2.push(random_step);
            for step in pattern2
            {
                clear();
                render_frame(*step);
                thread::sleep(button_press_delay);
                clear();
                render_frame(0);
                thread::sleep(animation_delay);
            }

            println!();
            println!("- up:    'u'");
            println!("- down:  'd'");
            println!("- left:  'l'");
            println!("- right: 'r'");
            println!("Example (up, down, down, left, right, up): \"uddlru\"");
            println!();
            print!("Repeat the patten an press enter: ");
            flush();

            let mut input_string = String::new();
            match io::stdin().read_line(&mut input_string)
            {
                Err(error) => { println!("error: {}", error); flush(); return; }
                _ => {},
            }
            let input_bytes = input_string.as_bytes();
            for i in 0..pattern.len()
            {
                let pattern_i = pattern[i];
                let input_bytes_i = char_to_u8_udlr(input_bytes[i] as char);
                if pattern_i != input_bytes_i
                {
                    lose = true;
                    println!("Wrong. Score: {}", pattern.len());
                    println!();
                    let mut valid_input = false;
                    while !valid_input
                    {
                        print!("Play again (yes/y)? ");
                        flush();
                        let mut input_string = String::new();
                        match io::stdin().read_line(&mut input_string)
                        {
                            Err(error) => { println!("error: {}", error); flush(); return; }
                            _ => {},
                        }
                        let input = input_string.trim();
                        match input
                        {
                            "yes" | "y" => { playagain = true;  valid_input = true; },
                            "no"  | "n" => { playagain = false; valid_input = true; },
                            _ => { println!("Invalid input. Try again..."); flush(); valid_input = false; },
                        }
                    }
                    break;
                }
            }
        }
    }
}

fn char_to_u8_udlr(c : char) -> u8
{
    match c
    {
        'u' => return 1,
        'r' => return 2,
        'd' => return 3,
        'l' => return 4,
        _   => return 0,
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

fn render_frame(i : u8)
{
    match i
    {
        0 =>
        {
            println!("           ╔══════╗       ");
            println!("           ║      ║       ");
            println!("           ╚╗    ╔╝       ");
            println!("    ╔═══╗   ╚╗  ╔╝   ╔═══╗");
            println!("    ║   ╚═══╗╚══╝╔═══╝   ║");
            println!("    ║       ║    ║       ║");
            println!("    ║   ╔═══╝╔══╗╚═══╗   ║");
            println!("    ╚═══╝   ╔╝  ╚╗   ╚═══╝");
            println!("           ╔╝    ╚╗       ");
            println!("           ║      ║       ");
            println!("           ╚══════╝       ");
        }
        1 =>
        {
            println!("           ╔══════╗       ");
            println!("           ║██████║       ");
            println!("           ╚╗████╔╝       ");
            println!("    ╔═══╗   ╚╗██╔╝   ╔═══╗");
            println!("    ║   ╚═══╗╚══╝╔═══╝   ║");
            println!("    ║       ║    ║       ║");
            println!("    ║   ╔═══╝╔══╗╚═══╗   ║");
            println!("    ╚═══╝   ╔╝  ╚╗   ╚═══╝");
            println!("           ╔╝    ╚╗       ");
            println!("           ║      ║       ");
            println!("           ╚══════╝       ");
        }
        2 =>
        {
            println!("           ╔══════╗       ");
            println!("           ║      ║       ");
            println!("           ╚╗    ╔╝       ");
            println!("    ╔═══╗   ╚╗  ╔╝   ╔═══╗");
            println!("    ║   ╚═══╗╚══╝╔═══╝███║");
            println!("    ║       ║    ║███████║");
            println!("    ║   ╔═══╝╔══╗╚═══╗███║");
            println!("    ╚═══╝   ╔╝  ╚╗   ╚═══╝");
            println!("           ╔╝    ╚╗       ");
            println!("           ║      ║       ");
            println!("           ╚══════╝       ");
        }
        3 =>
        {
            println!("           ╔══════╗       ");
            println!("           ║      ║       ");
            println!("           ╚╗    ╔╝       ");
            println!("    ╔═══╗   ╚╗  ╔╝   ╔═══╗");
            println!("    ║   ╚═══╗╚══╝╔═══╝   ║");
            println!("    ║       ║    ║       ║");
            println!("    ║   ╔═══╝╔══╗╚═══╗   ║");
            println!("    ╚═══╝   ╔╝██╚╗   ╚═══╝");
            println!("           ╔╝████╚╗       ");
            println!("           ║██████║       ");
            println!("           ╚══════╝       ");
        }
        4 =>
        {
            println!("           ╔══════╗       ");
            println!("           ║      ║       ");
            println!("           ╚╗    ╔╝       ");
            println!("    ╔═══╗   ╚╗  ╔╝   ╔═══╗");
            println!("    ║███╚═══╗╚══╝╔═══╝   ║");
            println!("    ║███████║    ║       ║");
            println!("    ║███╔═══╝╔══╗╚═══╗   ║");
            println!("    ╚═══╝   ╔╝  ╚╗   ╚═══╝");
            println!("           ╔╝    ╚╗       ");
            println!("           ║      ║       ");
            println!("           ╚══════╝       ");
        }
        _ => unreachable!(),
    }
}