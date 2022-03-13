use std::io;

fn set_board_to_fen(mut board: [i8; 64], fen: [char; 64]) {

	for i in 1..64 {
		match fen[i] {
			'\0' | ' ' => break,
			'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8' => 
			loop_empty_squares(board, fen[i], i),
			'p' => board[i] = -1,
			'r' => board[i] = -2,
			'n' => board[i] = -3,
			'b' => board[i] = -4,
			'q' => board[i] = -5,
			'k' => board[i] = -6,
			'P' => board[i] = 1,
			'R' => board[i] = 2,
			'N' => board[i] = 3,
			'B' => board[i] = 4,
			'Q' => board[i] = 5,
			'K' => board[i] = 6,
			'/' => continue,
			_ => break,
		}
	}
}

fn loop_empty_squares(mut board: [i8; 64], laps: char, pos: usize) {
	for i in 0..laps.to_digit(10).unwrap()-1 {
		board[pos + i as usize] = 0;
	}
}