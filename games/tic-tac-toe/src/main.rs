use std::io;
use rand::prelude::*;
use std::io::prelude::*;
use crossterm::event::*;

static mut closeRequested : bool = false;
static mut playerTurn : bool = true;
static mut board : [[char; 3]; 3] = [[' '; 3]; 3];

fn main()
{
    unsafe
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
                let mut validInput = false;
                while !closeRequested && !validInput
                {
                    let event = read().unwrap();
                    if event == Event::Key(KeyCode::Enter.into())
                    {
                        validInput = true;
                    }
                    if event == Event::Key(KeyCode::Esc.into())
                    {
                        closeRequested = true;
                    }
                }
            }
        }
    }
}

unsafe fn PlayerTurn()
{
    let mut row    : usize = 0;
    let mut column : usize = 0;
    let mut moved : bool = false;
    while !moved && !closeRequested
    {
        clear();
        RenderBoard();
        println!();
        println!("Choose a valid position and press enter.");
        cursor(row * 4 + 1, column * 6 + 1);
        let event = read().unwrap();

        if event == Event::Key(KeyCode::Left.into()) 
        {
            if row == 0 { row = 2; }
            else { row -= 1; }
        }
        if event == Event::Key(KeyCode::Right.into())
        {
            if row == 2 { row = 0; }
            else { row += 1; }
        }
        if event == Event::Key(KeyCode::Up.into())    
        {
            if column == 0 { column = 2;}
            else { column -= 1; }
        }
        if event == Event::Key(KeyCode::Down.into())
        {
            if column == 2 { column = 0; }
            else { column += 1; }
        }
        if event == Event::Key(KeyCode::Esc.into())
        {
            closeRequested = true;
        }
        if event == Event::Key(KeyCode::Enter.into())
        {
            if (board[row][column] == ' ')
            {
                board[row][column] = 'X';
                moved = true;
            }
        }
        if row    < 0 { row = 2; }
        if row    > 2 { row = 0; }
        if column < 0 { column = 2; }
        if column > 2 { column = 0; }
    }
}

unsafe fn ComputerTurn()
{
    let mut random : ThreadRng = rand::thread_rng();
    let mut possibleMoves: Vec<(usize, usize)> = Vec::new();
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

unsafe fn CheckForThree(c: char) -> bool
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

unsafe fn CheckForFullBoard() -> bool
{
    return
        board[0][0] != ' ' && board[1][0] != ' ' && board[2][0] != ' ' &&
        board[0][1] != ' ' && board[1][1] != ' ' && board[2][1] != ' ' &&
        board[0][2] != ' ' && board[1][2] != ' ' && board[2][2] != ' ';
}

unsafe fn RenderBoard()
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

unsafe fn EndGame(message: &str)
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

fn cursor(row : usize, column : usize)
{
    print!("\033[{};{}H", row, column);
}
