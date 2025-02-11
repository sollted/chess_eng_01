#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    w_pawn,
    w_knight,
    w_bishop,
    w_rook,
    w_queen,
    w_king,
    b_pawn,
    b_knight,
    b_bishop,
    b_rook,
    b_queen,
    b_king,
}

#[derive(Debug, Clone)]
pub struct ChessBoard {
    pub w_pawn: u64,
    pub w_knight: u64,
    pub w_bishop: u64,
    pub w_rook: u64,
    pub w_queen: u64,
    pub w_king: u64,
    pub b_pawn: u64,
    pub b_knight: u64,
    pub b_bishop: u64,
    pub b_rook: u64,
    pub b_queen: u64,
    pub b_king: u64,
    pub turn: bool,
    pub castle_rights: u8 // 1,2,3,4 1-2 white, 3-4 black 1,3 king side 2,4 queenside
}

impl ChessBoard {
    pub fn new() -> Self {
        //{{{
        ChessBoard {
            w_pawn: 0,
            w_knight: 0,
            w_bishop: 0,
            w_rook: 0,
            w_queen: 0,
            w_king: 0,
            b_pawn: 0,
            b_knight: 0,
            b_bishop: 0,
            b_rook: 0,
            b_queen: 0,
            b_king: 0,
            turn: true,
            castle_rights: 0,
        }
        //}}}
    }
    pub fn get_piece(&self, piece: Piece) -> Option<u64> {
        //{{{
        match piece {
            Piece::w_pawn => Some(self.w_pawn),
            Piece::w_knight => Some(self.w_knight),
            Piece::w_bishop => Some(self.w_bishop),
            Piece::w_rook => Some(self.w_rook),
            Piece::w_queen => Some(self.w_queen),
            Piece::w_king => Some(self.w_king),
            Piece::b_pawn => Some(self.b_pawn),
            Piece::b_knight => Some(self.b_knight),
            Piece::b_bishop => Some(self.b_bishop),
            Piece::b_rook => Some(self.b_rook),
            Piece::b_queen => Some(self.b_queen),
            Piece::b_king => Some(self.b_king),
        }
        //}}}
    }
    pub fn set_piece(&mut self, piece: Piece, bitboard: u64) {
        //{{{
        match piece {
            Piece::w_pawn => self.w_pawn = bitboard,
            Piece::w_knight => self.w_knight = bitboard,
            Piece::w_bishop => self.w_bishop = bitboard,
            Piece::w_rook => self.w_rook = bitboard,
            Piece::w_queen => self.w_queen = bitboard,
            Piece::w_king => self.w_king = bitboard,
            Piece::b_pawn => self.b_pawn = bitboard,
            Piece::b_knight => self.b_knight = bitboard,
            Piece::b_bishop => self.b_bishop = bitboard,
            Piece::b_rook => self.b_rook = bitboard,
            Piece::b_queen => self.b_queen = bitboard,
            Piece::b_king => self.b_king = bitboard,
        }
        //}}}
    }
    pub fn in_bounds(x: i32, y: i32) -> bool {
        // {{{
        if x < 8 && x >= 0 && y < 8 && y >= 0 {
            true
        } else {
            false 
        }
        // }}}
    }
    pub fn fen_to_board(&mut self, fen: &str) -> Result<(), String> {
        //{{{
        *self = Self::new();
        
        let mut rank = 7;  // 0-based indexing
        let mut file = 0;

        let parts: Vec<&str> = fen.split_whitespace().collect();
        //check if all necessary fields are in fen
        if parts.len() < 4 {
            return Err("Fen doesn't have all the necessary fields".to_string());
        }

        for c in parts[0].chars() {
            match c {
                '/' => {
                    if file != 8 {
                        return Err(format!("too many or not enough pieces in rank {}", rank));
                    }
                    rank -= 1;
                    file = 0;
                }
                '1'..='8' => {
                    file += c.to_digit(10).unwrap() as usize;
                }
                'p' | 'n' | 'b' | 'r' | 'q' | 'k' | 'P' | 'N' | 'B' | 'R' | 'Q' | 'K' => {
                    //get u64 with only least significant bit set to 1 and shift it (rank * 8 + file) times
                    let square = 1u64 << (rank * 8 + file);
                    match c {
                        'p' => self.b_pawn |= square,
                        'n' => self.b_knight |= square,
                        'b' => self.b_bishop |= square,
                        'r' => self.b_rook |= square,
                        'q' => self.b_queen |= square,
                        'k' => self.b_king |= square,
                        'P' => self.w_pawn |= square,
                        'N' => self.w_knight |= square,
                        'B' => self.w_bishop |= square,
                        'R' => self.w_rook |= square,
                        'Q' => self.w_queen |= square,
                        'K' => self.w_king |= square,
                        _ => unreachable!(),
                    }
                    file += 1;
                }
                _ => return Err(format!("Invalid char in fen: {}", c)),
            }
        }
        //handle turns
        self.turn = match parts[1] {
            "w" => true,
            "b" => false,
            _ => return Err("Invalid turn indicator".to_string()),
        };
        //castling rights
        self.castle_rights = 0;
        for c in parts[2].chars() {
            match c {
                'k' => self.castle_rights |= 0b1000,
                'q' => self.castle_rights |= 0b0100,
                'K' => self.castle_rights |= 0b0010,
                'Q' => self.castle_rights |= 0b0001,
                '-' => (),
                _ => return Err("Invalid castling rights".to_string()),
            }
        }
        Ok(())
        //}}}
    }
    pub fn to_fen(&self) -> String {
        //{{{
        let pieces = [
            (self.w_pawn, 'P'), (self.w_king, 'K'), (self.w_bishop, 'B'),
            (self.w_rook, 'R'), (self.w_queen, 'Q'), (self.w_knight, 'N'),
            (self.b_pawn, 'p'), (self.b_king, 'k'), (self.b_bishop, 'b'),
            (self.b_rook, 'r'), (self.b_queen, 'q'), (self.b_knight, 'n'),
        ];


        let mut fen = String::with_capacity(100);
        let mut empty_squares;  // Declare outside the loop is fine
        
        for rank in (0..8).rev() {
            empty_squares = 0;  // Just reset the value each iteration
            for file in 0..8 {
                let square = 1u64 << (rank * 8 + file);
                if let Some((_, piece)) = pieces.iter().find(|(bitboard, _)| bitboard & square != 0) {
                    fen.push(*piece);
                } else {
                    empty_squares += 1;
                }
            }
            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }
        fen.push(' ');
        fen.push(if self.turn { 'w' } else { 'b' });
        fen.push(' ');

        if self.castle_rights == 0 {
            fen.push('-');
        } else {
            for (right, c) in [(0b1000, 'K'), (0b0100, 'Q'), (0b0010, 'k'), (0b0001, 'q')] {
                if self.castle_rights & right != 0 {
                    fen.push(c);
                }
            }
        }

        fen.push(' ');
        fen.push('-');
        fen
        //}}}
    }                                                           
    pub fn empty(&self) -> u64 {
        //{{{
        //get empty squares
        !(self.w_pawn | self.w_knight | self.w_bishop | self.w_rook | self.w_queen | self.w_king | self.b_pawn | self.b_knight | self.b_bishop | self.b_rook | self.b_queen | self.b_king)
        //}}}
    }
    pub fn open_for_attack(&self, for_white:bool) -> u64 {
        //{{{
        // returns bitboard with 1s (where player can't go) and 0 (where player can go)
        if for_white {
            self.b_pawn | self.b_knight | self.b_bishop | self.b_rook | self.b_queen | self.b_king
        } else {
            self.w_pawn | self.w_knight | self.w_bishop | self.w_rook | self.w_queen | self.w_king
        }
        //}}}
    }
    pub fn pawn_moves(&self, for_white:bool, empty: u64, e_pieces: u64) -> Vec<Move> {
        //{{{
        let mut moves = Vec::new();
        let (mut pawns, dir, start_row, piece_name) = match for_white {
            true => (self.w_pawn, 8, 1, Piece::w_pawn),
            false => (self.b_pawn, -8, 6, Piece::b_pawn),
        };

        while pawns != 0 {
            let index = pawns.trailing_zeros() as i32;
            let row = index / 8;
            let col = index % 8;

            //one forward
            let one_square = index + dir;
            if (0..64).contains(&one_square) {
                let target_pos_1 = 1u64 << one_square;
                if target_pos_1 & empty != 0 {
                    moves.push(Move {
                        piece:piece_name,
                        start:1u64 << index,
                        dest:target_pos_1,
                        promotion: (row + (dir / 8)) == 7 || (row + (dir / 8)) == 0,
                    });
                    //two forward
                    let two_square = index + dir*2;
                    if row == start_row {
                        let target_pos_2 = 1u64 << two_square;
                        if target_pos_2 & empty != 0 {
                            moves.push(Move {
                                piece:piece_name,
                                start:1u64 << index,
                                dest:target_pos_2,
                                promotion:false,
                            });
                        }
                    }
                }

                let (left, right) = match (for_white, col) {
                    (true, 0) => (None, Some(index + dir + dir / 8)),
                    (true, 7) => (Some(index + dir - dir / 8), None),
                    (true, _) => (Some(index + dir - dir / 8), Some(index + dir + dir / 8)),

                    (false, 0) => (None, Some(index + dir - dir / 8)),
                    (false, 7) => (Some(index + dir + dir / 8), None),
                    (false, _) => (Some(index + dir + dir / 8), Some(index + dir - dir / 8)),
                };

                for opt in [&left, &right] {
                    if let Some(x) = opt {
                        if (0..64).contains(x) {
                            let dest = 1u64 << x;
                            if dest & e_pieces != 0 {
                                moves.push(Move {
                                    piece:piece_name,
                                    start:1u64 << index,
                                    dest:dest,
                                    promotion:(row + (dir / 8)) == 7 || (row + (dir / 8)) == 0,
                                });
                            }
                        }
                    }
                }
            }
            pawns &= pawns - 1;
        }
        moves
        //}}}
    }
    pub fn knight_moves(&self, for_white:bool, empty: u64, e_pieces: u64) -> Vec<Move> {
        //{{{
        let mut moves = Vec::new();
        let dirs: [(i32, i32); 8] = [(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2)];
        let (mut knights, piece_name) = match for_white {
            true => (self.w_knight, Piece::w_knight),
            false => (self.b_knight, Piece::b_knight),
        };


        while knights != 0  {
            let index = knights.trailing_zeros() as i32;

            for (x, y) in dirs.iter() {
                let dest = index + y * 8 + x;
                if (0..64).contains(&dest){
                    if Self::in_bounds(x + index % 8, y + index / 8) && (1u64 << dest) & (empty | e_pieces) != 0 {
                        moves.push(Move{
                            piece:piece_name,
                            start:1u64 << index,
                            dest:1u64 << dest,
                            promotion:false,
                        });
                    }
                }
            }
            knights &= knights - 1;
        }
        moves
        //}}}
    }
    pub fn bishop_moves(&self, for_white:bool, queen:bool, empty: u64, e_pieces: u64) -> Vec<Move> {
        //{{{
        let mut moves = Vec::new();
        let (mut bishops, piece_name);

        if queen {
            (bishops, piece_name) = match for_white {
                true => (self.w_queen, Piece::w_queen),
                false => (self.b_queen, Piece::b_queen),
            };
        } else {
            (bishops, piece_name) = match for_white {
                true => (self.w_bishop, Piece::w_bishop),
                false => (self.b_bishop, Piece::b_bishop),
            };
        }

        while bishops != 0 {
            let dirs = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
            let index = bishops.trailing_zeros() as i32;
            let row = index / 8;
            let col = index % 8;

            for (x, y) in dirs.iter() {
                for distance in 1..8 {
                    let new_row = row + y * distance;
                    let new_col = col + x * distance;
                    if !Self::in_bounds(new_col, new_row) {
                        break;
                    }
                    let dest = (new_row * 8) + new_col;
                    if  1u64 << dest & (empty | e_pieces) != 0 {
                        moves.push(Move {
                            piece:piece_name,
                            start:1u64 << index,
                            dest:1u64 << dest,
                            promotion:false,
                        });
                        //check if move is capture
                        if 1u64 << dest & e_pieces != 0 {
                            break;
                        }
                    } else {
                        //remove if direction blocked (cant jump over pieces)
                        break;
                    }
                }
            }
            bishops &= bishops - 1;
        }
        moves
        //}}}
    }
    pub fn rook_moves(&self, for_white:bool, queen:bool, empty: u64, e_pieces: u64) -> Vec<Move> {
        //{{{
        let mut moves = Vec::new();
        let (mut rooks, piece_name);

        if queen {
            (rooks, piece_name) = match for_white {
                true => (self.w_queen, Piece::w_queen),
                false => (self.b_queen, Piece::b_queen),
            };
        } else {
            (rooks, piece_name) = match for_white {
                true => (self.w_rook, Piece::w_rook),
                false => (self.b_rook, Piece::b_rook),
            };
        }

        while rooks != 0 {
            let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            let index = rooks.trailing_zeros() as i32;
            let row = index / 8;
            let col = index % 8;

            for (x, y) in dirs.iter() {
                for distance in 1..8 {
                    let new_row = row + y * distance;
                    let new_col = col + x * distance;
                    if !Self::in_bounds(new_col, new_row) {
                        break;
                    }
                    let dest = (new_row * 8) + new_col;
                    if  1u64 << dest & (empty | e_pieces) != 0 {
                        moves.push(Move {
                            piece:piece_name,
                            start:1u64 << index,
                            dest:1u64 << dest,
                            promotion:false,
                        });
                        //check if move is capture
                        if 1u64 << dest & e_pieces != 0 {
                            break;
                        }
                    } else {
                        //remove if direction blocked (cant jump over pieces)
                        break;
                    }
                }
            }
            rooks &= rooks - 1;
        }
        moves
        //}}}
    }
    pub fn queen_moves(&self, for_white:bool, empty: u64, e_pieces: u64) -> Vec<Move> {
        //{{{
        let mut moves = Vec::new();
        moves.extend(self.bishop_moves(for_white, true, empty, e_pieces));
        moves.extend(self.rook_moves(for_white, true, empty, e_pieces));
        moves
        //}}}
    }
    pub fn king_moves(&self, for_white:bool, empty: u64, e_pieces: u64) -> Vec<Move> {
        //{{{
        let mut moves = Vec::new();
        let (mut kings, piece_name) = match for_white {
            true => (self.w_king, Piece::w_king),
            false => (self.b_king, Piece::b_king),
        };

        while kings != 0 {
            let dirs = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
            let index = kings.trailing_zeros() as i32;
            let row = index / 8;
            let col = index % 8;

            // Normal king moves
            for (x, y) in dirs.iter() {
                let new_row = row + y;
                let new_col = col + x;
                if Self::in_bounds(new_col, new_row) {
                    let dest = (new_row * 8) + new_col;
                    if 1u64 << dest & (empty | e_pieces) != 0 {
                        moves.push(Move {
                            piece: piece_name,
                            start: 1u64 << index,
                            dest: 1u64 << dest,
                            promotion: false,
                        });
                    }
                }
            }

            // Castling moves
            // First check if king is in check
            if !self.is_king_under_attack(for_white) {
                let castle_data = if for_white {
                    vec![
                        // (castle right, king pos, rook pos, empty squares to check, squares to check for attacks, dest pos)
                        (0b0010, 4, 7, vec![5, 6], vec![4, 5, 6], 6),    // kingside
                        (0b0001, 4, 0, vec![1, 2, 3], vec![4, 2, 3], 2), // queenside
                    ]
                } else {
                    vec![
                        (0b1000, 60, 63, vec![61, 62], vec![60, 61, 62], 62),    // kingside
                        (0b0100, 60, 56, vec![57, 58, 59], vec![60, 58, 59], 58), // queenside
                    ]
                };

                for (right, king_pos, rook_pos, empty_squares, check_squares, dest_pos) in castle_data {
                    if self.castle_rights & right != 0 
                        && kings & (1u64 << king_pos) != 0
                        && (if for_white { self.w_rook } else { self.b_rook }) & (1u64 << rook_pos) != 0
                        && empty_squares.iter().all(|&sq| empty & (1u64 << sq) != 0)
                        // Check that none of the squares the king moves through are under attack
                        && check_squares.iter().all(|&sq| !self.is_square_under_attack(1u64 << sq, !for_white)) {
                        moves.push(Move {
                            piece: piece_name,
                            start: 1u64 << king_pos,
                            dest: 1u64 << dest_pos,
                            promotion: false,
                        });
                    }
                }
            }
            kings &= kings - 1;
        }
        moves
        //}}}
    }
    pub fn is_square_under_attack(&self, square: u64, by_white: bool) -> bool {
        //{{{
        let pos = square.trailing_zeros() as i32;
        let row = pos / 8;
        let col = pos % 8;
        
        // Get attacking pieces
        let (pawns, knights, bishops, rooks, queen, king) = if by_white {
            (self.w_pawn, self.w_knight, self.w_bishop, self.w_rook, self.w_queen, self.w_king)
        } else {
            (self.b_pawn, self.b_knight, self.b_bishop, self.b_rook, self.b_queen, self.b_king)
        };

        // Check pawn attacks - using same logic as pawn_moves
        let (dir, start_row) = if by_white {
            (8, 1)  // White moves up
        } else {
            (-8, 6) // Black moves down
        };

        // Check diagonal captures only
        let (left, right) = match (by_white, col) {
            (true, 0) => (None, Some(pos - dir + dir / 8)),
            (true, 7) => (Some(pos - dir - dir / 8), None),
            (true, _) => (Some(pos - dir - dir / 8), Some(pos - dir + dir / 8)),

            (false, 0) => (None, Some(pos - dir - dir / 8)),
            (false, 7) => (Some(pos - dir + dir / 8), None),
            (false, _) => (Some(pos - dir + dir / 8), Some(pos - dir - dir / 8)),
        };

        for opt in [&left, &right] {
            if let Some(x) = opt {
                if (0..64).contains(x) {
                    let attack_square = 1u64 << x;
                    if attack_square & pawns != 0 {
                        return true;
                    }
                }
            }
        }

        // Check knight attacks
        let knight_dirs = [(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2)];
        for (dy, dx) in knight_dirs.iter() {
            let new_row = row + dy;
            let new_col = col + dx;
            if Self::in_bounds(new_col, new_row) {
                let square = 1u64 << (new_row * 8 + new_col);
                if square & knights != 0 {
                    return true;
                }
            }
        }

        // Check sliding piece attacks (queen, rook, bishop)
        let dirs = [
            (0, 1), (0, -1), (1, 0), (-1, 0),  // Rook/Queen directions
            (1, 1), (1, -1), (-1, 1), (-1, -1)  // Bishop/Queen directions
        ];

        for (dy, dx) in dirs.iter() {
            let mut dist = 1;
            loop {
                let new_row = row + dy * dist;
                let new_col = col + dx * dist;
                if !Self::in_bounds(new_col, new_row) {
                    break;
                }
                let check_square = 1u64 << (new_row * 8 + new_col);
                let is_diagonal = dx.abs() == dy.abs();
                
                // If we hit any piece, check if it's an attacking piece and stop checking this direction
                if check_square & (queen | rooks | bishops | king) != 0 {
                    if check_square & queen != 0 || // Queen can attack from any direction
                       (!is_diagonal && check_square & rooks != 0) || // Rook can attack from orthogonal
                       (is_diagonal && check_square & bishops != 0) || // Bishop can attack from diagonal
                       (dist == 1 && check_square & king != 0) { // King can attack adjacent squares
                        return true;
                    }
                    break;
                } else if check_square & (self.w_pawn | self.w_knight | self.w_bishop | self.w_rook | 
                                  self.w_queen | self.w_king | self.b_pawn | self.b_knight | 
                                  self.b_bishop | self.b_rook | self.b_queen | self.b_king) != 0 {
                    break; // Hit a different piece, stop checking this direction
                }
                dist += 1;
            }
        }

        false
        //}}}
    }
    pub fn is_king_under_attack(&self, for_white: bool) -> bool {
        //{{{
        let king = if for_white { self.w_king } else { self.b_king };
        self.is_square_under_attack(king, !for_white)
        //}}}
    }
    pub fn legal_moves(&self, for_white:bool, pseudo_legal:bool) -> Vec<Move> {
        //{{{
        let mut moves = Vec::new();
        let empty = self.empty();
        let e_pieces = self.open_for_attack(for_white);

        //add all piece moves
        moves.extend(self.pawn_moves(for_white, empty, e_pieces));
        moves.extend(self.knight_moves(for_white, empty, e_pieces));
        moves.extend(self.bishop_moves(for_white, false, empty, e_pieces));
        moves.extend(self.rook_moves(for_white, false, empty, e_pieces));
        moves.extend(self.queen_moves(for_white, empty, e_pieces));
        moves.extend(self.king_moves(for_white, empty, e_pieces));

        if pseudo_legal {
            return moves;
        }

        // Filter out moves that leave the king in check
        moves.into_iter()
            .filter(|mv| {
                let mut test_board = self.clone();
                test_board.make_move(mv);
                !test_board.is_king_under_attack(for_white)
            })
            .collect()
        //}}}
    }
    pub fn make_move(&mut self, mv: &Move) {
        //{{{
        // Handle castling moves
        if mv.piece == Piece::w_king || mv.piece == Piece::b_king {
            let castle_data = [
                // (is_white, start, dest, rook_start, rook_dest)
                (true, 4, 6, 7, 5),   // White kingside
                (true, 4, 2, 0, 3),   // White queenside
                (false, 60, 62, 63, 61), // Black kingside
                (false, 60, 58, 56, 59), // Black queenside
            ];

            for (is_white, king_start, king_dest, rook_start, rook_dest) in castle_data {
                if (is_white == (mv.piece == Piece::w_king)) 
                    && mv.start == 1u64 << king_start 
                    && mv.dest == 1u64 << king_dest {
                    let rook = if is_white { &mut self.w_rook } else { &mut self.b_rook };
                    *rook &= !(1u64 << rook_start);
                    *rook |= 1u64 << rook_dest;
                    break;
                }
            }

            // Remove castling rights when king moves
            self.castle_rights &= if mv.piece == Piece::w_king { 0b1100 } else { 0b0011 };
        }

        // Remove castling rights when rooks move or are captured
        let rook_positions = [
            // (piece, pos, rights_mask)
            (Piece::w_rook, 0, 0b0001),  // White queenside
            (Piece::w_rook, 7, 0b0010),  // White kingside
            (Piece::b_rook, 56, 0b0100), // Black queenside
            (Piece::b_rook, 63, 0b1000), // Black kingside
        ];

        // Check for rook moves
        if mv.piece == Piece::w_rook || mv.piece == Piece::b_rook {
            for &(piece, pos, mask) in &rook_positions {
                if mv.piece == piece && mv.start == 1u64 << pos {
                    self.castle_rights &= !mask;
                }
            }
        }

        // Check for rook captures
        for &(_, pos, mask) in &rook_positions {
            if mv.dest & (1u64 << pos) != 0 {
                self.castle_rights &= !mask;
            }
        }

        // Remove the piece from its starting position
        if let Some(mut piece_bb) = self.get_piece(mv.piece) {
            piece_bb &= !mv.start;
            self.set_piece(mv.piece, piece_bb);
        }

        // Handle captures: Remove opponent's piece if present at `mv.dest`
        for enemy_piece in [
            Piece::w_pawn, Piece::w_knight, Piece::w_bishop, Piece::w_rook, Piece::w_queen, Piece::w_king,
            Piece::b_pawn, Piece::b_knight, Piece::b_bishop, Piece::b_rook, Piece::b_queen, Piece::b_king,
        ] {
            if let Some(mut enemy_bb) = self.get_piece(enemy_piece) {
                if enemy_bb & mv.dest != 0 {
                    enemy_bb &= !mv.dest;
                    self.set_piece(enemy_piece, enemy_bb);
                }
            }
        }

        // Handle promotion
        let new_piece = if mv.promotion {
            match mv.piece {
                Piece::w_pawn => Piece::w_queen, 
                Piece::b_pawn => Piece::b_queen, 
                _ => mv.piece,                   
            }
        } else {
            mv.piece
        };

        // Place piece at destination
        if let Some(mut updated_bb) = self.get_piece(new_piece) {
            updated_bb |= mv.dest;
            self.set_piece(new_piece, updated_bb);
        }

        // Switch turns
        self.turn = !self.turn;
        //}}}
    }
    pub fn evaluate_board(&mut self, for_white:bool) -> i32 {
        //{{{
        let mut score = 0;

        // Count material for both sides
        let white_score = (self.w_pawn.count_ones() as i32) * 1
            + (self.w_knight.count_ones() as i32) * 3
            + (self.w_bishop.count_ones() as i32) * 3
            + (self.w_rook.count_ones() as i32) * 5
            + (self.w_queen.count_ones() as i32) * 9
            + (self.w_king.count_ones() as i32) * 100;

        let black_score = (self.b_pawn.count_ones() as i32) * 1
            + (self.b_knight.count_ones() as i32) * 3
            + (self.b_bishop.count_ones() as i32) * 3
            + (self.b_rook.count_ones() as i32) * 5
            + (self.b_queen.count_ones() as i32) * 9
            + (self.b_king.count_ones() as i32) * 100;

        // Calculate score as material difference
        score = if for_white {
            white_score - black_score
        } else {
            black_score - white_score
        };

        score
        //}}}
    }
    pub fn get_computer_move(&mut self, depth: i32) -> Move {
        //{{{
        let moves = self.legal_moves(self.turn, false);
        let mut best_score = i32::MIN;  // Always looking for maximum from current player's perspective
        let mut best_move = moves[0].clone();

        for mv in moves {
            let mut board_copy = self.clone();
            board_copy.make_move(&mv);
            let score = -board_copy.minimax(depth - 1);  // Negamax style
            
            if score > best_score {
                best_score = score;
                best_move = mv;
            }
        }

        best_move
        //}}}
    }

    fn minimax(&mut self, depth: i32) -> i32 {
        if depth == 0 {
            return self.evaluate_board(self.turn);
        }

        let moves = self.legal_moves(self.turn, false);
        
        // If no legal moves, it's either checkmate or stalemate
        if moves.is_empty() {
            if self.is_king_under_attack(self.turn) {
                return -10000; // Checkmate (very bad for current player)
            }
            return 0; // Stalemate
        }

        let mut best_score = i32::MIN;

        for mv in moves {
            let mut board_copy = self.clone();
            board_copy.make_move(&mv);
            let score = -board_copy.minimax(depth - 1);  // Negamax style
            best_score = best_score.max(score);
        }

        best_score
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    pub piece: Piece,
    pub start: u64,
    pub dest: u64,
    pub promotion: bool,
}

impl Move {
    pub fn convert(&self) -> ((i8, i8), (i8, i8), bool) {
        //{{{
        (
            (
                (self.start.trailing_zeros() / 8) as i8,
                (self.start.trailing_zeros() % 8) as i8
            ),
            (
                (self.dest.trailing_zeros() / 8) as i8,
                (self.dest.trailing_zeros() % 8) as i8
            ),
            self.promotion
        )
        //}}}
    }
}
