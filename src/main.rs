// tic tac toe game in rust

// print player function
fn print_player(player: &char) {
    print!("{}", player);
}

// draws the board on the screen
fn draw(board_state: &[char]) {
    println!("\n");

    // for i in the number of rows
    for i in (0..3).rev() {
        let offset = i * 3;

        print!("-------------\n| ");
        print_player(&board_state[offset]);
        print!(" | ");
        print_player(&board_state[offset + 1]);
        print!(" | ");
        print_player(&board_state[offset + 2]);
        println!(" |");
    }
    println!("-------------")
}

// get input from user
fn get_user_input(board_state: &mut [char], player: char) {
    loop {
        println!("Player {}, enter a number: ", player);

        // mutable input, if not input return
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Could not read input, try again.");
            continue;
        }

        // if number is a number
        if let Ok(number) = input.trim().parse::<usize>() {
            if number < 1 || number > 9 {
                println!("Number out of range.");
                continue;
            }

            let number = number - 1;

            if board_state[number] == 'X' || board_state[number] == 'O' {
                print!("This field is already taken by '");
                print_player(&board_state[number]);
                println!("'.");
                continue;
            }

            board_state[number] = player;

            break;
        } else {
            println!("Only numbers are allowed.");
            continue;
        }
    }
}

// check the board for a winner
fn check_winner(board_state: &[char]) -> bool {
    for tmp in 0..3 {
        if board_state[tmp] == board_state[tmp + 3] && board_state[tmp] == board_state[tmp + 6] {
            return true;
        }

        let tmp = tmp * 3;

        if board_state[tmp] == board_state[tmp + 1] && board_state[tmp] == board_state[tmp + 2] {
            return true;
        }
    }

    if (board_state[0] == board_state[4] && board_state[0] == board_state[8])
        || (board_state[2] == board_state[4] && board_state[2] == board_state[6])
    {
        return true;
    }

    false
}

#[inline(always)]
fn is_over(board_state: &[char]) -> bool {
    board_state.iter().all(|&v| v == 'X' || v == 'O')
}

fn main() {
    // board variable
    let mut board_state = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut current_player = 'X';

    loop {
        draw(&board_state);
        get_user_input(&mut board_state, current_player);

        if check_winner(&board_state) {
            draw(&board_state);
            print!("Player ");
            print_player(&current_player);
            println!(" won!");
            break;
        }

        if is_over(&board_state) {
            draw(&board_state);
            println!("All spaces are filled, No winner.");
            break;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' }
    }
}
