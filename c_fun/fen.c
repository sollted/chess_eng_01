#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

struct board
{
    uint64_t pawn_w;
    uint64_t knight_w;
    uint64_t bishoop_w;
    uint64_t rook_w;
    uint64_t qeen_w;
    uint64_t king_w;
    uint64_t pawn_b;
    uint64_t knight_b;
    uint64_t bishoop_b;
    uint64_t rook_b;
    uint64_t qeen_b;
    uint64_t king_b;
    bool white_move;
    int castle_rights; // 4 bits: 1000 (8) = White kingside
                      //         0100 (4) = White queenside
                      //         0010 (2) = Black kingside
                      //         0001 (1) = Black queenside
};

struct move {
    uint64_t from;     // bitboard with single bit set for source square
    uint64_t to;       // bitboard with single bit set for destination square
    bool is_capture;   // whether this move captures a piece
    bool promotion;    // true if pawn promotes to queen, false otherwise
};

struct move_list {
    struct move* moves;
    int count;
    int capacity;
};

struct move_list* create_move_list(int initial_capacity) {
    struct move_list* list = malloc(sizeof(struct move_list));
    if (!list) return NULL;
    
    list->moves = malloc(sizeof(struct move) * initial_capacity);
    if (!list->moves) {
        free(list);
        return NULL;
    }
    
    list->count = 0;
    list->capacity = initial_capacity;
    return list;
}

void add_move(struct move_list* list, uint64_t from, uint64_t to, bool is_capture, bool promotion) {
    if (list->count >= list->capacity) {
        int new_capacity = list->capacity * 2;
        struct move* new_moves = realloc(list->moves, sizeof(struct move) * new_capacity);
        if (!new_moves) return;
        list->moves = new_moves;
        list->capacity = new_capacity;
    }
    
    list->moves[list->count].from = from;
    list->moves[list->count].to = to;
    list->moves[list->count].is_capture = is_capture;
    list->moves[list->count].promotion = promotion;
    list->count++;
}

// Helper function to make a move temporarily
static void make_temp_move(struct board* temp, uint64_t from, uint64_t to, bool white) {
    // Clear 'from' bit and set 'to' bit in appropriate bitboard
    uint64_t *piece_bb = NULL;
    
    if (white) {
        if (from & temp->pawn_w) piece_bb = &temp->pawn_w;
        else if (from & temp->knight_w) piece_bb = &temp->knight_w;
        else if (from & temp->bishoop_w) piece_bb = &temp->bishoop_w;
        else if (from & temp->rook_w) piece_bb = &temp->rook_w;
        else if (from & temp->qeen_w) piece_bb = &temp->qeen_w;
        else if (from & temp->king_w) piece_bb = &temp->king_w;
    } else {
        if (from & temp->pawn_b) piece_bb = &temp->pawn_b;
        else if (from & temp->knight_b) piece_bb = &temp->knight_b;
        else if (from & temp->bishoop_b) piece_bb = &temp->bishoop_b;
        else if (from & temp->rook_b) piece_bb = &temp->rook_b;
        else if (from & temp->qeen_b) piece_bb = &temp->qeen_b;
        else if (from & temp->king_b) piece_bb = &temp->king_b;
    }
    
    if (piece_bb) {
        *piece_bb &= ~from;  // Clear 'from' square
        *piece_bb |= to;     // Set 'to' square
    }
}

// Helper function to check if a move is legal (doesn't leave king in check)
static bool is_move_legal(const struct board* b, uint64_t from, uint64_t to, bool white) {
    // Create a temporary board to test the move
    struct board temp = *b;
    
    // Make the move on temporary board
    make_temp_move(&temp, from, to, white);
    
    // Check if our king is in check after the move
    return !is_in_check(&temp, white);
}

// Modified add_move to only add legal moves
static void add_legal_move(struct move_list* list, const struct board* b, uint64_t from, uint64_t to, 
                          bool is_capture, bool promotion, bool white) {
    if (is_move_legal(b, from, to, white)) {
        add_move(list, from, to, is_capture, promotion);
    }
}

struct move_list* generate_legal_moves(const struct board* b, bool white) {
    struct move_list* moves = create_move_list(64);  // Initial capacity of 64 moves
    if (!moves) return NULL;

