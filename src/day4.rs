use std::{fs::File, io::Read};

use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{line_ending, space0, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_str(input: &str) -> Result<u8, std::num::ParseIntError> {
    input.parse::<u8>()
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn parse_number(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(1, 2, is_digit), from_str)(input)
}

fn parse_number_list(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, res) = separated_list1(tag(","), parse_number)(input)?;
    Ok((input, res))
}

fn parse_bingo_line(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, _) = space0(input)?;
    let (input, res) = separated_list1(space1, parse_number)(input)?;
    Ok((input, res))
}

#[derive(Debug, PartialEq)]
struct BingoBoard {
    board: Vec<Vec<u8>>,
    marked: [[bool; 5]; 5],
}

impl BingoBoard {
    fn new(board: Vec<Vec<u8>>) -> Self {
        BingoBoard {
            board,
            marked: [[false; 5]; 5],
        }
    }
}

#[derive(Debug, PartialEq)]
struct BingoGame {
    numbers: Vec<u8>,
    boards: Vec<BingoBoard>,
}

fn parse_bingo_board(input: &str) -> IResult<&str, BingoBoard> {
    let (input, res) = separated_list1(line_ending, parse_bingo_line)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, BingoBoard::new(res)))
}

fn parse_file(input: &str) -> IResult<&str, BingoGame> {
    let (input, numbers) = parse_number_list(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;
    let (input, boards) = separated_list1(line_ending, parse_bingo_board)(input)?;

    Ok((input, BingoGame { numbers, boards }))
}

fn check_win(board: &BingoBoard) -> bool {
    for i in 0..5 {
        let mut check_row = true;
        let mut check_col = true;
        for j in 0..5 {
            check_row &= board.marked[i][j];
            check_col &= board.marked[j][i];
        }
        if check_row || check_col {
            return true;
        }
    }
    false
}

fn calculate_win(board: &BingoBoard) -> i32 {
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !board.marked[i][j] {
                sum += board.board[i][j] as i32;
            }
        }
    }
    sum
}

pub fn play_bingo_to_win(file: File) -> i32 {
    let mut file = file;
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let (_, mut g) = parse_file(&buf).unwrap();
    for bingo_num in g.numbers {
        for board in g.boards.iter_mut() {
            for i in 0..5 {
                for j in 0..5 {
                    let res = board.board[i][j] == bingo_num;
                    board.marked[i][j] |= res;
                }
            }
            if check_win(board) {
                return calculate_win(board) * bingo_num as i32;
            }
        }
    }
    0
}

pub fn play_bingo_to_lose(file: File) -> i32 {
    let mut file = file;
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let (_, mut g) = parse_file(&buf).unwrap();
    for bingo_num in g.numbers {
        let mut indices_to_remove = vec![];
        for i in 0..g.boards.len() {
            let mut board = &mut g.boards[i];
            for i in 0..5 {
                for j in 0..5 {
                    let res = board.board[i][j] == bingo_num;
                    board.marked[i][j] |= res;
                }
            }
            if check_win(board) {
                indices_to_remove.push(i);
                if g.boards.len() == 1 {
                    return calculate_win(&g.boards[0]) * bingo_num as i32;
                }
            }
        }
        for &index in indices_to_remove.iter().rev() {
            g.boards.remove(index);
        }
    }
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(is_digit('5'), true);
    }
    #[test]
    fn test2() {
        assert_eq!(is_digit('a'), false);
    }
    #[test]
    fn test3() {
        assert_eq!(from_str("5"), Ok(5));
    }
    #[test]
    fn test4() {
        let action = from_str("a");
        assert!(matches!(action, Err(_)));
    }
    #[test]
    fn test5() {
        let action = parse_number(",");
        assert!(matches!(action, Err(_)));
    }
    #[test]
    fn test6() {
        assert_eq!(parse_number("1,"), Ok((",", 1)));
    }
    #[test]
    fn test7() {
        assert_eq!(parse_number("12,"), Ok((",", 12)));
    }
    #[test]
    fn test8() {
        assert_eq!(parse_number("123,"), Ok(("3,", 12)));
    }
    #[test]
    fn test9() {
        assert_eq!(parse_number_list("12,34,5,6"), Ok(("", vec![12, 34, 5, 6])));
    }
    #[test]
    fn test10() {
        assert_eq!(
            parse_number_list("12,34,5,6 "),
            Ok((" ", vec![12, 34, 5, 6]))
        );
    }
    #[test]
    fn test11() {
        assert_eq!(
            parse_number_list("12,34,5,6  "),
            Ok(("  ", vec![12, 34, 5, 6]))
        );
    }
    #[test]
    fn test12() {
        assert_eq!(
            parse_bingo_line("12 34 5 6 8"),
            Ok(("", vec![12, 34, 5, 6, 8]))
        );
    }
    #[test]
    fn test13() {
        assert_eq!(
            parse_bingo_board(
                "12 34 5 6 8
12 34 5 6 8
12 34 5 6 4
12 34 5 6 8
12 34 5 6 9
"
            ),
            Ok((
                "",
                BingoBoard::new(vec![
                    vec![12, 34, 5, 6, 8],
                    vec![12, 34, 5, 6, 8],
                    vec![12, 34, 5, 6, 4],
                    vec![12, 34, 5, 6, 8],
                    vec![12, 34, 5, 6, 9],
                ])
            ))
        );
    }
    #[test]
    fn test14() {
        assert_eq!(
            parse_file(
                "46,79,77

84 94 24 52 44
96 33 74 35 13
60 51 41 19 95
50 93 27 40  1
67 23 37 88 85

84 94 24 52 44
96 33 74 35 13
60 51 41 19 95
50 93 27 40  1
67 23 37 88 85
"
            ),
            Ok((
                "",
                BingoGame {
                    numbers: vec![46, 79, 77],
                    boards: vec![
                        BingoBoard::new(vec![
                            vec![84, 94, 24, 52, 44],
                            vec![96, 33, 74, 35, 13],
                            vec![60, 51, 41, 19, 95],
                            vec![50, 93, 27, 40, 1],
                            vec![67, 23, 37, 88, 85],
                        ]),
                        BingoBoard::new(vec![
                            vec![84, 94, 24, 52, 44],
                            vec![96, 33, 74, 35, 13],
                            vec![60, 51, 41, 19, 95],
                            vec![50, 93, 27, 40, 1],
                            vec![67, 23, 37, 88, 85],
                        ])
                    ]
                }
            ))
        );
    }
}
