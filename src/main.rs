/*
 * 
 * Garden Bash
 * Created by Joel DeSante
 * Jan 27th, 2022
 * 
 * - Controls -
 * UP, DOWN, LEFT, RIGHT -> Movements the player
 * STEAL -> Steals five carrots from any player within 3 units of the player
 * 
 * - Instructions -
 * Each player has 100 turns to harvest the most carrots possible.
 * Move a top a carrot to pick it.
 * Move near a player and use the "steal" command to take at most five carrots from them.
 * 
 * 
 * TODO:
 * - Implement Carrot Collection
 *      - Implement Points Counting
 * - Make it so new carrots spawn on collection
 * - Make the game end and display a winner
 */

use colored::Colorize;

enum Direction {
    UP, DOWN, LEFT, RIGHT
}

struct Carrot {
    position: [u8; 2]
}

impl Carrot {
    fn get_icon(&self) -> String {
        return format!("{}", "V".truecolor(255, 215, 0));
    }

    fn spawn_carrot(board: &mut Board) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let x = rng.gen_range(0..15);
        let y = rng.gen_range(0..15);

        if board.get_carrot_at_coordinate(x, y).is_some() ||
           board.get_player_at_coordinate(x, y).is_some() 
        {
            Carrot::spawn_carrot(board);
            return;   
        }

        board.carrots.push(Carrot {
            position: [x, y]
        });
    }
}

struct Player {
    name: String,
    position: [u8; 2],
    score: u32
}

impl Player {
    fn walk(&mut self, direction: Direction) {
        let x = self.position[0] as i16;
        let y = self.position[1] as i16;

        match direction {
            Direction::UP => self.position[1] = (y - 1).clamp(0, 15) as u8,
            Direction::DOWN => self.position[1] = (y + 1).clamp(0, 15) as u8,
            Direction::LEFT => self.position[0] = (x - 1).clamp(0, 15) as u8,
            Direction::RIGHT => self.position[0] = (x + 1).clamp(0, 15) as u8
        }
    }
}

struct Board {
    steps: u8,
    players: Vec<Player>,
    carrots: Vec<Carrot>
}

impl Board {
    fn draw(&mut self) {
        // Meta data
        let current_turn = self.get_current_turn();
        let player = self.get_current_player();
        println!("  ┌───────────────────────────────•••");
        println!("  │ Turns Left {}; Player {}; Scr: {}", 100 - current_turn, player.name, player.score);
        println!("  └───────────────────────────────•••");
        
        // Board
        println!("    A B C D E F G H I J K L M N O P");
        println!("  ┌─────────────────────────────────┐");
        for y in 0..16 {
            print!("{}{}│ ", y + 1, if (y + 1) > 9 { "" } else { " " });    // <- This handles the numbers for the side of the board. Ternary operator doesnt seem to exist in Rust.
            for x in 0..16 {
                print!("{} ", self.get_character_for_coordinate(x, y));
            }
            print!("│\n");
        }
        println!("  └─────────────────────────────────┘");
    }

    /**
     * Checks the game state to see if a player needs
     * to be awarded a point for grabbing a carrot.
     */
    fn check_for_pickups(&mut self) {
        for (index, carrot) in self.carrots.iter_mut().enumerate() {
            for player in self.players.iter_mut() {
                if player.position[0] == carrot.position[0] && 
                   player.position[1] == carrot.position[1] 
                {
                    self.carrots.remove(index);
                    player.score += 1;
                    return;
                }
            }
        }
    }

    fn get_current_player(&mut self) -> &mut Player {
        let index = self.steps % (self.players.len() as u8);
        return self.players.get_mut(index as usize).unwrap();
    }

    /**
     * Converts the current game step to the current turn.
     * Depends on the number of players.
     * 
     * This function can cause a crash if there are no players set up. 
     * Make sure you have players before you begin a game!!!
     */
    fn get_current_turn(&self) -> u8 {
        return self.steps / self.players.len() as u8;
    }

    fn get_character_for_coordinate(&self, x: u8, y: u8) -> String {
        for player in self.players.iter() {
            if player.position[0] == x && player.position[1] == y {
                return format!("{}", player.name);
            }
        }

        for carrot in self.carrots.iter() {
            if carrot.position[0] == x && carrot.position[1] == y {
                return format!("{}", carrot.get_icon());
            }
        }

        return "◦".to_string();
    }

    fn get_carrot_at_coordinate(&self, x: u8, y: u8) -> Option<&Carrot> {
        for carrot in self.carrots.iter() {
            if carrot.position[0] == x && carrot.position[1] == y {
                return Option::from(carrot);
            }
        }
        return Option::None;
    }

    fn get_player_at_coordinate(&self, x: u8, y: u8) -> Option<&Player> {
        for player in self.players.iter() {
            if player.position[0] == x && player.position[1] == y {
                return Option::from(player);
            }
        }
        return Option::None;
    }

    fn await_command(&mut self) {

        let mut is_command_valid = false;

        while is_command_valid == false {
            let output = format!("Player {} Enter Command:", self.get_current_player().name);
            println!("{}", output.yellow().bold());

            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();

            match line.to_lowercase().trim() {
                "up" => { 
                    self.get_current_player().walk(Direction::UP);
                    is_command_valid = true;
                },
                "down" => { 
                    self.get_current_player().walk(Direction::DOWN);
                    is_command_valid = true;
                },
                "left" => {
                    self.get_current_player().walk(Direction::LEFT);
                    is_command_valid = true;
                },
                "right" => {
                    self.get_current_player().walk(Direction::RIGHT);
                    is_command_valid = true;
                },
                "steal" => {
                    is_command_valid = true;
                },
                _ => println!("{}", "Invalid Command! Try Again.\n".red().bold())
            }
        }
    }

}

fn main() {

    let mut board = Board {
        steps: 0,
        players: Vec::new(),
        carrots: Vec::new()
    };

    board.players.push(Player {
        name: format!("{}", "1".red()),
        position: [0, 0],
        score: 0
    });

    board.players.push(Player {
        name: format!("{}", "2".red()),
        position: [15, 15],
        score: 0
    });

    // Add carrots to the board
    for _carrot in 0..4 {
        Carrot::spawn_carrot(&mut board);
    }

    while board.get_current_turn() < 100 {
        //clearscreen::clear().unwrap();
        board.draw();
        board.await_command();
        board.check_for_pickups();
        board.steps += 1;
    }

    clearscreen::clear().unwrap();
    println!("{}", "GAME OVER YOU HUNK OF SHIT".bold());
    // End of game
}
