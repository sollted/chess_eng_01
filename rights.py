import numpy as np

class Move:
    def __init__(self, start, end, promotion=False):
        self.start = start
        self.end = end
        self.promotion = promotion

def pawn(board):
    moves = []
    pawns = np.where(board == 1)
    for i in range(len(pawns[0])):
        pawn = (pawns[0][i], pawns[1][i])
        # one square forward
        if board[pawn[0] + 1, pawn[1]] == 0:
            if pawn[0] + 1 < 8:
                moves.append(Move(pawn, (pawn[0] + 1, pawn[1]), pawn[0] + 1 == 7))    
        # two squares forward
        if pawn[0] == 1 and board[pawn[0] + 2, pawn[1]] == 0 and board[pawn[0] + 1, pawn[1]] == 0:
            moves.append(Move(pawn, (pawn[0] + 2, pawn[1])))
        # capture diagonally
        if pawn[0] + 1 < 8 and pawn[1] + 1 < 8 and board[pawn[0] + 1, pawn[1] + 1] < 0:
            moves.append(Move(pawn, (pawn[0] + 1, pawn[1] + 1), pawn[0] + 1 == 7))
        if pawn[0] + 1 < 8 and pawn[1] - 1 >= 0 and board[pawn[0] + 1, pawn[1] - 1] < 0:
            moves.append(Move(pawn, (pawn[0] + 1, pawn[1] - 1), pawn[0] + 1 == 7))
    return moves

def knight(board):
    moves = []
    knights = np.where(board == 2)
    for i in range(len(knights[0])):
        knight = (knights[0][i], knights[1][i])
        # check if the move is valid
        # All possible knight moves as relative coordinates
        knight_moves = [
            (2, 1), (2, -1), (-2, 1), (-2, -1),
            (1, 2), (1, -2), (-1, 2), (-1, -2)
        ]
        
        # Check each possible knight move
        for move in knight_moves:
            new_row = knight[0] + move[0]
            new_col = knight[1] + move[1]
            
            # Verify move is within bounds and target square is empty or enemy piece
            if (0 <= new_row < 8 and 0 <= new_col < 8 and 
                board[new_row, new_col] <= 0):
                moves.append(Move(knight, (new_row, new_col)))
    return moves

def bishop(board, equal=3):
    moves = []
    bishops = np.where(board == equal)
    for i in range(len(bishops[0])):
        bishop = (bishops[0][i], bishops[1][i])
        # Define diagonal directions: up-right, up-left, down-right, down-left
        directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)]
        
        for j in range(1, 8):
            for direction in directions.copy():  # Use copy to safely modify during iteration
                new_row = bishop[0] + j * direction[0]
                new_col = bishop[1] + j * direction[1]
                
                if 0 <= new_row < 8 and 0 <= new_col < 8:
                    if board[new_row, new_col] <= 0:  # Empty or enemy piece
                        moves.append(Move(bishop, (new_row, new_col)))
                        if board[new_row, new_col] < 0:  # Enemy piece blocks further moves
                            directions.remove(direction)
                    else:  # Friendly piece
                        directions.remove(direction)
                else:  # Out of bounds
                    directions.remove(direction)
    return moves

def rook(board, equal=4):
    moves = []
    rooks = np.where(board == equal)
    for i in range(len(rooks[0])):
        rook = (rooks[0][i], rooks[1][i])
        # Define directions once: right, left, down, up
        directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]
        
        for j in range(1, 8):
            for direction in directions.copy():
                    
                new_row = rook[0] + j * direction[0]
                new_col = rook[1] + j * direction[1]
                
                if 0 <= new_row < 8 and 0 <= new_col < 8:
                    if board[new_row, new_col] <= 0:  # Empty or enemy piece
                        moves.append(Move(rook, (new_row, new_col)))
                        if board[new_row, new_col] < 0:  # Enemy piece blocks further moves
                            directions.remove(direction)
                    else:  # Friendly piece
                        directions.remove(direction)
                else:  # Out of bounds
                    directions.remove(direction)
    return moves

def queen(board):
    moves = []
    moves.extend(bishop(board, equal=5))
    moves.extend(rook(board, equal=5))
    return moves

def king(board, castle_rights):
    moves = []
    kings = np.where(board == 6)
    for i in range(len(kings[0])):
        king = (kings[0][i], kings[1][i])
        # Normal king moves
        for move in [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)]:
            new_row = king[0] + move[0]
            new_col = king[1] + move[1]
            if 0 <= new_row < 8 and 0 <= new_col < 8 and board[new_row, new_col] <= 0:
                moves.append(Move(king, (new_row, new_col)))
        
        # Castling moves
        if king[0] == 0:  # King is on their back rank (after board orientation)
            # Kingside castling
            if castle_rights['w_king']:
                if (board[0, 5] == 0 and board[0, 6] == 0 and 
                    board[0, 7] == 4):  # Path is clear and rook is present
                    moves.append(Move(king, (0, 6)))
            
            # Queenside castling
            if castle_rights['w_queen']:
                if (board[0, 1] == 0 and board[0, 2] == 0 and board[0, 3] == 0 and 
                    board[0, 0] == 4):  # Path is clear and rook is present
                    moves.append(Move(king, (0, 2)))
    
    return moves