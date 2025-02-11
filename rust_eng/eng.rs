#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct ChessBoard {
    w_pawn: u64,
    w_king: u64,
    w_bishop: u64,
    w_rook: u64,
    w_queen: u64,
    w_knight: u64,
    b_pawn: u64,
    b_king: u64,
    b_bishop: u64,
    b_rook: u64,
    b_queen: u64,
    b_knight: u64,
    turn: bool,
    castling: u8, // 1111 = all castling allowed, 0000 = no castling allowed, 1000 = white king side, 0100 = white queen side, 0010 = black king side, 0001 = black queen side
} 

impl ChessBoard {
    // Get all occupied squares (both black and white pieces)
    fn occupied(&self) -> u64 {
        self.w_pawn | self.w_king | self.w_bishop | self.w_rook | 
        self.w_queen | self.w_knight | self.b_pawn | self.b_king | 
        self.b_bishop | self.b_rook | self.b_queen | self.b_knight
    }

    // Get all empty squares
    fn empty(&self) -> u64 {
        !self.occupied()
    }

    // Get all white pieces
    fn white_pieces(&self) -> u64 {
        self.w_pawn | self.w_king | self.w_bishop | 
        self.w_rook | self.w_queen | self.w_knight
    }

    // Get all black pieces
    fn black_pieces(&self) -> u64 {
        self.b_pawn | self.b_king | self.b_bishop | 
        self.b_rook | self.b_queen | self.b_knight
    }

    // Get pieces of current player
    fn current_pieces(&self) -> u64 {
        if self.turn {
            self.white_pieces()
        } else {
            self.black_pieces()
        }
    }

    // Get pieces of opponent
    fn opponent_pieces(&self) -> u64 {
        if self.turn {
            self.black_pieces()
        } else {
            self.white_pieces()
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
struct Move {
    source: u8,      // 0-63 (6 bits)
    destination: u8, // 0-63 (6 bits)
    promotion: bool,   // 0-15 (4 bits)
    move_type: u8,   // 0-15 (4 bits)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MoveType {
    Quiet,
    Capture,
    Promotion,
    Castling,
    EnPassant,
    DoublePawnPush,
    PromotionCapture,
}

impl From<u8> for MoveType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Quiet,
            1 => Self::Capture,
            2 => Self::Promotion,
            3 => Self::Castling,
            4 => Self::EnPassant,
            5 => Self::DoublePawnPush,
            6 => Self::PromotionCapture,
            _ => panic!("Invalid move type: {}", value),
        }
    }
}

fn fen_to_board(fen: &str) -> Result<ChessBoard, &'static str> {
    let mut board = ChessBoard::default();
    let parts: Vec<&str> = fen.split_whitespace().collect();
    
    if parts.len() < 4 {
        return Err("Invalid FEN string: missing required fields");
    }

    // Parse board position
    let board_str = parts[0];
    let mut rank:i8 = 7;
    let mut file:i8 = 0;

    for c in board_str.chars() {
        // Check if we've exceeded the board width
        if file > 8 {
            return Err("Invalid FEN string: too many pieces in rank");
        }

        match c {
            '/' => {
                if file != 8 {
                    return Err("Invalid FEN string: incomplete rank");
                }
                rank = rank.checked_sub(1).ok_or("Invalid FEN string: too many ranks")?;
                file = 0;
            }
            '1'..='8' => {
                let empty_count = c.to_digit(10).unwrap() as i8;
                file += empty_count;
            }
            'P'|'K'|'B'|'R'|'Q'|'p'|'k'|'b'|'r'|'q'|'n'|'N' => {  // Added knights!
                let square = 1u64 << (rank * 8 + file);
                match c {
                    'P' => board.w_pawn |= square,
                    'K' => board.w_king |= square,
                    'B' => board.w_bishop |= square,
                    'R' => board.w_rook |= square,
                    'Q' => board.w_queen |= square,
                    'p' => board.b_pawn |= square,
                    'k' => board.b_king |= square,
                    'b' => board.b_bishop |= square,
                    'r' => board.b_rook |= square,
                    'q' => board.b_queen |= square,
                    'N' => board.w_knight |= square,  // You'll need to add these fields
                    'n' => board.b_knight |= square,  // to your ChessBoard struct
                    _ => unreachable!(),
                }
                file += 1;
            }
            _ => return Err("Invalid character in FEN string"),
        }
    }

    // Verify final rank
    if rank != 0 || file != 8 {
        return Err("Invalid FEN string: incorrect number of ranks or files");
    }

    // Parse turn
    board.turn = match parts[1] {
        "w" => true,
        "b" => false,
        _ => return Err("Invalid turn indicator in FEN string"),
    };

    // Parse castling rights
    let castling_str = parts[2];
    board.castling = if castling_str == "-" {
        0
    } else {
        let mut rights = 0u8;
        for c in castling_str.chars() {
            match c {
                'K' => rights |= 0b1000,
                'Q' => rights |= 0b0100,
                'k' => rights |= 0b0010,
                'q' => rights |= 0b0001,
                _ => return Err("Invalid castling rights in FEN string"),
            }
        }
        rights
    };

    Ok(board)
}

