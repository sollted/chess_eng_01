#[cfg(test)]
mod tests {
    use reng::board::{ChessBoard, Move};
    
    #[test]
    fn test_new_board() {
        let board = ChessBoard::new();
        
        assert_eq!(board.w_pawn, 0);
        assert_eq!(board.w_knight, 0);
        assert_eq!(board.w_bishop, 0);
        assert_eq!(board.w_rook, 0);
        assert_eq!(board.w_queen, 0);
        assert_eq!(board.w_king, 0);
        assert_eq!(board.b_pawn, 0);
        assert_eq!(board.b_knight, 0);
        assert_eq!(board.b_bishop, 0);
        assert_eq!(board.b_rook, 0);
        assert_eq!(board.b_queen, 0);
        assert_eq!(board.b_king, 0);
        assert!(board.turn);
        assert_eq!(board.castle_rights, 0);
    }

    #[test]
    fn test_initial_position() {
        let mut board = ChessBoard {
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
        };
        
        let initial_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
        assert!(board.fen_to_board(initial_fen).is_ok());
        
        // Test some expected piece positions
        assert_eq!(board.w_pawn, 0xFF00);                    // Second rank (rank 2): 0000_0000_0000_0000_1111_1111_0000_0000
        assert_eq!(board.w_rook, 0x81);                      // Corners of first rank (rank 1): 0000_0000_0000_0000_0000_0000_1000_0001
        assert_eq!(board.b_pawn, 0x00FF000000000000);        // Seventh rank (rank 7): 0000_0000_1111_1111_0000_0000_0000_0000
        assert_eq!(board.b_rook, 0x8100000000000000);        // Corners of eighth rank (rank 8): 1000_0001_0000_0000_0000_0000_0000_0000
    }

    #[test]
    fn test_invalid_fen() {
        let mut board = ChessBoard {
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
        };
        
        let invalid_fen = "invalid";
        assert!(board.fen_to_board(invalid_fen).is_err());
    }
    #[test]
    fn test_board_to_fen_piece_part() {
        let mut board = ChessBoard::new();
        let initial_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
        assert!(board.fen_to_board(initial_fen).is_ok());
        assert_eq!(board.to_fen().split_whitespace().next().unwrap_or(""), initial_fen.split_whitespace().next().unwrap_or(""));
    }
    #[test]
    fn test_board_to_fen() {
        let mut board = ChessBoard::new();
        let initial_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
        assert!(board.fen_to_board(initial_fen).is_ok());
        assert_eq!(board.to_fen(), initial_fen);
    }
    /*
    #[test]
    fn test_pawn_moves() {
        let mut board = ChessBoard::new();
        let initial_fen ="8/8/8/8/8/1p6/P7/8 w KQkq - 0 1";
        assert!(board.fen_to_board(initial_fen).is_ok());
        for mv in board.pawn_moves(true).iter_mut() {
            let m = mv.convert();
            println!("start: (rank, column): {:?}", m.0);
            println!("end: (rank, column): {:?}", m.1);
            println!("promotion to queen: {:?}", m.2);
        }
    }
    */
    #[test]
    fn test_pawn_basic_moves() {
        let mut board = ChessBoard::new();
        // Test single forward move for white pawn from non-starting position
        let fen = "8/8/8/8/8/P7/8/8 w - - 0 1";
        assert!(board.fen_to_board(fen).is_ok());
        let moves = board.pawn_moves(true);
        assert_eq!(moves.len(), 1); // Only one move possible
        let mv = moves[0].convert();
        assert_eq!(mv.0, (2, 0)); // Start from rank 3, file A
        assert_eq!(mv.1, (3, 0)); // Move to rank 4, file A
        assert!(!mv.2); // No promotion

        // Test single forward move for black pawn from non-starting position
        let fen = "8/8/8/p7/8/8/8/8 b - - 0 1";
        assert!(board.fen_to_board(fen).is_ok());
        let moves = board.pawn_moves(false);
        assert_eq!(moves.len(), 1);
        let mv = moves[0].convert();
        assert_eq!(mv.0, (4, 0)); // Start from rank 5, file A
        assert_eq!(mv.1, (3, 0)); // Move to rank 4, file A
        assert!(!mv.2); // No promotion
    }

    #[test]
    fn test_pawn_initial_double_move() {
        let mut board = ChessBoard::new();
        // Test initial double move for white pawn
        let fen = "8/8/8/8/8/8/P7/8 w - - 0 1";
        assert!(board.fen_to_board(fen).is_ok());
        let moves = board.pawn_moves(true);
        assert_eq!(moves.len(), 2); // Both single and double move possible
        
        // Test initial double move for black pawn
        let fen = "8/p7/8/8/8/8/8/8 b - - 0 1";
        assert!(board.fen_to_board(fen).is_ok());
        let moves = board.pawn_moves(false);
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_pawn_captures() {
        let mut board = ChessBoard::new();
        // Test white pawn captures
        let fen = "8/8/8/8/1p1p4/2P5/8/8 w - - 0 1";
        assert!(board.fen_to_board(fen).is_ok());
        let moves = board.pawn_moves(true);
        assert_eq!(moves.len(), 3); // Forward + 2 captures

        // Test black pawn captures
        let fen = "8/2p5/1P1P4/8/8/8/8/8 w - - 0 1";
        assert!(board.fen_to_board(fen).is_ok());
        let moves = board.pawn_moves(false);
        assert_eq!(moves.len(), 4); // Forward + 2 Forward + 2 captures
    }

    #[test]
    fn test_pawn_promotion() {
        let mut board = ChessBoard::new();
        // Test white pawn promotion
        let fen = "8/P7/8/8/8/8/8/8 w - - 0 1";
        assert!(board.fen_to_board(fen).is_ok());
        let moves = board.pawn_moves(true);
        assert_eq!(moves.len(), 1);
        let mv = moves[0].convert();
        assert!(mv.2); // Should be promotion

        // Test black pawn promotion
        let fen = "8/8/8/8/8/8/p7/8 b - - 0 1";
        assert!(board.fen_to_board(fen).is_ok());
        let moves = board.pawn_moves(false);
        assert_eq!(moves.len(), 1);
        let mv = moves[0].convert();
        assert!(mv.2); // Should be promotion
    }

    #[test]
    fn test_blocked_pawn_moves() {
        let mut board = ChessBoard::new();
        // Test blocked white pawn
        let fen = "8/8/8/8/8/p7/P7/8 w - - 0 1";
        assert!(board.fen_to_board(fen).is_ok());
        let moves = board.pawn_moves(true);
        assert_eq!(moves.len(), 0); // No moves possible for the blocked pawn

        // Test blocked black pawn
        let fen = "8/p7/P7/8/8/8/8/8 b - - 0 1";
        assert!(board.fen_to_board(fen).is_ok());
        let moves = board.pawn_moves(false);
        assert_eq!(moves.len(), 0); // No moves possible for the blocked pawn
    }
}
