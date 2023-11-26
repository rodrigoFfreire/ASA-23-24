#ifndef __SOLVER_H__
#define __SOLVER_H__

#include <vector>

struct Piece {
    int _x;
    int _y;
    int _price;
};


int calculateBestValue(int piece_x, int piece_y, int x, int y, 
    int sheet_x, std::vector<int>* matrix);

void getMinimumPiece(const std::vector<Piece> &order, int (&result)[3], int sheet_x);

int solveBestValue(const std::vector<Piece> &order, int amount, int sheet_x, int sheet_y);

#endif