fn board_to_fen(board: &ChessBoard) -> String {
    let mut fen = String::with_capacity(100); // Pre-allocate space
    let mut empty_squares = 0;

    // Helper function to handle piece addition
    let add_piece = |c: char, empty: &mut u8, fen: &mut String| {
        if *empty > 0 {
            fen.push_str(&empty.to_string());
            *empty = 0;
        }
        fen.push(c);
    };

    // Process each rank and file
    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = 1u64 << (rank * 8 + file);
            
            match () {
                _ if board.w_pawn & square != 0 => add_piece('P', &mut empty_squares, &mut fen),
                _ if board.w_king & square != 0 => add_piece('K', &mut empty_squares, &mut fen),
                _ if board.w_bishop & square != 0 => add_piece('B', &mut empty_squares, &mut fen),
                _ if board.w_rook & square != 0 => add_piece('R', &mut empty_squares, &mut fen),
                _ if board.w_queen & square != 0 => add_piece('Q', &mut empty_squares, &mut fen),
                _ if board.w_knight & square != 0 => add_piece('N', &mut empty_squares, &mut fen),
                _ if board.b_pawn & square != 0 => add_piece('p', &mut empty_squares, &mut fen),
                _ if board.b_king & square != 0 => add_piece('k', &mut empty_squares, &mut fen),
                _ if board.b_bishop & square != 0 => add_piece('b', &mut empty_squares, &mut fen),
                _ if board.b_rook & square != 0 => add_piece('r', &mut empty_squares, &mut fen),
                _ if board.b_queen & square != 0 => add_piece('q', &mut empty_squares, &mut fen),
                _ if board.b_knight & square != 0 => add_piece('n', &mut empty_squares, &mut fen),
                _ => empty_squares += 1,
            }
        }
        
        if empty_squares > 0 {
            fen.push_str(&empty_squares.to_string());
            empty_squares = 0;
        }
        
        if rank > 0 {
            fen.push('/');
        }
    }

    // Append the remaining FEN components
    fen.push_str(if board.turn { " w " } else { " b " });
    
    if board.castling == 0 {
        fen.push('-');
    } else {
        for (right, c) in [(0b1000, 'K'), (0b0100, 'Q'), (0b0010, 'k'), (0b0001, 'q')] {
            if board.castling & right != 0 {
                fen.push(c);
            }
        }
    }

    // Add placeholder values for en passant, halfmove clock, and fullmove number
    fen.push_str(" - 0 1");
    
    fen
}

