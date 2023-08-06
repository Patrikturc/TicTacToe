use std::io;
use rand::Rng;

//Noughts and Crosses (Tic-Tac-Toe)
//Author: B00389261
//Version: 1.0.0
//Last Update: 22.04.2022
fn main(){
  
  let st_play = String::new();
  
  let mut i = 0;

  //Welcome the player
  println!("Hello there! Wanna play? Please enter Y/N\n");

  //Request valid input, after 10 invalid inputs assume the player wants to quit
  while st_play.to_lowercase().ne("y") && st_play.to_lowercase().ne("n") && i < 10{
    let mut st_play = String::new();
    i += 1;
    
    //Request input from the player
    io::stdin().read_line(&mut st_play)
      .expect("Read line Error");

    //Trim the String object
    let st_play = st_play.trim();
    if st_play.to_lowercase().eq("y") {
        println!("\nGood Luck!\n_______________________________________\n");
        game();
        i = 0;
      } else if st_play.to_lowercase().eq("n") {
      println!("See you, thanks for playing.");
      break;
    } else {
      println!("\nSorry, didn't understand that. Please type Y for yes or N for no\n");
    }
  }
}

//Rundom Number Generator
fn init() -> u32 {
  rand::thread_rng().gen_range(1..3)
}

//Game function - manages the game, calls the relevant player, passes turn and monitors game state
fn game() {
  //Boolean to determine who currently plays
  let mut current_player;
  //Moves counter
  let mut count = 0;
  //Board Array
  let mut board = ["1","2","3","4","5","6","7","8","9"];
  //Randomly decide who plays Noughts or Crosses

  //Share the decision with the player
  if init() == 1 {
    println!("You are playing as Crosses\n\nGame Board:");
    print_map(board);
    current_player = false;
  } else {
    println!("You are playing as Noughts\n\nGame Board:");
    current_player = true;
  }

  //Main Game Loop
  while count <= 8 {
    //Temporary storage of the board
    let temp_board = board;
    //Check whos' move is it
    if current_player {
      //Call a simple AI function to make a move
      board = ai(board, count);
      //Check if win state was achieved
      if check_win(board) {
        print_map(board);
        println!("Sorry, you lost this one.");
        println!("___________________________\nWould you like to play again? Enter Y/N");
        break;
      } else if board_full(board) { //Check if it is draw
        print_map(board);
        println!("Draw!");
        println!("___________________________\nWould you like to play again? Enter Y/N");
        break;
      }
      //Pass the turn
      current_player = false;
      //Increase turn count
      count += 1;
    } else {
      board = player_move(board, count);
      if check_win(board) {
        print_map(board);
        println!("Well done! You are the Winner");
        println!("___________________________\nWould you like to play again? Enter Y/N");
        break;
      } else if board_full(board) {
        print_map(board);
        println!("Draw!");
        println!("___________________________\nWould you like to play again? Enter Y/N");
        break;
      }
      if temp_board != board {
        current_player = true;
        count += 1;
      }
    }
    //inform player of the state of the board
    print_map(board);
  }
}

//Win check function
fn check_win(b: [&str; 9]) -> bool {
  let win = true;
  let i = 0;

  //Check all 8 possible win states, returns false if no condition is met
  if b[i] == b[i+1] && b[i] == b[i+2] { // Horizontal 1 2 3
    return win;
  } else if b[i] == b[i+3] && b[i] == b[i+6] { // Vertical 1 4 7
    return win;
  } else if b[i] == b[i+4] && b[i] == b[i+8] { // Diagonal 1 5 9
    return win;
  } else if b[i+1] == b[i+4] && b[i+1] == b[i+7] { // vertical 2 5 8
    return win;
  } else if b[i+2] == b[i+5] && b[i+2] == b[i+8] { // vertical 3 6 9
    return win;
  } else if b[i+2] == b[i+4] && b[i+2] == b[i+6] { // Diagonal 3 5 7
    return win;
  } else if b[i+3] == b[i+4] && b[i+3] == b[i+5] { // Horizontal 4 5 6
    return win;
  } else if b[i+6] == b[i+7] && b[i+6] == b[i+8] { // Horizontal 7 8 9
    return win;
  } else { // No winner
    return !win;
  }
}

//Is board full check function, returns true if yes.
fn board_full(b: [&str; 9]) -> bool {
  let full = true;
  for s in b {
    if s != "X" && s != "O" {
      return !full;
    }
  }
  return full;
}

//Print function prints the board
fn print_map(b: [&str; 9]){
  let mut i = 0;
    for s in b {
      if i <= 1 || i <= 4 && i >= 3 || i <= 7 && i >= 6{
        print!("{}|", s)
      } else if i == 2 || i == 5 {
        print!("{}\n-----\n", s)
      } else if i == 8 {
        print!("{}\n\n", s);
      }
      i += 1;
    }
}

//Simplest computer decision making algorithm. Selects the next available place on the map. 
//If not careful, loss is possible, otherwise player should be able to control the outcome of the game.
fn ai(mut b: [&str; 9], count: u32) -> [&str; 9] {
  
  let mut i : usize = 0;

  for s in b {
    if s != "X" && s != "O" {
      if count % 2 == 1 {
        println!("\nComputer Chose: {}\n", b[i]);
        b[i] = "O";
        return b;
      }
      else if count % 2 == 0 {
        println!("\nComputer Selected: {}\n", b[i]);
        b[i] = "X";
        return b;
      }
    }
    i += 1;
  }
  return b;
}

//Player move function. Extensive testing was done to ensure program will not result in an error
fn player_move(mut b: [&str; 9], count: u32) -> [&str; 9] {
  //Variable to mark the first input attempt, following attempt will result in a different text displayed
  let mut i = 0;
  //Loop ensuring input is correct
  loop {
    //condition to find out which request should be displayed
    if i == 0{
      println!("To which location would you like to insert your symbol - Enter number between 1 - 9");
      i += 1;
    } else {
      println!("Sorry, that didn't work.\nPlease select a valid location - Number between 1 and 9");
    }
    
    let mut st_play = String::new();

    io::stdin().read_line(&mut st_play)
      .expect("Failed to read line");

    //Check if input is valid
    let st_play: usize = match st_play.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    if st_play > b.len() || st_play == 0{
      continue;
    }
    println!("\nYour Choice was: {}\n", st_play);
    for _s in b {
      if b[st_play-1] != "X" && b[st_play-1] != "O" {
        if count % 2 == 1 {
          b[st_play-1] = "O";
          break;
        } else if count % 2 == 0 {
          b[st_play-1] = "X";
          break;
        } 
      } else {
        println!("Sorry, that spot is taken");
        break;
      }
    } 
    return b;
  }
}