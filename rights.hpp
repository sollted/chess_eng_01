#pragma once
#include <vector>
#include <array>
#include <map>

struct Move {
    std::pair<int, int> start;
    std::pair<int, int> end;
    bool promotion;
    
    Move(std::pair<int, int> s, std::pair<int, int> e, bool p = false) 
        : start(s), end(e), promotion(p) {}
};

using Board = std::array<std::array<int, 8>, 8>;
using CastlingRights = std::map<std::string, bool>;

std::vector<Move> pawn_moves(const Board& board, int turn);
std::vector<Move> knight_moves(const Board& board, int turn);
std::vector<Move> bishop_moves(const Board& board, int turn, int equal = -1);
std::vector<Move> rook_moves(const Board& board, int turn, int equal = -1);
std::vector<Move> queen_moves(const Board& board, int turn);
std::vector<Move> king_moves(const Board& board, int turn, const CastlingRights& castle_rights); 