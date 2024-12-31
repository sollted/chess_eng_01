import time
import numpy as np
from rights import pawn as py_pawn, knight as py_knight, bishop as py_bishop
from rights import rook as py_rook, queen as py_queen, king as py_king
from rights_cpp import pawn as cpp_pawn, knight as cpp_knight, bishop as cpp_bishop
from rights_cpp import rook as cpp_rook, queen as cpp_queen, king as cpp_king

def create_test_board():
    # Create a typical mid-game position
    board = np.zeros((8, 8), dtype=int)
    
    # Set up some pieces
    # Pawns
    board[1] = [1, 1, 1, 0, 1, 1, 1, 1]  # White pawns
    board[6] = [-1, -1, -1, -1, 0, -1, -1, -1]  # Black pawns
    
    # Other pieces
    board[0] = [4, 2, 3, 5, 6, 3, 2, 4]  # White back rank
    board[7] = [-4, -2, -3, -5, -6, -3, -2, -4]  # Black back rank
    
    # Add some pieces in the middle
    board[3, 3] = 1  # White pawn
    board[3, 4] = -1  # Black pawn
    board[4, 2] = 2  # White knight
    board[4, 5] = -2  # Black knight
    
    return board

def time_function(func, *args, iterations=1000):
    start_time = time.time()
    for _ in range(iterations):
        result = func(*args)
    end_time = time.time()
    return end_time - start_time, len(result)

def compare_implementations():
    board = create_test_board()
    castle_rights = {
        'w_king': True,
        'w_queen': True,
        'b_king': True,
        'b_queen': True
    }
    iterations = 10000
    
    tests = [
        ("Pawn moves (White)", 
         (py_pawn, cpp_pawn), 
         (board, 1)),
        ("Pawn moves (Black)", 
         (py_pawn, cpp_pawn), 
         (board, -1)),
        ("Knight moves (White)", 
         (py_knight, cpp_knight), 
         (board, 1)),
        ("Knight moves (Black)", 
         (py_knight, cpp_knight), 
         (board, -1)),
        ("Bishop moves (White)", 
         (py_bishop, cpp_bishop), 
         (board, 1)),
        ("Bishop moves (Black)", 
         (py_bishop, cpp_bishop), 
         (board, -1)),
        ("Rook moves (White)", 
         (py_rook, cpp_rook), 
         (board, 1)),
        ("Rook moves (Black)", 
         (py_rook, cpp_rook), 
         (board, -1)),
        ("Queen moves (White)", 
         (py_queen, cpp_queen), 
         (board, 1)),
        ("Queen moves (Black)", 
         (py_queen, cpp_queen), 
         (board, -1)),
        ("King moves (White)", 
         (py_king, cpp_king), 
         (board, 1, castle_rights)),
        ("King moves (Black)", 
         (py_king, cpp_king), 
         (board, -1, castle_rights))
    ]
    
    print(f"\nPerformance comparison over {iterations} iterations:")
    print("-" * 80)
    print(f"{'Test Case':<25} {'Python Time':<15} {'C++ Time':<15} {'Speedup':<10} {'Moves'}")
    print("-" * 80)
    
    for test_name, (py_func, cpp_func), args in tests:
        py_time, py_moves = time_function(py_func, *args, iterations=iterations)
        cpp_time, cpp_moves = time_function(cpp_func, *args, iterations=iterations)
        
        speedup = py_time / cpp_time if cpp_time > 0 else float('inf')
        
        print(f"{test_name:<25} {py_time:>8.4f}s      {cpp_time:>8.4f}s      {speedup:>6.2f}x    {py_moves}")
        
        # Verify that both implementations return the same number of moves
        if py_moves != cpp_moves:
            print(f"WARNING: Move count mismatch for {test_name}! Python: {py_moves}, C++: {cpp_moves}")

if __name__ == "__main__":
    compare_implementations() 