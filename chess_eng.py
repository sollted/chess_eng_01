import numpy as np
from rights import *
from tqdm import tqdm

class Board:
    
    def __init__(self):
        self.board = np.zeros((8, 8), dtype=int)
        self.turn = 1 # 1 for white, -1 for black
        self.white = False
        self.castling_rights = {
            'w_king': True,
            'w_queen': True,
            'b_king': True,
            'b_queen': True,
        }
        self.pieces = {
            'empty': 0,
            'w_pawn': 1,
            'w_knight': 2,
            'w_bishop': 3,
            'w_rook': 4,
            'w_queen': 5,
            'w_king': 6,
            'b_pawn': -1,
            'b_knight': -2,
            'b_bishop': -3,
            'b_rook': -4,
            'b_queen': -5,
            'b_king': -6
        }
        self.value_map = {
            'empty': 0,
            'pawn': 1,
            'knight': 3,
            'bishop': 3,
            'rook': 5,
            'queen': 9,
            'king': 100,
        }


    def flip_board(self):
        # Flip the board vertically and negate all pieces
        self.board = -np.flipud(self.board)
        self.white = not self.white
        
        # Swap white and black castling rights
        self.castling_rights['w_king'], self.castling_rights['b_king'] = \
            self.castling_rights['b_king'], self.castling_rights['w_king']
        self.castling_rights['w_queen'], self.castling_rights['b_queen'] = \
            self.castling_rights['b_queen'], self.castling_rights['w_queen']

    def to_fen(self):
        fen = ""
        empty_count = 0
        
        # Board position
        for rank in range(7, -1, -1):
            for file in range(8):
                piece = self.board[rank][file]
                
                if piece == 0:
                    empty_count += 1
                else:
                    if empty_count > 0:
                        fen += str(empty_count)
                        empty_count = 0
                        
                    if piece == self.pieces['w_pawn']: fen += 'P'
                    elif piece == self.pieces['w_knight']: fen += 'N'
                    elif piece == self.pieces['w_bishop']: fen += 'B'
                    elif piece == self.pieces['w_rook']: fen += 'R'
                    elif piece == self.pieces['w_queen']: fen += 'Q'
                    elif piece == self.pieces['w_king']: fen += 'K'
                    elif piece == self.pieces['b_pawn']: fen += 'p'
                    elif piece == self.pieces['b_knight']: fen += 'n'
                    elif piece == self.pieces['b_bishop']: fen += 'b'
                    elif piece == self.pieces['b_rook']: fen += 'r'
                    elif piece == self.pieces['b_queen']: fen += 'q'
                    elif piece == self.pieces['b_king']: fen += 'k'
            
            if empty_count > 0:
                fen += str(empty_count)
                empty_count = 0
                
            if rank > 0:
                fen += '/'
        
        # Add turn
        fen += ' w ' if self.turn == 1 else ' b '
        
        # Add castling rights
        castling = ''
        if self.castling_rights['w_king']: castling += 'K'
        if self.castling_rights['w_queen']: castling += 'Q'
        if self.castling_rights['b_king']: castling += 'k'
        if self.castling_rights['b_queen']: castling += 'q'
        fen += castling if castling else '-'
        
        # Add placeholder for en passant and move counters
        fen += ' - 0 1'
        
        return fen

    def from_fen(self, fen):
        # Split FEN into its components
        parts = fen.split()
        board_fen = parts[0]
        
        # Get turn if provided
        if len(parts) > 1:
            self.turn = 1 if parts[1] == 'w' else -1
        
        # Get castling rights if provided
        if len(parts) > 2:
            self.castling_rights = {
                'w_king': 'K' in parts[2],
                'w_queen': 'Q' in parts[2],
                'b_king': 'k' in parts[2],
                'b_queen': 'q' in parts[2]
            }
        
        # Reset board to empty
        self.board = np.zeros((8, 8), dtype=int)
        
        # Parse board position
        rank = 7
        file = 0
        
        for char in board_fen:
            if char == '/':
                rank -= 1
                file = 0
            elif char.isdigit():
                file += int(char)
            else:
                # Convert FEN character to piece number
                if char == 'P': piece = self.pieces['w_pawn']
                elif char == 'N': piece = self.pieces['w_knight']
                elif char == 'B': piece = self.pieces['w_bishop'] 
                elif char == 'R': piece = self.pieces['w_rook']
                elif char == 'Q': piece = self.pieces['w_queen']
                elif char == 'K': piece = self.pieces['w_king']
                elif char == 'p': piece = self.pieces['b_pawn']
                elif char == 'n': piece = self.pieces['b_knight']
                elif char == 'b': piece = self.pieces['b_bishop']
                elif char == 'r': piece = self.pieces['b_rook']
                elif char == 'q': piece = self.pieces['b_queen']
                elif char == 'k': piece = self.pieces['b_king']
                else:
                    continue
                
                if 0 <= rank < 8 and 0 <= file < 8:
                    self.board[rank][file] = piece
                file += 1

    def in_check(self, white):
        # if white is true, return true if white is in check
        if white != self.white:
            self.flip_board()
            moves = self.generate_legal_moves(is_pseudo=True)
            self.flip_board()
        else:
            moves = self.generate_legal_moves(is_pseudo=True)
        
        # Look for the king (6 for white king, -6 for black king)
        king_value = 6 if white else -6
        kings = np.where(self.board == king_value)
        if kings[0].size > 0:
            king = (kings[0][0], kings[1][0])
            for move in moves:
                if move.end == king:
                    return True
        return False
    
    def generate_legal_moves(self, is_pseudo=False):
        moves = []
        moves.extend(pawn(self.board))
        moves.extend(knight(self.board))
        moves.extend(bishop(self.board))
        moves.extend(rook(self.board))
        moves.extend(queen(self.board))
        moves.extend(king(self.board, self.castling_rights))

        # If we flipped the board, flip it back and transform the move coordinates
        if self.white:
            # Transform moves to match original board orientation
            for move in moves:
                move.start = (7-move.start[0], move.start[1])
                move.end = (7-move.end[0], move.end[1])

        if not is_pseudo:
            legal_moves = []
            for move in moves:
                piece, rights = self.make_move(move)
                if not self.in_check(not self.white):  # Check if our king is safe after move
                    legal_moves.append(move)
                self.undo_move(move, piece, rights)
            return legal_moves
        return moves

    def make_move(self, move):
        # Store the piece at the end position (captured piece)
        captured_piece = self.board[move.end[0]][move.end[1]]
        moving_piece = self.board[move.start[0]][move.start[1]]
        
        # Store original castling rights
        original_castling_rights = self.castling_rights.copy()
        
        # Move the piece
        self.board[move.end[0]][move.end[1]] = self.board[move.start[0]][move.start[1]]
        self.board[move.start[0]][move.start[1]] = 0
        
        # Handle castling
        if abs(moving_piece) == self.pieces['w_king']:
            # Update castling rights when king moves
            if moving_piece > 0:
                self.castling_rights['w_king'] = False
                self.castling_rights['w_queen'] = False
            else:
                self.castling_rights['b_king'] = False
                self.castling_rights['b_queen'] = False
            
            # Handle castling move
            if abs(move.start[1] - move.end[1]) == 2:
                # Kingside castling
                if move.end[1] == 6:
                    self.board[move.start[0]][5] = self.board[move.start[0]][7]  # Move rook
                    self.board[move.start[0]][7] = 0  # Clear rook's original position
                # Queenside castling
                elif move.end[1] == 2:
                    self.board[move.start[0]][3] = self.board[move.start[0]][0]  # Move rook
                    self.board[move.start[0]][0] = 0  # Clear rook's original position
        
        # Update castling rights when rook moves
        elif abs(moving_piece) == self.pieces['w_rook']:
            if move.start[0] == 0 and move.start[1] == 0:
                self.castling_rights['w_queen'] = False
            elif move.start[0] == 0 and move.start[1] == 7:
                self.castling_rights['w_king'] = False
            elif move.start[0] == 7 and move.start[1] == 0:
                self.castling_rights['b_queen'] = False
            elif move.start[0] == 7 and move.start[1] == 7:
                self.castling_rights['b_king'] = False
        
        # Handle pawn promotion
        if move.promotion:
            self.board[move.end[0]][move.end[1]] = 5 if moving_piece > 0 else -5  # Promote to queen
        
        # Switch turn
        self.turn *= -1
        
        return captured_piece, original_castling_rights

    def undo_move(self, move, captured_piece, original_castling_rights):
        moving_piece = self.board[move.end[0]][move.end[1]]
        
        # Move piece back to start position
        self.board[move.start[0]][move.start[1]] = self.board[move.end[0]][move.end[1]]
        
        # If it was a promotion, restore original pawn
        if move.promotion:
            self.board[move.start[0]][move.start[1]] = 1 if moving_piece > 0 else -1
        
        # Restore captured piece if any
        self.board[move.end[0]][move.end[1]] = captured_piece
        
        # Handle undoing castling
        if abs(moving_piece) == self.pieces['w_king']:
            if abs(move.start[1] - move.end[1]) == 2:
                # Undo kingside castling
                if move.end[1] == 6:
                    self.board[move.start[0]][7] = self.board[move.start[0]][5]  # Move rook back
                    self.board[move.start[0]][5] = 0  # Clear rook's temporary position
                # Undo queenside castling
                elif move.end[1] == 2:
                    self.board[move.start[0]][0] = self.board[move.start[0]][3]  # Move rook back
                    self.board[move.start[0]][3] = 0  # Clear rook's temporary position
        
        # Restore original castling rights
        self.castling_rights = original_castling_rights
        
        # Switch turn back
        self.turn *= -1

    def game_over(self):
        if len(self.generate_legal_moves()) == 0:
            # If no legal moves and in check, it's checkmate
            if self.in_check(not self.white):
                return True, "black loss" if self.white else "white loss"
            # If no legal moves but not in check, it's stalemate
            return True, "draw by stalemate"
        return False, "keep playing"

    def evaluate_board(self):
        score = 0
        
        # Material evaluation with slightly adjusted values
        piece_values = {
            'pawn': 100,    # Base value 1.0
            'knight': 320,  # Base value 3.2
            'bishop': 330,  # Base value 3.3
            'rook': 500,    # Base value 5.0
            'queen': 900,   # Base value 9.0
            'king': 20000   # Very high to prioritize king safety
        }
        
        for i in range(8):
            for j in range(8):
                piece = self.board[i][j]
                if piece != 0:
                    # Get piece type (pawn, knight, etc.)
                    piece_type = list(self.pieces.keys())[list(self.pieces.values()).index(abs(piece))].split('_')[1]
                    
                    # Add material value
                    score += piece_values[piece_type] * (1 if piece > 0 else -1)
                    
                    # Simple positional bonuses
                    if abs(piece) == 1:  # Pawns
                        # Bonus for advanced pawns
                        if piece > 0:
                            score += i * 10  # More points as pawn advances
                        else:
                            score -= (7-i) * 10
                    
                    # Small bonus for controlling center with any piece
                    if 2 <= i <= 5 and 2 <= j <= 5:
                        score += 10 * (1 if piece > 0 else -1)
        
        return score
    
    def find_best_move(self, depth=5):
        def minimax(depth, alpha, beta, maximizing_player):
            if depth == 0:
                # Don't negate the evaluation since board flipping already handles perspective
                return self.evaluate_board()
            
            # Check for game over
            is_over, result = self.game_over()
            if is_over:
                if result == "white loss":
                    return -1000
                elif result == "black loss":
                    return 1000
                else:  # Draw
                    return 0

            if maximizing_player:
                max_eval = float('-inf')
                moves = self.generate_legal_moves()
                for move in moves:
                    piece, rights = self.make_move(move)
                    eval = minimax(depth - 1, alpha, beta, False)
                    self.undo_move(move, piece, rights)
                    max_eval = max(max_eval, eval)
                    alpha = max(alpha, eval)
                    if beta <= alpha:
                        break  # Beta cut-off
                return max_eval
            else:
                min_eval = float('inf')
                moves = self.generate_legal_moves()
                for move in moves:
                    piece, rights = self.make_move(move)
                    eval = minimax(depth - 1, alpha, beta, True)
                    self.undo_move(move, piece, rights)
                    min_eval = min(min_eval, eval)
                    beta = min(beta, eval)
                    if beta <= alpha:
                        break  # Alpha cut-off
                return min_eval

        best_move = None
        # Always maximize since board flipping handles perspective
        best_value = float('-inf')
        alpha = float('-inf')
        beta = float('inf')
        
        legal_moves = self.generate_legal_moves()
        
        if not legal_moves:
            return None
        
        # Add progress bar
        with tqdm(total=len(legal_moves), desc=f"Searching depth {depth}") as pbar:
            # Evaluate each move
            for move in legal_moves:
                piece, rights = self.make_move(move)
                value = minimax(depth - 1, alpha, beta, False)
                self.undo_move(move, piece, rights)
                
                if value > best_value:
                    best_value = value
                    best_move = move
                alpha = max(alpha, value)
                
                # Update progress bar
                pbar.update(1)
                
                if beta <= alpha:
                    break
        
        return best_move