    uint64_t own_pieces = white ? 
        (b->pawn_w | b->knight_w | b->bishoop_w | b->rook_w | b->qeen_w | b->king_w) :
        (b->pawn_b | b->knight_b | b->bishoop_b | b->rook_b | b->qeen_b | b->king_b);
    
    uint64_t enemy_pieces = white ?
        (b->pawn_b | b->knight_b | b->bishoop_b | b->rook_b | b->qeen_b | b->king_b) :
        (b->pawn_w | b->knight_w | b->bishoop_w | b->rook_w | b->qeen_w | b->king_w);
    
    uint64_t all_pieces = own_pieces | enemy_pieces;
    uint64_t empty_squares = ~all_pieces;

    // Generate pawn moves
    uint64_t pawns = white ? b->pawn_w : b->pawn_b;
    while (pawns) {
        // Get least significant set bit and clear it
        uint64_t pawn = pawns & -pawns;
        pawns &= ~pawn;
        
        int sq = __builtin_ctzll(pawn);
        int rank = sq / 8;
        int file = sq % 8;

        // Single push
        uint64_t target = white ? (pawn << 8) : (pawn >> 8);
        if (target & empty_squares) {
            bool is_promotion = (white && rank == 6) || (!white && rank == 1);
            add_legal_move(moves, b, pawn, target, false, is_promotion, white);

            // Double push from starting position
            if ((white && rank == 1) || (!white && rank == 6)) {
                uint64_t double_target = white ? (pawn << 16) : (pawn >> 16);
                if ((double_target & empty_squares) && (target & empty_squares)) {
                    add_legal_move(moves, b, pawn, double_target, false, false, white);
                }
            }
        }

        // Captures
        if (file > 0) {  // Can capture left
            target = white ? (pawn << 7) : (pawn >> 9);
            if (target & enemy_pieces) {
                bool is_promotion = (white && rank == 6) || (!white && rank == 1);
                add_legal_move(moves, b, pawn, target, true, is_promotion, white);
            }
        }
        if (file < 7) {  // Can capture right
            target = white ? (pawn << 9) : (pawn >> 7);
            if (target & enemy_pieces) {
                bool is_promotion = (white && rank == 6) || (!white && rank == 1);
                add_legal_move(moves, b, pawn, target, true, is_promotion, white);
            }
        }
    }
    
    

    // TODO: Add moves for other pieces (knights, bishops, rooks, queens, king)
    // Similar pattern: iterate through pieces, generate possible moves,
    // check if moves are legal (don't leave king in check)

    return moves;
}

