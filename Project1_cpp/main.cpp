#include <iostream>
#include "solver.h"
using namespace std;


int main() {
    int sheet_x, sheet_y;
    int pieceAmount;
    int bestValue;

    cin >> sheet_x >> sheet_y;
    cin >> pieceAmount;

    if (pieceAmount <= 0) bestValue = 0;
    else {
        vector<Piece> order = vector<Piece> (pieceAmount);

        for (int i = 0; i < pieceAmount; i++) {
            int x, y, price;
            cin >> x >> y >> price;

            order.push_back({x, y, price});
            cin.ignore();
        }
        if (sheet_y > sheet_x) swap(sheet_x, sheet_y); // Force sheet to be in horizontal position

        bestValue = solveBestValue(order, pieceAmount, sheet_x, sheet_y);
    }

    cout << bestValue << endl;
    return 0;
}