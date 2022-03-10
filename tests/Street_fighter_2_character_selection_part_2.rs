/**
 * @Author: BearV
 * @Description: https://www.codewars.com/kata/58583922c1d5b415b00000ff/train/rust
 * @Date: create in 2022/3/9 3:32 PM
 */


struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }
}

fn move_handle(move_x: i32, move_y: i32, max_x: i32, max_y: i32, target_x: i32, target_y: i32, check_vec: &mut Vec<String>, fighters: &[&[&str]], d: Direction) -> (i32, i32) {
    let mut x = target_x;
    let mut y = target_y;


    x = if x + move_x > max_x {
        0
    } else if x + move_x < 0 {
        max_x
    } else {
        x + move_x
    };
    y = if (y == 0 && move_y < 0) || (y == max_y && move_y > 0) { y } else if y + move_y > max_y {
        0
    } else if y + move_y < 0 {
        max_y
    } else {
        y + move_y
    };

    let check_name = fighters[y as usize][x as usize];
    println!("{:?}", d);
    println!("check_name:{}", check_name);
    if check_name != "" {
        check_vec.push(check_name.to_string());
    } else {
        if move_x == 0 {
            x = target_x;
            y = target_y;
            let check_name = fighters[y as usize][x as usize];
            check_vec.push(check_name.to_string());
        }
        if move_y == 0 {
            while fighters[y as usize][x as usize] == "" {
                x = if x + move_x > max_x {
                    0
                } else if x + move_x < 0 {
                    max_x
                } else {
                    x + move_x
                };
            }
            let check_name = fighters[y as usize][x as usize];
            check_vec.push(check_name.to_string());
        }
    };
    (x, y)
}


fn super_street_fighter_selection(
    fighters: &[&[&str]],
    position: Position,
    moves: &[Direction],
) -> Vec<String> {
    let max_y = fighters.len() as i32 - 1;
    let max_x = fighters[0].len() as i32 - 1;
    let mut target_x = position.x;
    let mut target_y = position.y;

    let mut check_vec: Vec<String> = vec![];

    if moves.len() == 0 {
        return check_vec;
    }

    for i in moves {
        match i {
            Direction::Up => {
                let a = move_handle(0, -1, max_x, max_y, target_x, target_y, &mut check_vec, fighters, Direction::Up);
                target_x = a.0;
                target_y = a.1;
            }
            Direction::Down => {
                let a = move_handle(0, 1, max_x, max_y, target_x, target_y, &mut check_vec, fighters, Direction::Down);
                target_x = a.0;
                target_y = a.1;
            }
            Direction::Left => {
                let a = move_handle(-1, 0, max_x, max_y, target_x, target_y, &mut check_vec, fighters, Direction::Left);
                target_x = a.0;
                target_y = a.1;
            }
            Direction::Right => {
                let a = move_handle(1, 0, max_x, max_y, target_x, target_y, &mut check_vec, fighters, Direction::Right);
                target_x = a.0;
                target_y = a.1;
            }
        }
    }
    println!("check_vec:{:?}", check_vec);

    check_vec
}

#[cfg(test)]
mod tests {
    use super::{super_street_fighter_selection, Direction, Position};

    #[rustfmt::skip]
    const FIGHTERS_A: [&[&'static str]; 3] = [
        &["", "Ryu", "E.Honda", "Blanka", "Guile", ""],
        &["Balrog", "Ken", "Chun Li", "Zangief", "Dhalsim", "Sagat"],
        &["Vega", "T.Hawk", "Fei Long", "Deejay", "Cammy", "M.Bison"],
    ];

    #[rustfmt::skip]
    const FIGHTERS_B: [&[&'static str]; 3] = [
        &["", "Ryu", "E.Honda", "Cammy", "Blanka", "Guile", "", "Chun Li"],
        &["Balrog", "Ken", "Chun Li", "", "M.Bison", "Zangief", "Dhalsim", "Sagat"],
        &["Vega", "", "Fei Long", "Balrog", "Deejay", "Cammy", "", "T.Hawk"],
    ];

    #[rustfmt::skip]
    const FIGHTERS_C: [&[&'static str]; 6] = [
        &["", "Ryu", "E.Honda", "Cammy"],
        &["Balrog", "Ken", "Chun Li", ""],
        &["Vega", "", "Fei Long", "Balrog"],
        &["Blanka", "Guile", "", "Chun Li"],
        &["M.Bison", "Zangief", "Dhalsim", "Sagat"],
        &["Deejay", "Cammy", "", "T.Hawk"],
    ];


    #[test]
    fn no_selection() {
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 0), &[] as &[Direction]),
            vec![] as Vec<String>,
            "it should work with no selection cursor moves",
        );
    }

