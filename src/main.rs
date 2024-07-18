use std::time::Duration;
use std::thread;

const ARRAY_SIZE: usize = 17;

fn print_matrix(array: &[[bool; ARRAY_SIZE]; ARRAY_SIZE], size: usize) {
    print!("┏");
    for _i in 0..size * 6 - 1{
        print!("━");
    }
    println!("┓");
    for i in 0..size {
        print!("┃ ");
        for j in 0..size - 1 {
            if array[j][i] == true {
                print!("{0: <3} | ", " X");
            } else {
                print!("{0: <3} | ", "");
            }
        }
        if array[size - 1][i] == true {
            print!("{0: <3} ", " X");
        } else {
            print!("{0: <3} ", "");
        }
        println!("┃")
    }
    print!("┗");
    for _i in 0..size * 6 - 1{
        print!("━");
    }
    println!("┛");
}

fn get_living_neighbours(array: &[[bool; ARRAY_SIZE]; ARRAY_SIZE], x: usize, y: usize) -> i16 {
    let mut count: i16 = 0;
    let start: usize;
    let end: usize;

    if x == 0 {
        start = x;
    } else {
        start = x - 1;
    }

    if x == ARRAY_SIZE - 1 {
        end = x + 1;
    } else {
        end = x + 2;
    }

    if y + 1 < ARRAY_SIZE {
        for i in start..end {
            if array[i][y + 1] == true {
                count += 1;
            }
        } 
    }

    for i in start..end {
        if i == x {continue;}
        if array[i][y] == true {
            count += 1;
        }
    }

    if y > 0 {
        for i in start..end {
            if array[i][y - 1] == true {
                count += 1;
            }
        }
    }
    count
}

fn main_loop(array: &mut[[bool; ARRAY_SIZE]; ARRAY_SIZE], old_array: &[[bool; ARRAY_SIZE]; ARRAY_SIZE], size: usize) {
    let mut count: i16;
    for i in 0..size {
        for j in 0..size {
            count = get_living_neighbours(old_array, j, i);

            if array[j][i] == true {
                if count <= 1 {
                    array[j][i] = false;
                } else if count <= 3 {
                    continue;
                } else {
                    array[j][i] = false;
                }
            } else {
                if count == 3 {
                    array[j][i] = true;
                }
            }
        }
    }
}


fn main() {
    let mut game: [[bool; ARRAY_SIZE]; ARRAY_SIZE] = [[false; ARRAY_SIZE]; ARRAY_SIZE];

    game[4][2] = true;
    game[5][2] = true;
    game[6][2] = true;
    game[10][2] = true;
    game[11][2] = true;
    game[12][2] = true;

    game[2][4] = true;
    game[7][4] = true;
    game[9][4] = true;
    game[14][4] = true;

    game[2][5] = true;
    game[7][5] = true;
    game[9][5] = true;
    game[14][5] = true;

    game[2][6] = true;
    game[7][6] = true;
    game[9][6] = true;
    game[14][6] = true;

    game[4][7] = true;
    game[5][7] = true;
    game[6][7] = true;
    game[10][7] = true;
    game[11][7] = true;
    game[12][7] = true;

    game[4][9] = true;
    game[5][9] = true;
    game[6][9] = true;
    game[10][9] = true;
    game[11][9] = true;
    game[12][9] = true;

    game[2][10] = true;
    game[7][10] = true;
    game[9][10] = true;
    game[14][10] = true;

    game[2][11] = true;
    game[7][11] = true;
    game[9][11] = true;
    game[14][11] = true;

    game[2][12] = true;
    game[7][12] = true;
    game[9][12] = true;
    game[14][12] = true;

    game[4][14] = true;
    game[5][14] = true;
    game[6][14] = true;
    game[10][14] = true;
    game[11][14] = true;
    game[12][14] = true;

    let mut old_game: [[bool; ARRAY_SIZE]; ARRAY_SIZE] = game;

    loop {
        print_matrix(&game, ARRAY_SIZE);
        thread::sleep(Duration::from_secs(1));
        main_loop(&mut game, &old_game, ARRAY_SIZE);
        old_game = game;
    }
}







