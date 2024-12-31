import pygame
import os
from chess_eng import Board
from rights import Move

class ChessGUI:
    def __init__(self):
        pygame.init()
        self.SQUARE_SIZE = 80
        self.BOARD_SIZE = self.SQUARE_SIZE * 8
        self.screen = pygame.display.set_mode((self.BOARD_SIZE, self.BOARD_SIZE))
        pygame.display.set_caption("Chess")
        
        # Initialize chess board
        self.board = Board()
        self.board.from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        
        # Load pieces
        self.pieces = {}
        self.load_pieces()
        
        # Add selected square tracking and legal moves
        self.selected_square = None
        self.legal_moves = []

        self.piece_map = {
            1: 'white-pawn',
            2: 'white-knight',
            3: 'white-bishop',
            4: 'white-rook',
            5: 'white-queen',
            6: 'white-king',
            -1: 'black-pawn',
            -2: 'black-knight',
            -3: 'black-bishop',
            -4: 'black-rook',
            -5: 'black-queen',
            -6: 'black-king'
        }
        
    def load_pieces(self):
        piece_names = ['pawn', 'knight', 'bishop', 'rook', 'queen', 'king']
        colors = ['white', 'black']
        
        for color in colors:
            for piece in piece_names:
                img_path = f'pieces/{color}-{piece}.png'
                if os.path.exists(img_path):
                    img = pygame.image.load(img_path)
                    img = pygame.transform.scale(img, (self.SQUARE_SIZE, self.SQUARE_SIZE))
                    self.pieces[f'{color}-{piece}'] = img
    
    def get_piece_image(self, piece_value):
        return self.pieces.get(self.piece_map.get(piece_value))
    
    def handle_click(self, pos):
        # Convert screen coordinates to board coordinates
        col = pos[0] // self.SQUARE_SIZE
        row = pos[1] // self.SQUARE_SIZE
        board_row = 7 - row  # Convert screen row to board row
        board_col = col
        
        # Only handle clicks for white's turn
        if self.board.turn == -1:
            return
        
        # If no square is currently selected
        if self.selected_square is None:
            piece = self.board.board[board_row][board_col]
            if piece > 0:  # Only allow selecting white pieces
                self.selected_square = (board_row, board_col)
                self.legal_moves = [
                    move for move in self.board.generate_legal_moves(is_pseudo=False)
                    if move.start == (board_row, board_col)
                ]
        else:
            # Check if the clicked square is a valid move
            move_made = False
            for move in self.legal_moves:
                if move.end == (board_row, board_col):
                    # Make the move
                    captured_piece, rights = self.board.make_move(move)
                    self.board.turn *= -1
                    move_made = True
                    
                    # After white's move, make black's move automatically
                    black_move = self.board.find_best_move()
                    if black_move:
                        captured_piece, rights = self.board.make_move(black_move)
                        self.board.turn *= -1
                    break
            
            # Clear selection
            self.selected_square = None
            self.legal_moves = []

    def draw_board(self):
        for row in range(8):
            for col in range(8):
                board_row = 7 - row  # Restore this inversion
                board_col = col
                
                # Determine square color - base it on visual position (row, col)
                if self.selected_square and self.selected_square == (board_row, board_col):
                    color = (255, 255, 0)  # Yellow for selected square
                elif any(move.end == (board_row, board_col) for move in self.legal_moves):
                    color = (144, 238, 144)  # Light green for legal moves
                else:
                    # Base checkerboard pattern on screen coordinates
                    color = (240, 217, 181) if (row + col) % 2 == 0 else (181, 136, 99)
                
                pygame.draw.rect(self.screen, color, 
                               (col * self.SQUARE_SIZE, row * self.SQUARE_SIZE, 
                                self.SQUARE_SIZE, self.SQUARE_SIZE))
                
                # Draw coordinate numbers
                font = pygame.font.SysFont('Arial', 12)
                text = font.render(f'({board_row},{board_col})', True, (128, 128, 128))
                text_rect = text.get_rect()
                text_rect.topleft = (col * self.SQUARE_SIZE + 5, row * self.SQUARE_SIZE + 5)
                self.screen.blit(text, text_rect)
                
                # Draw pieces - no more conditional inversion of piece values
                piece_value = self.board.board[board_row][board_col]
                if piece_value != 0:
                    piece_img = self.get_piece_image(piece_value)
                    if piece_img:
                        self.screen.blit(piece_img, 
                                    (col * self.SQUARE_SIZE, row * self.SQUARE_SIZE))

    def run(self):
        running = True
        clock = pygame.time.Clock()
        
        # Check for game over
        def check_game_over():
            is_over, result = self.board.game_over()
            if is_over:
                print(f"Game Over! Result: {result}")
                return True
            return False
        
        while running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    running = False
                elif event.type == pygame.MOUSEBUTTONDOWN:
                    if not check_game_over():  # Only allow moves if game isn't over
                        self.handle_click(event.pos)
            
            self.screen.fill((255, 255, 255))
            self.draw_board()
            pygame.display.flip()
            clock.tick(60)  # Limit to 60 FPS
            
        pygame.quit()

if __name__ == "__main__":
    chess_gui = ChessGUI()
    chess_gui.run()