const char* board_to_fen(const struct board *b)
{
    if (!b) {
        return NULL;
    }

    // Allocate memory for FEN string (max length estimate)
    char* fen = malloc(100);
    if (!fen) {
        return NULL;
    }
    
    char* ptr = fen;
    int empty = 0;

    // Iterate through board positions
    for (int rank = 7; rank >= 0; rank--) {
        for (int file = 0; file < 8; file++) {
            uint64_t pos = 1ULL << (rank * 8 + file);
            char piece = '\0';

            if (b->pawn_w & pos) piece = 'P';
            else if (b->knight_w & pos) piece = 'N';
            else if (b->bishoop_w & pos) piece = 'B';
            else if (b->rook_w & pos) piece = 'R';
            else if (b->qeen_w & pos) piece = 'Q';
            else if (b->king_w & pos) piece = 'K';
            else if (b->pawn_b & pos) piece = 'p';
            else if (b->knight_b & pos) piece = 'n';
            else if (b->bishoop_b & pos) piece = 'b';
            else if (b->rook_b & pos) piece = 'r';
            else if (b->qeen_b & pos) piece = 'q';
            else if (b->king_b & pos) piece = 'k';

            if (piece) {
                if (empty) {
                    *ptr++ = empty + '0';
                    empty = 0;
                }
                *ptr++ = piece;
            } else {
                empty++;
            }
        }
        
        if (empty) {
            *ptr++ = empty + '0';
            empty = 0;
        }
        
        if (rank > 0) {
            *ptr++ = '/';
        }
    }

    // Add active color
    *ptr++ = ' ';
    *ptr++ = b->white_move ? 'w' : 'b';
    
    // Add castling rights
    *ptr++ = ' ';
    bool has_castle = false;
    if (b->castle_rights & 8) { *ptr++ = 'K'; has_castle = true; }
    if (b->castle_rights & 4) { *ptr++ = 'Q'; has_castle = true; }
    if (b->castle_rights & 2) { *ptr++ = 'k'; has_castle = true; }
    if (b->castle_rights & 1) { *ptr++ = 'q'; has_castle = true; }
    if (!has_castle) { *ptr++ = '-'; }

    // Add en passant target (not tracked in struct, using '-')
    *ptr++ = ' ';
    *ptr++ = '-';

    // Add halfmove and fullmove (not tracked in struct, using 0 1)
    *ptr++ = ' ';
    *ptr++ = '0';
    *ptr++ = ' ';
    *ptr++ = '1';

    *ptr = '\0';
    return fen;
}
const struct board* fen_to_board(char *fen)
{
    struct board* new_board = malloc(sizeof(struct board));
    if (!new_board || !fen) {
        return NULL;
    }

    // Initialize all bitboards to 0
    new_board->pawn_w = 0;
    new_board->knight_w = 0;
    new_board->bishoop_w = 0;
    new_board->rook_w = 0;
    new_board->qeen_w = 0;
    new_board->king_w = 0;
    new_board->pawn_b = 0;
    new_board->knight_b = 0;
    new_board->bishoop_b = 0;
    new_board->rook_b = 0;
    new_board->qeen_b = 0;
    new_board->king_b = 0;
    new_board->castle_rights = 0;

    int rank = 7;
    int file = 0;
    
    // Parse board position
    while (*fen && *fen != ' ') {
        if (*fen == '/') {
            rank--;
            file = 0;
        } else if (*fen >= '1' && *fen <= '8') {
            file += (*fen - '0');
        } else {
            uint64_t pos = 1ULL << (rank * 8 + file);
            switch (*fen) {
                case 'P': new_board->pawn_w |= pos; break;
                case 'N': new_board->knight_w |= pos; break;
                case 'B': new_board->bishoop_w |= pos; break;
                case 'R': new_board->rook_w |= pos; break;
                case 'Q': new_board->qeen_w |= pos; break;
                case 'K': new_board->king_w |= pos; break;
                case 'p': new_board->pawn_b |= pos; break;
                case 'n': new_board->knight_b |= pos; break;
                case 'b': new_board->bishoop_b |= pos; break;
                case 'r': new_board->rook_b |= pos; break;
                case 'q': new_board->qeen_b |= pos; break;
                case 'k': new_board->king_b |= pos; break;
            }
            file++;
        }
        fen++;
    }

    // Skip space
    if (*fen) fen++;

    // Active color
    new_board->white_move = (*fen == 'w');
    if (*fen) fen += 2; // Skip color and space

    // Castling rights
    while (*fen && *fen != ' ') {
        switch (*fen) {
            case 'K': new_board->castle_rights |= 8; break;
            case 'Q': new_board->castle_rights |= 4; break;
            case 'k': new_board->castle_rights |= 2; break;
            case 'q': new_board->castle_rights |= 1; break;
        }
        fen++;
    }

    return new_board;
}

