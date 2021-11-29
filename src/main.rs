use rand::Rng;

fn game_of_life(grid: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    // Get number of rows & columns
    let rows = grid.len();
    let columns = grid[0].len();

    // Empty grid that holds future generations
    // Has mut keyword to show that it is a mutable variable
    let mut future: Vec<Vec<i8>> = vec![vec![0; rows]; columns];

    for row in 0..rows {
        for column in 0..columns {
            let cell_state = grid[row][column];
            let mut live_neighbors = 0;
            for x in -1i8..=1 {
                for y in -1i8..=1 {

                    // Position of the neighbor 
                    let new_x = (row as i8) + x;
                    let new_y = (column as i8) + y; 

                    // Check if the new position is inbounds
                    if new_x > 0 && new_y > 0 && new_x < rows as i8 && new_y < columns as i8 {
                        live_neighbors += grid[new_x as usize][new_y as usize];
                    }
                }
            }
            // Remove current state to get the number of alive neighbours
            live_neighbors -= cell_state;
            
            // Applying rules of the game of life to future generations
            if cell_state == 1 && live_neighbors < 2 {
                future[row][column] = 0;
            } else if cell_state == 1 && live_neighbors > 3 {
                future[row][column] = 0;
            } else if cell_state == 0 && live_neighbors == 3 {
                future[row][column] = 1;
            } else {
                future[row][column] = cell_state;
            }
        }
    }
    future
}

fn main() {
    // set the number of rows and columns of the grid
    let mut rng = rand::thread_rng();
    let row_initial: usize = rng.gen::<i8>();
    let column_initial: usize = rng.gen::<i8>();
    let (rows, cols) = (row_initial, column_initial);

    // create the grid
    let mut grid: Vec<Vec<i8>> = vec![vec![0; cols]; rows];

    // set the initial state of the grid (blinker)
    grid[1][2] = 1;
    grid[2][2] = 1;
    grid[3][2] = 1;

    // print the initial state of the grid;
    println!("Initial grid:");
    grid.iter().for_each(|i| {
        println!("{:?}", i);
    });

    println!("");

    // Number of generations
    const ITR: u8 = 5;

    // compute and print the next generation
    for i in 0..ITR {
        grid = game_of_life(&grid);

        println!("Generation {}:", i+1);
        grid.iter().for_each(|i| {
            println!("{:?}", i);
        });
        println!("");
    }
}