    #[test]
    fn empty_vertical_spaces_single_move() {
        use Direction::*;

        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 1), &[Up]),
            vec!["Balrog"],
            "it should stop on empty spaces vertically",
        );
    }

    #[test]
    fn empty_vertical_spaces_multiple_moves() {
        use Direction::*;

        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 1), &[Up, Up, Up, Up]),
            vec!["Balrog", "Balrog", "Balrog", "Balrog"],
            "it should stop on empty spaces vertically (up)",
        );

        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 1), &[Down, Down, Down, Down]),
            vec!["Vega", "Vega", "Vega", "Vega"],
            "it should stop on empty spaces vertically (down)",
        );

        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(5, 1), &[Up, Up, Up, Up]),
            vec!["Sagat", "Sagat", "Sagat", "Sagat"],
            "it should stop on empty spaces vertically (up)",
        );

        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(5, 1), &[Down, Down, Down, Down]),
            vec!["M.Bison", "M.Bison", "M.Bison", "M.Bison"],
            "it should stop on empty spaces vertically (down)",
        );
    }

    #[test]
    fn rotate_horizontally() {
        use Direction::*;

        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(2, 0), &[Left, Left, Left, Left, Left, Left, Left, Left]),
            vec!["Ryu", "Guile", "Blanka", "E.Honda", "Ryu", "Guile", "Blanka", "E.Honda"],
            "it should rotate horizontally (left)",
        );

        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(3, 1), &[Left, Left, Left, Left, Left, Left, Left, Left]),
            vec!["Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "Chun Li", "Ken"],
            "it should rotate horizontally (left)",
        );
    }

    #[test]
    fn rotate_horizontally_with_empty_spaces() {
        use Direction::*;

        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(2, 0), &[Right, Right, Right, Right, Right, Right, Right, Right]),
            vec!["Blanka", "Guile", "Ryu", "E.Honda", "Blanka", "Guile", "Ryu", "E.Honda"],
            "it should rotate horizontally with empty spaces",
        );
    }

    #[test]
    fn rotate_on_all_rows() {
        use Direction::*;

        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(2, 0), &[Right, Right, Right, Right, Right, Right, Down, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Down, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right]),
            vec!["Blanka", "Guile", "Ryu", "E.Honda", "Blanka", "Guile", "Dhalsim", "Zangief", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Cammy", "M.Bison", "Vega", "T.Hawk", "Fei Long", "Deejay", "Cammy", "M.Bison", "Vega", "T.Hawk", "Fei Long", "Deejay", "Cammy"],
            "it should rotate on all rows",
        );

        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_B[..], Position::new(2, 0), &[Right, Right, Right, Right, Right, Right, Down, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Down, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right]),
            vec!["Cammy", "Blanka", "Guile", "Chun Li", "Ryu", "E.Honda", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "M.Bison", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "Cammy", "T.Hawk", "Vega", "Fei Long", "Balrog", "Deejay", "Cammy", "T.Hawk", "Vega", "Fei Long", "Balrog", "Deejay", "Cammy"],
            "it should rotate on all rows",
        );
    }

    #[test]
    fn should_rotate_on_all_rows() {
        use Direction::*;

        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_B[..], Position::new(3, 0), &[Down, Right, Right, Right, Down, Left, Left, Down, Right, Right, Right, Up]),
            vec!["Cammy", "Blanka", "Guile", "Chun Li", "Sagat", "Dhalsim", "Zangief", "Cammy", "T.Hawk", "Vega", "Fei Long", "Chun Li"],
            "it should rotate on all rows",
        );
    }

    #[test]
    fn should_work_with_longer_grid() {
        use Direction::*;

        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_C[..], Position::new(3, 0), &[Left, Left, Down, Right, Right, Right, Right, Down, Left, Left, Left, Left, Down, Right, Right, Down, Right, Right, Right, Down, Left, Left, Left, Down, Left, Left, Left]),
            vec!["E.Honda", "Ryu", "Ken", "Chun Li", "Balrog", "Ken", "Chun Li", "Fei Long", "Vega", "Balrog", "Fei Long", "Vega", "Blanka", "Guile", "Chun Li", "Sagat", "M.Bison", "Zangief", "Dhalsim", "Dhalsim", "Zangief", "M.Bison", "Sagat", "T.Hawk", "Cammy", "Deejay", "T.Hawk"],
            "it should work with longer grid",
        );
    }

    #[test]
    fn should_work_with_odd_initial_position() {
        use Direction::*;

        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_C[..], Position::new(3, 3), &[Left, Left, Down, Right, Right, Right, Right, Down, Left, Left, Left, Left, Up, Right, Right, Up, Right, Right, Right]),
            vec!["Guile", "Blanka", "M.Bison", "Zangief", "Dhalsim", "Sagat", "M.Bison", "Deejay", "T.Hawk", "Cammy", "Deejay", "T.Hawk", "Sagat", "M.Bison", "Zangief", "Guile", "Chun Li", "Blanka", "Guile"],
            "it should work with odd initial position",
        );
    }

    #[test]
    fn random() {
        use Direction::*;

        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_C[..], Position::new(1, 3), &[Right, Right, Left, Left, Up, Left, Up, Up, Up, Left, Right, Right, Right, Down, Left, Left, Left, Left, Up, Left, Up, Up, Left, Up, Up, Left, Right, Left, Right, Right, Down, Left, Up, Down]),
            vec!["Chun Li", "Blanka", "Chun Li", "Guile", "Guile", "Blanka", "Vega", "Balrog", "Balrog", "Chun Li", "Balrog", "Ken", "Chun Li", "Fei Long", "Vega", "Balrog", "Fei Long", "Vega", "Balrog", "Chun Li", "E.Honda", "E.Honda", "Ryu", "Ryu", "Ryu", "Cammy", "Ryu", "Cammy", "Ryu", "E.Honda", "Chun Li", "Ken", "Ryu", "Ken"],
            "it should work with odd initial position",
        )
    }
}

