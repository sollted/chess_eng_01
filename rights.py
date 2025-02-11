import numpy as np

class Move:
    def __init__(self, start, end, promotion=False):
        self.start = start
        self.end = end
        self.promotion = promotion

def pawn(board, turn):
    moves = []
    is_white = turn == 1
    pawns = np.where(board == (1 if is_white else -1))
    
    for i in range(len(pawns[0])):
        pawn = (pawns[0][i], pawns[1][i])
        direction = 1 if is_white else -1  # White moves down, black moves up
        
        # one square forward
        new_row = pawn[0] + direction
        if 0 <= new_row < 8 and board[new_row, pawn[1]] == 0:
            moves.append(Move(pawn, (new_row, pawn[1]), new_row == 0 or new_row == 7))
        
        # two squares forward
        if (pawn[0] == 1 and is_white) or (pawn[0] == 6 and not is_white):
            new_row = pawn[0] + 2 * direction
            if board[pawn[0] + direction, pawn[1]] == 0 and board[new_row, pawn[1]] == 0:
                moves.append(Move(pawn, (new_row, pawn[1])))
        
        # captures
        for j in [-1, 1]:
            new_row = pawn[0] + direction
            if 0 <= pawn[1] + j < 8 and 0 <= new_row < 8:
                target = board[new_row, pawn[1] + j]
                if (is_white and target < 0) or (not is_white and target > 0):
                    moves.append(Move(pawn, (new_row, pawn[1] + j), new_row == 0 or new_row == 7))
    
    return moves

def knight(board, turn):
    moves = []
    is_white = turn == 1
    knights = np.where(board == (2 if is_white else -2))
    
    for i in range(len(knights[0])):
        knight = (knights[0][i], knights[1][i])
        knight_moves = [
            (2, 1), (2, -1), (-2, 1), (-2, -1),
            (1, 2), (1, -2), (-1, 2), (-1, -2)
        ]
        
        for move in knight_moves:
            new_row = knight[0] + move[0]
            new_col = knight[1] + move[1]
            
            if 0 <= new_row < 8 and 0 <= new_col < 8:
                target = board[new_row, new_col]
                if (is_white and target <= 0) or (not is_white and target >= 0):
                    moves.append(Move(knight, (new_row, new_col)))
    return moves

def bishop(board, turn, equal=None):
    moves = []
    is_white = turn == 1
    value = 3 if equal is None else equal
    bishops = np.where(board == (value if is_white else -value))
    
    for i in range(len(bishops[0])):
        bishop = (bishops[0][i], bishops[1][i])
        directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)]
        
        for direction in directions:
            for j in range(1, 8):
                new_row = bishop[0] + j * direction[0]
                new_col = bishop[1] + j * direction[1]
                
                if 0 <= new_row < 8 and 0 <= new_col < 8:
                    target = board[new_row, new_col]
                    if (is_white and target <= 0) or (not is_white and target >= 0):
                        moves.append(Move(bishop, (new_row, new_col)))
                        if target != 0:  # Stop if we hit any piece
                            break
                    else:
                        break
                else:
                    break
    return moves

def rook(board, turn, equal=None):
    moves = []
    is_white = turn == 1
    value = 4 if equal is None else equal
    rooks = np.where(board == (value if is_white else -value))
    
    for i in range(len(rooks[0])):
        rook = (rooks[0][i], rooks[1][i])
        directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]
        
        for direction in directions:
            for j in range(1, 8):
                new_row = rook[0] + j * direction[0]
                new_col = rook[1] + j * direction[1]
                
                if 0 <= new_row < 8 and 0 <= new_col < 8:
                    target = board[new_row, new_col]
                    if (is_white and target <= 0) or (not is_white and target >= 0):
                        moves.append(Move(rook, (new_row, new_col)))
                        if target != 0:  # Stop if we hit any piece
                            break
                    else:
                        break
                else:
                    break
    return moves

def queen(board, turn):
    moves = []
    is_white = turn == 1
    moves.extend(bishop(board, turn, equal=5))
    moves.extend(rook(board, turn, equal=5))
    return moves

def king(board, turn, castle_rights):
    moves = []
    is_white = turn == 1
    kings = np.where(board == (6 if is_white else -6))
    
    for i in range(len(kings[0])):
        king = (kings[0][i], kings[1][i])
        for move in [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)]:
            new_row = king[0] + move[0]
            new_col = king[1] + move[1]
            
            if 0 <= new_row < 8 and 0 <= new_col < 8:
                target = board[new_row, new_col]
                if (is_white and target <= 0) or (not is_white and target >= 0):
                    moves.append(Move(king, (new_row, new_col)))
        
        # Castling moves
        if is_white and king[0] == 0:  # White king
            if castle_rights['w_king']:
                if board[0, 5] == 0 and board[0, 6] == 0 and board[0, 7] == 4:
                    moves.append(Move(king, (0, 6)))
            if castle_rights['w_queen']:
                if board[0, 1] == 0 and board[0, 2] == 0 and board[0, 3] == 0 and board[0, 0] == 4:
                    moves.append(Move(king, (0, 2)))
        elif not is_white and king[0] == 7:  # Black king
            if castle_rights['b_king']:
                if board[7, 5] == 0 and board[7, 6] == 0 and board[7, 7] == -4:
                    moves.append(Move(king, (7, 6)))
            if castle_rights['b_queen']:
                if board[7, 1] == 0 and board[7, 2] == 0 and board[7, 3] == 0 and board[7, 0] == -4:
                    moves.append(Move(king, (7, 2)))
    
    return moves
