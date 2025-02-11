import numpy as np
from tqdm import tqdm
from rights import Move

# Create a flag to switch between C++ and Python implementations
USE_CPP_RIGHTS = True

if USE_CPP_RIGHTS:
    from rights_cpp import (
        pawn as pawn,
        knight as knight,
        bishop as bishop,
        rook as rook,
        queen as queen,
        king as king
    )
else:
    from rights import pawn, knight, bishop, rook, queen, king

class Board:
    
    def __init__(self):
        self.board = np.zeros((8, 8), dtype=int)
        self.turn = 1 # 1 for white, -1 for black
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
        
        # Add piece-square tables as class attributes
        self.pawn_table = np.array([
            [ 0,  0,  0,  0,  0,  0,  0,  0],
            [50, 50, 50, 50, 50, 50, 50, 50],
            [10, 10, 20, 30, 30, 20, 10, 10],
            [ 5,  5, 10, 25, 25, 10,  5,  5],
            [ 0,  0,  0, 20, 20,  0,  0,  0],
            [ 5, -5,-10,  0,  0,-10, -5,  5],
            [ 5, 10, 10,-20,-20, 10, 10,  5],
            [ 0,  0,  0,  0,  0,  0,  0,  0]
        ])
        
        self.knight_table = np.array([
            [-50,-40,-30,-30,-30,-30,-40,-50],
            [-40,-20,  0,  0,  0,  0,-20,-40],
            [-30,  0, 10, 15, 15, 10,  0,-30],
            [-30,  5, 15, 20, 20, 15,  5,-30],
            [-30,  0, 15, 20, 20, 15,  0,-30],
            [-30,  5, 10, 15, 15, 10,  5,-30],
            [-40,-20,  0,  5,  5,  0,-20,-40],
            [-50,-40,-30,-30,-30,-30,-40,-50]
        ])
        
        # Add position history to track repetitions
        self.position_history = []
        
        # Add piece_values as a class attribute
        self.piece_values = {
            1: 100,    # Pawn
            2: 320,    # Knight
            3: 330,    # Bishop
            4: 500,    # Rook
            5: 900,    # Queen
            6: 20000   # King
        }

        self.transposition_table = {}  # Add at class level in __init__

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
        # Generate opponent's moves directly
        moves = self.generate_legal_moves(is_pseudo=True, for_white=(not white))
        
        # Look for the king
        king_value = 6 if white else -6
        kings = np.where(self.board == king_value)
        if kings[0].size > 0:
            king = (kings[0][0], kings[1][0])
            for move in moves:
                if move.end == king:
                    return True
        return False
    
    def generate_legal_moves(self, is_pseudo=False, for_white=None):
        moves = []
        # Store the original turn
        original_turn = self.turn
        
        # Use provided color or current turn if not specified
        """
        if for_white is not None:
            self.turn = 1 if for_white else -1
        """
        
        try:
            if self.turn == 1:  # White's moves
                moves.extend(pawn(self.board, 1))
                moves.extend(knight(self.board, 1))
                moves.extend(bishop(self.board, 1))
                moves.extend(rook(self.board, 1))
                moves.extend(queen(self.board, 1))
                moves.extend(king(self.board, 1, self.castling_rights))
            else:  # Black's moves
                moves.extend(pawn(self.board, -1))
                moves.extend(knight(self.board, -1))
                moves.extend(bishop(self.board, -1))
                moves.extend(rook(self.board, -1))
                moves.extend(queen(self.board, -1))
                moves.extend(king(self.board, -1, self.castling_rights))

            if not is_pseudo:
                legal_moves = []
                for move in moves:
                    piece, rights = self.make_move(move)
                    if not self.in_check(self.turn == 1):  # Check if our king is safe after move
                        legal_moves.append(move)
                    self.undo_move(move, piece, rights)
                return legal_moves
            return moves
        
        finally:
            # Always restore the original turn, even if an error occurs
            self.turn = original_turn

    def make_move(self, move):
        # Store position before making move
        current_fen = self.to_fen().split(' ')[0]
        self.position_history.append(current_fen)
        
        # Limit history length to prevent memory issues
        if len(self.position_history) > 50:
            self.position_history.pop(0)
        
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
        

    def game_over(self):
        if len(self.generate_legal_moves()) == 0:
            if self.in_check(self.turn == 1):
                return True, "black loss" if self.turn == 1 else "white loss"
            return True, "draw by stalemate"
        return False, "keep playing"

    def evaluate_board(self):
        """
        Evaluates the board position from White's perspective.
        Positive values mean White is winning, negative values mean Black is winning.
        """
        score = 0
        
        # Material counting (most important factor)
        for row in range(8):
            for col in range(8):
                piece = self.board[row][col]
                if piece != 0:
                    # Base piece value
                    value = self.piece_values[abs(piece)]
                    score += value if piece > 0 else -value
        
        # Simple positional bonuses
        for row in range(8):
            for col in range(8):
                piece = self.board[row][col]
                if piece != 0:
                    # Center control (small bonus)
                    if 2 <= row <= 5 and 2 <= col <= 5:
                        score += 10 if piece > 0 else -10
                    
                    # Pawn structure
                    if abs(piece) == 1:  # Pawns
                        # Bonus for advanced pawns
                        if piece > 0:  # White pawns
                            score += (row - 1) * 10
                        else:  # Black pawns
                            score -= (6 - row) * 10
        
        # King safety (simplified)
        white_king_pos = np.where(self.board == 6)
        black_king_pos = np.where(self.board == -6)
        
        if white_king_pos[0].size > 0 and black_king_pos[0].size > 0:
            w_king = (white_king_pos[0][0], white_king_pos[1][0])
            b_king = (black_king_pos[0][0], black_king_pos[1][0])
            
            # Penalize exposed kings
            if w_king[0] > 1:  # White king moved away from back rank
                score -= 50
            if b_king[0] < 6:  # Black king moved away from back rank
                score += 50
        
        # Mobility (simplified)
        self.turn = 1  # Temporarily set turn to white
        white_moves = len(self.generate_legal_moves())
        self.turn = -1  # Set turn to black
        black_moves = len(self.generate_legal_moves())
        self.turn = 1  # Restore original turn
        
        score += (white_moves - black_moves) * 5  # Small bonus for mobility
        
        return score
    
    def find_best_move(self, depth=4):
        """
        Optimized version with move ordering, transposition table, and basic quiescence search
        """
        def move_value(move):
            """Order moves to improve alpha-beta pruning efficiency"""
            piece = self.board[move.start[0]][move.start[1]]
            captured = self.board[move.end[0]][move.end[1]]
            
            # Base score: MVV-LVA (Most Valuable Victim - Least Valuable Attacker)
            score = 0
            if captured != 0:
                score = 10 * self.piece_values[abs(captured)] - self.piece_values[abs(piece)]
            
            # Bonus for promotions
            if move.promotion:
                score += 900
            
            # Bonus for attacking center squares
            if 2 <= move.end[0] <= 5 and 2 <= move.end[1] <= 5:
                score += 10
            
            return score

        def quiescence_search(alpha, beta, depth=0, max_depth=4):
            """Search capture moves to avoid horizon effect"""
            stand_pat = self.evaluate_board()
            
            if stand_pat >= beta:
                return beta
            if stand_pat > alpha:
                alpha = stand_pat
            if depth >= max_depth:
                return alpha

            # Only look at captures
            moves = [move for move in self.generate_legal_moves() 
                    if self.board[move.end[0]][move.end[1]] != 0]
            moves.sort(key=move_value, reverse=True)

            for move in moves:
                piece, rights = self.make_move(move)
                self.turn *= -1
                score = -quiescence_search(-beta, -alpha, depth + 1)
                self.turn *= -1
                self.undo_move(move, piece, rights)
                
                if score >= beta:
                    return beta
                if score > alpha:
                    alpha = score

            return alpha

        def minimax(depth, alpha, beta, maximizing_player):
            # Check transposition table
            board_hash = hash(str(self.board.tobytes()) + str(maximizing_player))
            if board_hash in self.transposition_table:
                stored_depth, stored_value, stored_move = self.transposition_table[board_hash]
                if stored_depth >= depth:
                    return stored_value, stored_move

            if depth == 0:
                return quiescence_search(alpha, beta), None

            legal_moves = self.generate_legal_moves()
            if not legal_moves:
                return -20000 if maximizing_player else 20000, None

            # Move ordering - sort moves by their estimated value
            legal_moves.sort(key=move_value, reverse=True)
            
            best_move = None
            if maximizing_player:
                max_eval = float('-inf')
                for move in legal_moves:
                    piece, rights = self.make_move(move)
                    self.turn *= -1
                    eval, _ = minimax(depth - 1, alpha, beta, False)
                    self.turn *= -1
                    self.undo_move(move, piece, rights)
                    
                    if eval > max_eval:
                        max_eval = eval
                        best_move = move
                    alpha = max(alpha, eval)
                    if beta <= alpha:
                        break
                # Store in transposition table
                self.transposition_table[board_hash] = (depth, max_eval, best_move)
                return max_eval, best_move
            else:
                min_eval = float('inf')
                for move in legal_moves:
                    piece, rights = self.make_move(move)
                    self.turn *= -1
                    eval, _ = minimax(depth - 1, alpha, beta, True)
                    self.turn *= -1
                    self.undo_move(move, piece, rights)
                    
                    if eval < min_eval:
                        min_eval = eval
                        best_move = move
                    beta = min(beta, eval)
                    if beta <= alpha:
                        break
                # Store in transposition table
                self.transposition_table[board_hash] = (depth, min_eval, best_move)
                return min_eval, best_move

        # Call minimax with initial parameters
        _, best_move = minimax(depth, float('-inf'), float('inf'), self.turn == 1)
        return best_move
    