fn pawn_moves(board: &ChessBoard) -> Vec<Move> {
    let mut moves = Vec::new();
    let pawns = if board.turn { board.w_pawn } else { board.b_pawn };
    let empty = board.empty();
    let enemies = board.opponent_pieces();
    
    // Direction and starting rank depend on color
    let (direction, start_rank, promotion_rank) = if board.turn {
        (8, 1, 6)  // White pawns move up (+8)
    } else {
        (-8, 6, 1) // Black pawns move down (-8)
    };

    let mut curr_pawns = pawns;
    while curr_pawns != 0 {
        let source = curr_pawns.trailing_zeros() as u8;
        let rank = source / 8;
        let file = source % 8;
        
        // Clear the least significant bit
        curr_pawns &= curr_pawns - 1;

        // Single push
        let dest = (source as i8 + direction) as u8;
        if dest < 64 && (empty & (1 << dest)) != 0 {
            if rank == promotion_rank {
                // Pawn promotion
                moves.push(Move {
                    source,
                    destination: dest,
                    promotion: true,
                    move_type: MoveType::Promotion as u8,
                });
            } else {
                moves.push(Move {
                    source,
                    destination: dest,
                    promotion: false,
                    move_type: MoveType::Quiet as u8,
                });

                // Double push (only from starting rank)
                if rank == start_rank {
                    let double_dest = (source as i8 + direction * 2) as u8;
                    if (empty & (1 << double_dest)) != 0 {
                        moves.push(Move {
                            source,
                            destination: double_dest,
                            promotion: false,
                            move_type: MoveType::DoublePawnPush as u8,
                        });
                    }
                }
            }
        }

        // Captures (including promotions)
        for capture_offset in [-1, 1] {
            // Skip if pawn is on edge of board
            if (file == 0 && capture_offset == -1) || (file == 7 && capture_offset == 1) {
                continue;
            }

            let dest = (source as i8 + direction + capture_offset) as u8;
            if dest < 64 {
                let dest_square = 1u64 << dest;
                if (enemies & dest_square) != 0 {
                    if rank == promotion_rank {
                        // Promotion capture
                        moves.push(Move {
                            source,
                            destination: dest,
                            promotion: true,
                            move_type: MoveType::PromotionCapture as u8,
                        });
                    } else {
                        moves.push(Move {
                            source,
                            destination: dest,
                            promotion: false,
                            move_type: MoveType::Capture as u8,
                        });
                    }
                }
            }
        }
    }
    moves
}

fn knight_moves(board: &ChessBoard) -> Vec<Move> {
    let mut moves = Vec::new();
    let knight = if board.turn { board.w_knight } else { board.b_knight };
    let enemies = board.opponent_pieces();
    let friends = board.current_pieces();

    let mut knights = knight;
    while knights != 0 {
        let source = knights.trailing_zeros() as u8;
        let file = source % 8;
        let rank = source / 8;

        // All possible knight moves
        let offsets = [
            (-2, -1), (-2, 1), (-1, -2), (-1, 2),
            (1, -2), (1, 2), (2, -1), (2, 1)
        ];

        for (rank_offset, file_offset) in offsets.iter() {
            let new_rank = rank as i8 + rank_offset;
            let new_file = file as i8 + file_offset;

            // Check if move is on board
            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                let dest = (new_rank * 8 + new_file) as u8;
                let dest_square = 1u64 << dest;

                // If square is empty or contains enemy piece
                if (dest_square & friends) == 0 {
                    if (dest_square & enemies) != 0 {
                        moves.push(Move {
                            source,
                            destination: dest,
                            promotion: false,
                            move_type: MoveType::Capture as u8,
                        });
                    } else {
                        moves.push(Move {
                            source,
                            destination: dest,
                            promotion: false,
                            move_type: MoveType::Quiet as u8,
                        });
                    }
                }
            }
        }
        knights &= knights - 1; // Clear least significant bit
    }
    moves
}

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /*
    match fen_to_board(fen) {
        Ok(board) => println!("{:?}", board),
        Err(e) => println!("Error: {}", e),
    }
    */
    match fen_to_board(fen) {
        Ok(board) => {  
            println!("{:?}", board);
            println!("{}", board_to_fen(&board));
            println!("{}", if fen == board_to_fen(&board) { "true" } else { "false" });
        }
        Err(e) => println!("Error: {}", e),
    }
}

