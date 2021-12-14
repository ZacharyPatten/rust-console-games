use std::io;
use rand::prelude::*;
use std::io::prelude::*;
use crossterm::event::*;

static mut CLOSE_REQUESTED : bool = false;
static mut PLAYER_TURN : bool = true;
static mut BOARD : [[char; 3]; 3] = [[' '; 3]; 3];

fn main()
{
    unsafe
    {
        while !CLOSE_REQUESTED
        {
            BOARD = [[' '; 3]; 3];
            while !CLOSE_REQUESTED
            {
                if PLAYER_TURN
                {
                    player_turn();
                    if check_for_three('X')
                    {
                        end_game("You Win.");
                        break;
                    }
                }
                else
                {
                    computer_turn();
                    if check_for_three('O')
                    {
                        end_game("You Lose.");
                        break;
                    }
                }
                PLAYER_TURN = !PLAYER_TURN;
                if check_for_full_board()
                {
                    end_game("Draw.");
                    break;
                }
            }
            if !CLOSE_REQUESTED
            {
                println!();
                println!("Play Again [enter], or quit [escape]?");
                let mut valid_input = false;
                while !CLOSE_REQUESTED && !valid_input
                {
                    let event = read().unwrap();
                    if event == Event::Key(KeyCode::Enter.into())
                    {
                        valid_input = true;
                    }
                    if event == Event::Key(KeyCode::Esc.into())
                    {
                        CLOSE_REQUESTED = true;
                    }
                }
            }
        }
    }
}

unsafe fn player_turn()
{
    let mut row    : usize = 0;
    let mut column : usize = 0;
    let mut moved : bool = false;
    while !moved && !CLOSE_REQUESTED
    {
        clear();
        render_board();
        println!();
        println!("Choose a valid position and press enter.");
        cursor(row * 4 + 1, column * 6 + 1);
        let event = read().unwrap();
        if event == Event::Key(KeyCode::Up.into()) 
        {
            if row == 0 { row = 2; }
            else { row -= 1; }
        }
        if event == Event::Key(KeyCode::Down.into())
        {
            if row == 2 { row = 0; }
            else { row += 1; }
        }
        if event == Event::Key(KeyCode::Left.into())    
        {
            if column == 0 { column = 2;}
            else { column -= 1; }
        }
        if event == Event::Key(KeyCode::Right.into())
        {
            if column == 2 { column = 0; }
            else { column += 1; }
        }
        if event == Event::Key(KeyCode::Esc.into())
        {
            CLOSE_REQUESTED = true;
        }
        if event == Event::Key(KeyCode::Enter.into())
        {
            if BOARD[row][column] == ' '
            {
                BOARD[row][column] = 'X';
                moved = true;
            }
        }
    }
    clear();
}

unsafe fn computer_turn()
{
    let mut random : ThreadRng = rand::thread_rng();
    let mut possible_moves: Vec<(usize, usize)> = Vec::new();
    for i in 0..3
    {
        for j in 0..3
        {
            if BOARD[i][j] == ' '
            {
                possible_moves.push((i, j));
            }
        }
    }
    let index = random.gen_range(0..(possible_moves.len()));
    let (x, y) = possible_moves[index];
    BOARD[x][y] = 'O';
}

unsafe fn check_for_three(c: char) -> bool
{
    return
        BOARD[0][0] == c && BOARD[1][0] == c && BOARD[2][0] == c ||
        BOARD[0][1] == c && BOARD[1][1] == c && BOARD[2][1] == c ||
        BOARD[0][2] == c && BOARD[1][2] == c && BOARD[2][2] == c ||
        BOARD[0][0] == c && BOARD[0][1] == c && BOARD[0][2] == c ||
        BOARD[1][0] == c && BOARD[1][1] == c && BOARD[1][2] == c ||
        BOARD[2][0] == c && BOARD[2][1] == c && BOARD[2][2] == c ||
        BOARD[0][0] == c && BOARD[1][1] == c && BOARD[2][2] == c ||
        BOARD[2][0] == c && BOARD[1][1] == c && BOARD[0][2] == c;
}

unsafe fn check_for_full_board() -> bool
{
    return
        BOARD[0][0] != ' ' && BOARD[1][0] != ' ' && BOARD[2][0] != ' ' &&
        BOARD[0][1] != ' ' && BOARD[1][1] != ' ' && BOARD[2][1] != ' ' &&
        BOARD[0][2] != ' ' && BOARD[1][2] != ' ' && BOARD[2][2] != ' ';
}

unsafe fn render_board()
{
    println!();
    println!(" {}  ║  {}  ║  {}", BOARD[0][0], BOARD[0][1], BOARD[0][2]);
    println!("    ║     ║");
    println!(" ═══╬═════╬═══");
    println!("    ║     ║");
    println!(" {}  ║  {}  ║  {}", BOARD[1][0], BOARD[1][1], BOARD[1][2]);
    println!("    ║     ║");
    println!(" ═══╬═════╬═══");
    println!("    ║     ║");
    println!(" {}  ║  {}  ║  {}", BOARD[2][0], BOARD[2][1], BOARD[2][2]);
}

unsafe fn end_game(message: &str)
{
    clear();
    render_board();
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
    print!("\x1B[{};{}H", row + 1, column + 1);
    flush();
}