#include "rights.hpp"

std::vector<Move> pawn_moves(const Board& board, int turn) {
    std::vector<Move> moves;
    bool is_white = turn == 1;
    int direction = is_white ? 1 : -1;
    
    for (int row = 0; row < 8; row++) {
        for (int col = 0; col < 8; col++) {
            if (board[row][col] == (is_white ? 1 : -1)) {
                // One square forward
                int new_row = row + direction;
                if (new_row >= 0 && new_row < 8 && board[new_row][col] == 0) {
                    moves.emplace_back(
                        std::make_pair(row, col),
                        std::make_pair(new_row, col),
                        new_row == 0 || new_row == 7
                    );
                }
                
                // Two squares forward
                if ((row == 1 && is_white) || (row == 6 && !is_white)) {
                    new_row = row + 2 * direction;
                    if (board[row + direction][col] == 0 && board[new_row][col] == 0) {
                        moves.emplace_back(
                            std::make_pair(row, col),
                            std::make_pair(new_row, col)
                        );
                    }
                }
                
                // Captures
                for (int j : {-1, 1}) {
                    new_row = row + direction;
                    int new_col = col + j;
                    if (new_col >= 0 && new_col < 8 && new_row >= 0 && new_row < 8) {
                        int target = board[new_row][new_col];
                        if ((is_white && target < 0) || (!is_white && target > 0)) {
                            moves.emplace_back(
                                std::make_pair(row, col),
                                std::make_pair(new_row, new_col),
                                new_row == 0 || new_row == 7
                            );
                        }
                    }
                }
            }
        }
    }
    return moves;
}

std::vector<Move> knight_moves(const Board& board, int turn) {
    std::vector<Move> moves;
    bool is_white = turn == 1;
    const std::vector<std::pair<int, int>> knight_moves = {
        {2, 1}, {2, -1}, {-2, 1}, {-2, -1},
        {1, 2}, {1, -2}, {-1, 2}, {-1, -2}
    };
    
    for (int row = 0; row < 8; row++) {
        for (int col = 0; col < 8; col++) {
            if (board[row][col] == (is_white ? 2 : -2)) {
                for (const auto& move : knight_moves) {
                    int new_row = row + move.first;
                    int new_col = col + move.second;
                    
                    if (new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8) {
                        int target = board[new_row][new_col];
                        if ((is_white && target <= 0) || (!is_white && target >= 0)) {
                            moves.emplace_back(
                                std::make_pair(row, col),
                                std::make_pair(new_row, new_col)
                            );
                        }
                    }
                }
            }
        }
    }
    return moves;
}

std::vector<Move> bishop_moves(const Board& board, int turn, int equal) {
    std::vector<Move> moves;
    bool is_white = turn == 1;
    int value = equal == -1 ? 3 : equal;
    const std::vector<std::pair<int, int>> directions = {{1, 1}, {1, -1}, {-1, 1}, {-1, -1}};
    
    for (int row = 0; row < 8; row++) {
        for (int col = 0; col < 8; col++) {
            if (board[row][col] == (is_white ? value : -value)) {
                for (const auto& dir : directions) {
                    for (int j = 1; j < 8; j++) {
                        int new_row = row + j * dir.first;
                        int new_col = col + j * dir.second;
                        
                        if (new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8) {
                            int target = board[new_row][new_col];
                            if ((is_white && target <= 0) || (!is_white && target >= 0)) {
                                moves.emplace_back(
                                    std::make_pair(row, col),
                                    std::make_pair(new_row, new_col)
                                );
                                if (target != 0) break;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    return moves;
}

std::vector<Move> rook_moves(const Board& board, int turn, int equal) {
    std::vector<Move> moves;
    bool is_white = turn == 1;
    int value = equal == -1 ? 4 : equal;
    const std::vector<std::pair<int, int>> directions = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};
    
    for (int row = 0; row < 8; row++) {
        for (int col = 0; col < 8; col++) {
            if (board[row][col] == (is_white ? value : -value)) {
                for (const auto& dir : directions) {
                    for (int j = 1; j < 8; j++) {
                        int new_row = row + j * dir.first;
                        int new_col = col + j * dir.second;
                        
                        if (new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8) {
                            int target = board[new_row][new_col];
                            if ((is_white && target <= 0) || (!is_white && target >= 0)) {
                                moves.emplace_back(
                                    std::make_pair(row, col),
                                    std::make_pair(new_row, new_col)
                                );
                                if (target != 0) break;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    return moves;
}

std::vector<Move> queen_moves(const Board& board, int turn) {
    std::vector<Move> moves = bishop_moves(board, turn, 5);
    auto rook_mvs = rook_moves(board, turn, 5);
    moves.insert(moves.end(), rook_mvs.begin(), rook_mvs.end());
    return moves;
}

std::vector<Move> king_moves(const Board& board, int turn, const CastlingRights& castle_rights) {
    std::vector<Move> moves;
    bool is_white = turn == 1;
    const std::vector<std::pair<int, int>> directions = {
        {1, 0}, {-1, 0}, {0, 1}, {0, -1},
        {1, 1}, {1, -1}, {-1, 1}, {-1, -1}
    };
    
    for (int row = 0; row < 8; row++) {
        for (int col = 0; col < 8; col++) {
            if (board[row][col] == (is_white ? 6 : -6)) {
                // Normal moves
                for (const auto& dir : directions) {
                    int new_row = row + dir.first;
                    int new_col = col + dir.second;
                    
                    if (new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8) {
                        int target = board[new_row][new_col];
                        if ((is_white && target <= 0) || (!is_white && target >= 0)) {
                            moves.emplace_back(
                                std::make_pair(row, col),
                                std::make_pair(new_row, new_col)
                            );
                        }
                    }
                }
                
                // Castling moves
                if (is_white && row == 0) {
                    if (castle_rights.at("w_king")) {
                        if (board[0][5] == 0 && board[0][6] == 0 && board[0][7] == 4) {
                            moves.emplace_back(
                                std::make_pair(0, 4),
                                std::make_pair(0, 6)
                            );
                        }
                    }
                    if (castle_rights.at("w_queen")) {
                        if (board[0][1] == 0 && board[0][2] == 0 && 
                            board[0][3] == 0 && board[0][0] == 4) {
                            moves.emplace_back(
                                std::make_pair(0, 4),
                                std::make_pair(0, 2)
                            );
                        }
                    }
                } else if (!is_white && row == 7) {
                    if (castle_rights.at("b_king")) {
                        if (board[7][5] == 0 && board[7][6] == 0 && board[7][7] == -4) {
                            moves.emplace_back(
                                std::make_pair(7, 4),
                                std::make_pair(7, 6)
                            );
                        }
                    }
                    if (castle_rights.at("b_queen")) {
                        if (board[7][1] == 0 && board[7][2] == 0 && 
                            board[7][3] == 0 && board[7][0] == -4) {
                            moves.emplace_back(
                                std::make_pair(7, 4),
                                std::make_pair(7, 2)
                            );
                        }
                    }
                }
            }
        }
    }
    return moves;
}