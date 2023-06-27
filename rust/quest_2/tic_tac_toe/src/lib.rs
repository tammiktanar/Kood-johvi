#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    let mut res = "Tie";
    if diagonals("X", &table) || horizontal("X", &table) || vertical("X", &table) {
        res = "player X won";
    } else if diagonals("O", &table) || horizontal("O", &table) || vertical("O", &table) {
        res = "player O won";
    }

    return res.to_string()
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut done = false;
    // 00 11 22 33

    for (nr , _) in table.iter().enumerate(){
        if table[nr][nr].to_string() == player.to_string(){
            done = true;
        } else {

            done = false;
            break
        }
    }


    // 03 12 21 

    if !done {

        for (nr , _) in table.iter().enumerate(){
            if table[nr][3 - (nr+1)].to_string() == player.to_string(){
                done = true;
            } else {

                done = false;
                break
            }
        }

    }

    return done
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut nr = 0;
    let mut done = false;

    for _ in table {
        if done {
            break
        }

        for column in &table[nr] {
            if column.to_string() == player.to_string(){
                done = true;
            } else {

                done = false;
                break
            }
        }

        nr+=1;    
    }

    return done
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut done = false;

    for (cur_column ,_) in table.iter().enumerate(){ 
        if done {
            break
        }

        for (cur_row , _) in table.iter().enumerate(){ 

            if table[cur_row][cur_column].to_string() == player.to_string(){
                done = true;
            } else {

                done = false;
                break
            }
        }
    }

    return done
}