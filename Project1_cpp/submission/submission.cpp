#include <iostream>
#include <stdlib.h>
#include <vector>


struct Piece {
    int _x;
    int _y;
    int _price;
};


int calculateBestValue(int piece_x, int piece_y, int x, int y, 
    int sheet_x, std::vector<int> &matrix)
{
    int yk = y * sheet_x;
    int pysx = piece_y * sheet_x + x;

    int v_cut_value = matrix[yk + piece_x] + matrix[yk - piece_x + x];
    int h_cut_value = matrix[pysx] + matrix[yk - pysx];

    return std::max(h_cut_value, v_cut_value);
}


void getMinimumPiece(const std::vector<Piece> &order, int (&result)[3], int sheet_x) {
    int min_x = order[0]._x;
    int min_y = order[0]._y;
    int min_index = min_y * sheet_x + min_x;
    int index = 0;

    for (const Piece &piece: order) {
        std::cout << "AA" << std::endl;
        index = piece._y * sheet_x + piece._x;
        if (index < min_index) {
            min_x = piece._x;
            min_y = piece._y;
            min_index = index;
        }
    }
    result[0] = min_x, result[1] = min_y, result[2] = min_index;
    return;
}


int solveBestValue(const std::vector<Piece> &order, int sheet_x, int sheet_y) {
    int minPiece[3];
    int matrix_x = sheet_x + 1;
    getMinimumPiece(order, minPiece, matrix_x);

    std::vector<int> maxValue = std::vector<int> (matrix_x * (sheet_y + 1));
    int x = minPiece[0];
    int y = minPiece[1];
    int currentIndex = minPiece[2] - 1;
    int bestValue = 0;
    std::cout << x << y << currentIndex << std::endl;

    while (y <= sheet_y) {
        while (x <= sheet_x) {
            for (const Piece &piece: order) {
                int piece_x = piece._x;
                int piece_y = piece._y;
                int piece_price = piece._price;

                if (!(piece_x <= x && piece_y <= y))
                    continue;
                else if (piece_x == x && piece_y == y) {
                    bestValue = std::max(bestValue, piece_price);
                } else {
                    bestValue = std::max(
                        bestValue,
                        calculateBestValue(piece_x, piece_y, x, y, matrix_x, maxValue)
                    );
                    if (piece_x <= y && piece_y <= x) {
                        bestValue = std::max(
                            bestValue,
                            calculateBestValue(piece_y, piece_x, x, y, matrix_x, maxValue)
                        );
                    }
                }
            }
            currentIndex++;
            maxValue[currentIndex] = bestValue;
            if (x > y && x < sheet_y)
                maxValue[x * matrix_x + y] = bestValue;
            x++;
        }
        y++;
        x = y;
        currentIndex += y;
    }
    return maxValue[sheet_y * matrix_x + sheet_x];
}


int main() {
    int sheet_x, sheet_y;
    int pieceAmount;
    int bestValue;

    std::cin >> sheet_x >> sheet_y;
    std::cin >> pieceAmount;

    if (pieceAmount <= 0) bestValue = 0;
    else {
        std::vector<Piece> order(pieceAmount);

        for (int i = 0; i < pieceAmount; i++) {
            int x, y, price;
            std::cin >> x >> y >> price;

            order.push_back({x, y, price});
        }
        if (sheet_y > sheet_x) std::swap(sheet_x, sheet_y); // Force sheet to be in horizontal position

        bestValue = solveBestValue(order, sheet_x, sheet_y);
    }

    std::cout << bestValue << std::endl;
    return 0;
}