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
        # Convert pixel coordinates to board coordinates
        col = pos[0] // self.SQUARE_SIZE
        row = 7 - (pos[1] // self.SQUARE_SIZE)  # Flip row coordinate
        
        # Adjust coordinates for black's turn
        if self.board.turn == 1:
            row = 7 - row
        
        # Only allow human moves during white's turn
        if self.board.turn == -1:  # Black's turn
            return
        
        if self.selected_square is None:
            # First click - select piece if it's valid
            piece = self.board.board[row][col]
            if piece > 0:  # Only allow selecting white pieces
                self.selected_square = (row, col)
                
                # Generate legal moves for the selected piece
                self.legal_moves = [
                    move for move in self.board.generate_legal_moves()
                    if move.start == (row, col)
                ]
        else:
            # Second click - attempt to make move
            start = self.selected_square
            end = (row, col)
            move = Move(start, end, promotion=(row == 7 and self.board.board[start[0]][start[1]] == 1))
            
            # Check if move is legal
            matching_move = next((m for m in self.legal_moves if m.start == move.start and m.end == move.end), None)
            
            if matching_move:
                # Make the move
                self.board.make_move(matching_move)
                # Flip the board for the next player
                self.board.flip_board()
                self.board.white = not self.board.white

                # AI makes a move for black
                if self.board.turn == -1:  # Black's turn
                    best_move = self.board.find_best_move(depth=6)  # Increase depth to at least 4
                    if best_move:
                        self.board.make_move(best_move)
                        self.board.flip_board()
                        self.board.white = not self.board.white
            
            self.selected_square = None
            self.legal_moves = []
    
    def draw_board(self):
        for row in range(8):
            for col in range(8):
                # Always keep white on top visually, but flip the visual board for black's turn
                if self.board.turn == 1:  # White's turn
                    # Invert piece values for white's turn
                    board_row = row
                    board_col = col
                else:  # Black's turn
                    board_row = 7 - row
                    board_col = col
                
                # Determine square color - base it on visual position (row, col), not board position
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
                
                # Draw pieces
                piece_value = self.board.board[board_row][board_col]
                if piece_value != 0:
                    if self.board.turn == -1:  # If it's black's turn, invert the piece values
                        piece_value = -piece_value
                    piece_img = self.get_piece_image(piece_value)
                    if piece_img:
                        self.screen.blit(piece_img, 
                                    (col * self.SQUARE_SIZE, row * self.SQUARE_SIZE))

    def run(self):
        running = True
        while running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    running = False
                elif event.type == pygame.MOUSEBUTTONDOWN:
                    self.handle_click(event.pos)
            
            self.screen.fill((255, 255, 255))
            self.draw_board()

            pygame.display.flip()
        pygame.quit()

if __name__ == "__main__":
    chess_gui = ChessGUI()
    chess_gui.run()