int count_pieces(const struct board* b, bool white)
{
    // Position modifiers for each square from center to edge
    uint64_t center = 0x0000001818000000ULL;         // +0.5  (e4,d4,e5,d5)
    uint64_t inner = 0x00003C24243C0000ULL;          // +0.25 (ring around center)
    uint64_t neutral = 0x007E424242427E00ULL;        // +0    (middle ring)
    uint64_t outer = 0x0081818181818100ULL;          // -0.25 (next to edge)
    uint64_t edge = 0x7E00000000000000ULL |          // -0.5  (outer edge except corners)
                    0x000000000000007EULL |
                    0x0100000000000001ULL |
                    0x0200000000000002ULL |
                    0x0400000000000004ULL |
                    0x0800000000000008ULL;
    uint64_t corners = 0x8100000000000081ULL;        // -0.75 (corner squares)
    int count = 0;

    if(white)
    {

        count += __builtin_popcountll(b->pawn_w & center) * 1.5;      // 1 + 0.5
        count += __builtin_popcountll(b->pawn_w & inner) * 1.25;      // 1 + 0.25
        count += __builtin_popcountll(b->pawn_w & neutral) * 1;       // 1 + 0
        count += __builtin_popcountll(b->pawn_w & outer) * 0.75;      // 1 - 0.25
        count += __builtin_popcountll(b->pawn_w & edge) * 0.5;        // 1 - 0.5
        count += __builtin_popcountll(b->pawn_w & corners) * 0.25;    // 1 - 0.75

        count += __builtin_popcountll(b->knight_w & center) * 3.5;    // 3 + 0.5
        count += __builtin_popcountll(b->knight_w & inner) * 3.25;    // 3 + 0.25
        count += __builtin_popcountll(b->knight_w & neutral) * 3;     // 3 + 0
        count += __builtin_popcountll(b->knight_w & outer) * 2.75;    // 3 - 0.25
        count += __builtin_popcountll(b->knight_w & edge) * 2.5;      // 3 - 0.5
        count += __builtin_popcountll(b->knight_w & corners) * 2.25;  // 3 - 0.75

        count += __builtin_popcountll(b->bishoop_w & center) * 3.5;   // 3 + 0.5
        count += __builtin_popcountll(b->bishoop_w & inner) * 3.25;   // 3 + 0.25
        count += __builtin_popcountll(b->bishoop_w & neutral) * 3;    // 3 + 0
        count += __builtin_popcountll(b->bishoop_w & outer) * 2.75;   // 3 - 0.25
        count += __builtin_popcountll(b->bishoop_w & edge) * 2.5;     // 3 - 0.5
        count += __builtin_popcountll(b->bishoop_w & corners) * 2.25; // 3 - 0.75

        count += __builtin_popcountll(b->rook_w & center) * 5.5;      // 5 + 0.5
        count += __builtin_popcountll(b->rook_w & inner) * 5.25;      // 5 + 0.25
        count += __builtin_popcountll(b->rook_w & neutral) * 5;       // 5 + 0
        count += __builtin_popcountll(b->rook_w & outer) * 4.75;      // 5 - 0.25
        count += __builtin_popcountll(b->rook_w & edge) * 4.5;        // 5 - 0.5
        count += __builtin_popcountll(b->rook_w & corners) * 4.25;    // 5 - 0.75

        count += __builtin_popcountll(b->qeen_w & center) * 9.5;      // 9 + 0.5
        count += __builtin_popcountll(b->qeen_w & inner) * 9.25;      // 9 + 0.25
        count += __builtin_popcountll(b->qeen_w & neutral) * 9;       // 9 + 0
        count += __builtin_popcountll(b->qeen_w & outer) * 8.75;      // 9 - 0.25
        count += __builtin_popcountll(b->qeen_w & edge) * 8.5;        // 9 - 0.5
        count += __builtin_popcountll(b->qeen_w & corners) * 8.25;    // 9 - 0.75

        count += __builtin_popcountll(b->king_w & center) * 99.5;     // 100 - 0.5
        count += __builtin_popcountll(b->king_w & inner) * 99.75;     // 100 - 0.25
        count += __builtin_popcountll(b->king_w & neutral) * 100;     // 100 + 0
        count += __builtin_popcountll(b->king_w & outer) * 100.25;    // 100 + 0.25
        count += __builtin_popcountll(b->king_w & edge) * 100.5;      // 100 + 0.5
        count += __builtin_popcountll(b->king_w & corners) * 100.75;  // 100 + 0.75
    }
    else
    {
        count += __builtin_popcountll(b->pawn_b & center) * 1.5;      // 1 + 0.5
        count += __builtin_popcountll(b->pawn_b & inner) * 1.25;      // 1 + 0.25
        count += __builtin_popcountll(b->pawn_b & neutral) * 1;       // 1 + 0
        count += __builtin_popcountll(b->pawn_b & outer) * 0.75;      // 1 - 0.25
        count += __builtin_popcountll(b->pawn_b & edge) * 0.5;        // 1 - 0.5
        count += __builtin_popcountll(b->pawn_b & corners) * 0.25;    // 1 - 0.75

        count += __builtin_popcountll(b->knight_b & center) * 3.5;    // 3 + 0.5
        count += __builtin_popcountll(b->knight_b & inner) * 3.25;    // 3 + 0.25
        count += __builtin_popcountll(b->knight_b & neutral) * 3;     // 3 + 0
        count += __builtin_popcountll(b->knight_b & outer) * 2.75;    // 3 - 0.25
        count += __builtin_popcountll(b->knight_b & edge) * 2.5;      // 3 - 0.5
        count += __builtin_popcountll(b->knight_b & corners) * 2.25;  // 3 - 0.75

        count += __builtin_popcountll(b->bishoop_b & center) * 3.5;   // 3 + 0.5
        count += __builtin_popcountll(b->bishoop_b & inner) * 3.25;   // 3 + 0.25
        count += __builtin_popcountll(b->bishoop_b & neutral) * 3;    // 3 + 0
        count += __builtin_popcountll(b->bishoop_b & outer) * 2.75;   // 3 - 0.25
        count += __builtin_popcountll(b->bishoop_b & edge) * 2.5;     // 3 - 0.5
        count += __builtin_popcountll(b->bishoop_b & corners) * 2.25; // 3 - 0.75

        count += __builtin_popcountll(b->rook_b & center) * 5.5;      // 5 + 0.5
        count += __builtin_popcountll(b->rook_b & inner) * 5.25;      // 5 + 0.25
        count += __builtin_popcountll(b->rook_b & neutral) * 5;       // 5 + 0
        count += __builtin_popcountll(b->rook_b & outer) * 4.75;      // 5 - 0.25
        count += __builtin_popcountll(b->rook_b & edge) * 4.5;        // 5 - 0.5
        count += __builtin_popcountll(b->rook_b & corners) * 4.25;    // 5 - 0.75

        count += __builtin_popcountll(b->qeen_b & center) * 9.5;      // 9 + 0.5
        count += __builtin_popcountll(b->qeen_b & inner) * 9.25;      // 9 + 0.25
        count += __builtin_popcountll(b->qeen_b & neutral) * 9;       // 9 + 0
        count += __builtin_popcountll(b->qeen_b & outer) * 8.75;      // 9 - 0.25
        count += __builtin_popcountll(b->qeen_b & edge) * 8.5;        // 9 - 0.5
        count += __builtin_popcountll(b->qeen_b & corners) * 8.25;    // 9 - 0.75

        count += __builtin_popcountll(b->king_b & center) * 99.5;     // 100 - 0.5
        count += __builtin_popcountll(b->king_b & inner) * 99.75;     // 100 - 0.25
        count += __builtin_popcountll(b->king_b & neutral) * 100;     // 100 + 0
        count += __builtin_popcountll(b->king_b & outer) * 100.25;    // 100 + 0.25
        count += __builtin_popcountll(b->king_b & edge) * 100.5;      // 100 + 0.5
        count += __builtin_popcountll(b->king_b & corners) * 100.75;  // 100 + 0.75
    }
    return count;
}

