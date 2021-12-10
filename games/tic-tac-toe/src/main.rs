use std::io;
use rand::prelude::*;
use std::io::prelude::*;
use crossterm::event::*;

static mut random : ThreadRng = rand::thread_rng();
static mut closeRequested : bool = false;
static mut playerTurn : bool = true;
static mut board : [[char; 3]; 3] = [[' '; 3]; 3];

fn main()
{
    while !closeRequested
    {
        board = [[' '; 3]; 3];
        while !closeRequested
        {
            if playerTurn
            {
                PlayerTurn();
                if CheckForThree('X')
                {
                    EndGame("You Win.");
                    break;
                }
            }
            else
            {
                ComputerTurn();
                if CheckForThree('O')
                {
                    EndGame("You Lose.");
                    break;
                }
            }
            playerTurn = !playerTurn;
            if CheckForFullBoard()
            {
                EndGame("Draw.");
                break;
            }
        }
        if !closeRequested
        {
            println!();
            println!("Play Again [enter], or quit [escape]?");
        GetInput:
            switch (Console.ReadKey(true).Key)
            {
                case ConsoleKey.Enter: break;
                case ConsoleKey.Escape:
                    closeRequested = true;
                    Console.Clear();
                    break;
                default: goto GetInput;
            }
        }
    }

    println!("tic-tac-toe");
    println!();
    println!("Still under development...");
}

fn PlayerTurn()
{
    let mut row : u32 = 0;
    let mut column : u32 = 0;
    let mut moved : bool = false;
    while !moved && !closeRequested
    {
        clear();
        RenderBoard();
        println!();
        println!("Choose a valid position and press enter.");
        cursor(row * 4 + 1, column * 6 + 1);
        switch (Console.ReadKey(true).Key)
        {
            case ConsoleKey.UpArrow:    row = row <= 0 ? 2 : row - 1; break;
            case ConsoleKey.DownArrow:  row = row >= 2 ? 0 : row + 1; break;
            case ConsoleKey.LeftArrow:  column = column <= 0 ? 2 : column - 1; break;
            case ConsoleKey.RightArrow: column = column >= 2 ? 0 : column + 1; break;
            case ConsoleKey.Enter:
                if (board[row, column] is ' ')
                {
                    board[row, column] = 'X';
                    moved = true;
                }
                break;
            case ConsoleKey.Escape:
                Console.Clear();
                closeRequested = true;
                break;
        }
    }
}

fn ComputerTurn()
{
    let possibleMoves: Vec<(usize, usize)> = Vec::new();
    for i in 0..3
    {
        for j in 0..3
        {
            if board[i][j] == ' '
            {
                possibleMoves.push((i, j));
            }
        }
    }
    let index = random.gen_range(0..(possibleMoves.len()));
    let (x, y) = possibleMoves[index];
    board[x][y] = 'O';
}

fn CheckForThree(c: char) -> bool
{
    return
        board[0][0] == c && board[1][0] == c && board[2][0] == c ||
        board[0][1] == c && board[1][1] == c && board[2][1] == c ||
        board[0][2] == c && board[1][2] == c && board[2][2] == c ||
        board[0][0] == c && board[0][1] == c && board[0][2] == c ||
        board[1][0] == c && board[1][1] == c && board[1][2] == c ||
        board[2][0] == c && board[2][1] == c && board[2][2] == c ||
        board[0][0] == c && board[1][1] == c && board[2][2] == c ||
        board[2][0] == c && board[1][1] == c && board[0][2] == c;
}

fn CheckForFullBoard() -> bool
{
    return
        board[0][0] != ' ' && board[1][0] != ' ' && board[2][0] != ' ' &&
        board[0][1] != ' ' && board[1][1] != ' ' && board[2][1] != ' ' &&
        board[0][2] != ' ' && board[1][2] != ' ' && board[2][2] != ' ';
}

fn RenderBoard()
{
    println!();
    println!(" {}  ║  {}  ║  {}", board[0][0], board[0][1], board[0][2]);
    println!("    ║     ║");
    println!(" ═══╬═════╬═══");
    println!("    ║     ║");
    println!(" {}  ║  {}  ║  {}", board[1][0], board[1][1], board[1][2]);
    println!("    ║     ║");
    println!(" ═══╬═════╬═══");
    println!("    ║     ║");
    println!(" {}  ║  {}  ║  {}", board[2][0], board[2][1], board[2][2]);
}

fn EndGame(message: &str)
{
    clear();
    RenderBoard();
    println!();
    print!("{}", message);
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

fn cursor(row : u32, column : u32)
{
    print!("\033[{};{}H", row, column);
}
