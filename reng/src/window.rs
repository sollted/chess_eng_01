use macroquad::prelude::*;
use crate::board::{ChessBoard, Move};

const SQUARE_SIZE: f32 = 60.0;
const BOARD_SIZE: f32 = SQUARE_SIZE * 8.0;

pub struct GameWindow {
    board: ChessBoard,
    textures: ChessPieceTextures,
    selected_square: Option<(i8, i8)>,
    legal_moves: Vec<Move>,
}

struct ChessPieceTextures {
    white_pawn: Texture2D,
    white_knight: Texture2D,
    white_bishop: Texture2D,
    white_rook: Texture2D,
    white_queen: Texture2D,
    white_king: Texture2D,
    black_pawn: Texture2D,
    black_knight: Texture2D,
    black_bishop: Texture2D,
    black_rook: Texture2D,
    black_queen: Texture2D,
    black_king: Texture2D,
}

impl GameWindow {
    pub async fn new() -> Self {
        let textures = ChessPieceTextures {
            white_pawn: load_texture("pieces-basic-png/white-pawn.png").await.unwrap(),
            white_knight: load_texture("pieces-basic-png/white-knight.png").await.unwrap(),
            white_bishop: load_texture("pieces-basic-png/white-bishop.png").await.unwrap(),
            white_rook: load_texture("pieces-basic-png/white-rook.png").await.unwrap(),
            white_queen: load_texture("pieces-basic-png/white-queen.png").await.unwrap(),
            white_king: load_texture("pieces-basic-png/white-king.png").await.unwrap(),
            black_pawn: load_texture("pieces-basic-png/black-pawn.png").await.unwrap(),
            black_knight: load_texture("pieces-basic-png/black-knight.png").await.unwrap(),
            black_bishop: load_texture("pieces-basic-png/black-bishop.png").await.unwrap(),
            black_rook: load_texture("pieces-basic-png/black-rook.png").await.unwrap(),
            black_queen: load_texture("pieces-basic-png/black-queen.png").await.unwrap(),
            black_king: load_texture("pieces-basic-png/black-king.png").await.unwrap(),
        };

        let mut board = ChessBoard::new();
        board.fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -").unwrap();

        Self {
            board,
            textures,
            selected_square: None,
            legal_moves: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        // If it's black's turn, make a computer move
        if !self.board.turn {  // false means black's turn
            let start_time = std::time::Instant::now();
            let computer_move = self.board.get_computer_move(3);
            let duration = start_time.elapsed();
            println!("Computer move took: {:?}", duration);
            self.board.make_move(&computer_move);
            return;
        }

        // Rest of the existing update logic for white's moves
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let board_x = (screen_width() - BOARD_SIZE) / 2.0;
            let board_y = (screen_height() - BOARD_SIZE) / 2.0;

            // Convert mouse position to board coordinates
            if mouse_pos.0 >= board_x && mouse_pos.0 < board_x + BOARD_SIZE &&
               mouse_pos.1 >= board_y && mouse_pos.1 < board_y + BOARD_SIZE {
                let file = ((mouse_pos.0 - board_x) / SQUARE_SIZE) as i8;
                let rank = 7 - ((mouse_pos.1 - board_y) / SQUARE_SIZE) as i8;

                if let Some(selected) = self.selected_square {
                    // If a square was already selected, try to make a move
                    let start_square = 1u64 << (selected.0 * 8 + selected.1);
                    let target_square = 1u64 << (rank * 8 + file);

                    if let Some(legal_move) = self.legal_moves.iter()
                        .find(|m| m.start == start_square && m.dest == target_square) {
                        // Update the board with the move
                        self.board.make_move(legal_move);
                        self.selected_square = None;
                        self.legal_moves.clear();
                    } else {
                        // If clicked on a different piece of same color, select it instead
                        let clicked_square = 1u64 << (rank * 8 + file);
                        let all_moves = self.board.legal_moves(self.board.turn, false);
                        // Filter moves to only those starting from the clicked square
                        let piece_moves: Vec<Move> = all_moves.into_iter()
                            .filter(|m| m.start == clicked_square)
                            .collect();
                        
                        if !piece_moves.is_empty() {
                            self.selected_square = Some((rank, file));
                            self.legal_moves = piece_moves;
                        } else {
                            self.selected_square = None;
                            self.legal_moves.clear();
                        }
                    }
                } else {
                    // Select the square if it contains a piece
                    let clicked_square = 1u64 << (rank * 8 + file);
                    let all_moves = self.board.legal_moves(self.board.turn, false);
                    // Filter moves to only those starting from the clicked square
                    let piece_moves: Vec<Move> = all_moves.into_iter()
                        .filter(|m| m.start == clicked_square)
                        .collect();
                    
                    if !piece_moves.is_empty() {
                        self.selected_square = Some((rank, file));
                        self.legal_moves = piece_moves;
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        // Draw the chess board
        let board_x = (screen_width() - BOARD_SIZE) / 2.0;
        let board_y = (screen_height() - BOARD_SIZE) / 2.0;

        // Draw squares
        for rank in 0..8 {
            for file in 0..8 {
                let x = board_x + file as f32 * SQUARE_SIZE;
                let y = board_y + (7 - rank) as f32 * SQUARE_SIZE;
                let color = if (rank + file) % 2 == 0 {
                    Color::new(0.9, 0.9, 0.8, 1.0) // Light squares
                } else {
                    Color::new(0.5, 0.5, 0.4, 1.0) // Dark squares
                };

                draw_rectangle(x, y, SQUARE_SIZE, SQUARE_SIZE, color);

                // Highlight selected square
                if let Some(selected) = self.selected_square {
                    if selected == (rank as i8, file as i8) {
                        draw_rectangle(x, y, SQUARE_SIZE, SQUARE_SIZE, Color::new(0.0, 1.0, 0.0, 0.3));
                    }
                }

                // Highlight legal moves
                let square = 1u64 << (rank * 8 + file);
                if self.legal_moves.iter().any(|m| m.dest == square) {
                    draw_rectangle(x, y, SQUARE_SIZE, SQUARE_SIZE, Color::new(0.0, 0.0, 1.0, 0.3));
                }
            }
        }

        // Draw pieces
        self.draw_pieces(board_x, board_y);
    }

    fn draw_pieces(&self, board_x: f32, board_y: f32) {
        // Helper function to draw a piece
        let draw_piece = |texture: &Texture2D, rank: u32, file: u32| {
            let x = board_x + file as f32 * SQUARE_SIZE;
            let y = board_y + (7 - rank) as f32 * SQUARE_SIZE;
            // Scale the texture to fit the square
            let scale = SQUARE_SIZE / texture.width();
            let params = DrawTextureParams {
                dest_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ..Default::default()
            };
            draw_texture_ex(texture, x, y, WHITE, params);
        };

        // Draw all pieces based on the bitboards
        for rank in 0..8 {
            for file in 0..8 {
                let square = 1u64 << (rank * 8 + file);
                
                if self.board.w_pawn & square != 0 {
                    draw_piece(&self.textures.white_pawn, rank, file);
                } else if self.board.w_knight & square != 0 {
                    draw_piece(&self.textures.white_knight, rank, file);
                } else if self.board.w_bishop & square != 0 {
                    draw_piece(&self.textures.white_bishop, rank, file);
                } else if self.board.w_rook & square != 0 {
                    draw_piece(&self.textures.white_rook, rank, file);
                } else if self.board.w_queen & square != 0 {
                    draw_piece(&self.textures.white_queen, rank, file);
                } else if self.board.w_king & square != 0 {
                    draw_piece(&self.textures.white_king, rank, file);
                } else if self.board.b_pawn & square != 0 {
                    draw_piece(&self.textures.black_pawn, rank, file);
                } else if self.board.b_knight & square != 0 {
                    draw_piece(&self.textures.black_knight, rank, file);
                } else if self.board.b_bishop & square != 0 {
                    draw_piece(&self.textures.black_bishop, rank, file);
                } else if self.board.b_rook & square != 0 {
                    draw_piece(&self.textures.black_rook, rank, file);
                } else if self.board.b_queen & square != 0 {
                    draw_piece(&self.textures.black_queen, rank, file);
                } else if self.board.b_king & square != 0 {
                    draw_piece(&self.textures.black_king, rank, file);
                }
            }
        }
    }
}