bool is_in_check(const struct board* b, bool white);  // You'll need this helper function

bool w_mate(const struct board* b)
{
    if (!is_in_check(b, true)) {
        return false;
    }
    
    // Get all possible moves for white pieces
    struct move* moves = generate_legal_moves(b, true);
    if (!moves) {
        return true;  // No legal moves and in check = checkmate
    }
    
    int num_moves = get_move_count(moves);
    free(moves);
    return num_moves == 0;
}

bool b_mate(const struct board* b)
{
    if (!is_in_check(b, false)) {
        return false;
    }
    
    // Get all possible moves for black pieces
    struct move* moves = generate_legal_moves(b, false);
    if (!moves) {
        return true;  // No legal moves and in check = checkmate
    }
    
    int num_moves = get_move_count(moves);
    free(moves);
    return num_moves == 0;
}

bool draw(const struct board* b)
{
}

int evaluate_board(const struct board* b)
{
    if(w_mate(b)) return 1000000;
    if(b_mate(b)) return -1000000;
    if(draw(b)) return 0;
    int score = 0;
    score += count_pieces(b, true);
    score -= count_pieces(b, false);
    return score;
}


int main()
{
    printf("script started\n");
    char* test_fen = "bnrqknrb/pppppppp/8/8/8/8/PPPPPPPP/BNRQKNRB w - - 0 1";
    printf("Testing FEN: %s\n", test_fen);
    
    // Convert FEN to board
    const struct board* test_board = fen_to_board(test_fen);
    if (!test_board) {
        printf("Failed to create board from FEN\n");
        return 1;
    }

    // Convert board back to FEN
    const char* result_fen = board_to_fen(test_board);
    if (!result_fen) {
        printf("Failed to convert board back to FEN\n");
        free((void*)test_board);
        return 1;
    }

    // Compare original and resulting FEN strings
    printf("Result FEN: %s\n", result_fen);
    if (strcmp(test_fen, result_fen) == 0) {
        printf("FEN strings match!\n");
    } else {
        printf("FEN strings do not match!\n");
    }

    // Clean up
    free((void*)result_fen);
    free((void*)test_board);
}
