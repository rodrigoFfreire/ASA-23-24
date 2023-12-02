#include <iostream>
#include <stdlib.h>
#include <vector>
#include "solver.h"

using namespace std;


int main() {
    uint sheetX, sheetY, pieceAmount;
    uint result = 0;

    scanf("%u %u", &sheetX, &sheetY);
    scanf("%u", &pieceAmount);

    if (pieceAmount > 0) {
        if (sheetY > sheetX)
            swap(sheetX, sheetY);

        uint startIndex = 0xFFFFFFFF;
        uint matrixX = sheetX + 1;
        uint maxIndex = sheetY * matrixX + sheetX;
        uint pieceX, pieceY, piecePrice;

        vector<uint> valueMatrix(matrixX * (sheetY + 1), 0);
        for (; pieceAmount > 0; pieceAmount--) {
            scanf("%u %u %u", &pieceX, &pieceY, &piecePrice);
            if (pieceY > pieceX)
                swap(pieceX, pieceY);

            uint index = pieceY * matrixX + pieceX;
            if (pieceX > sheetX || index > maxIndex) continue;

            valueMatrix[index] = max(valueMatrix[index], piecePrice);
            if (pieceX <= sheetY)
                valueMatrix[pieceX * matrixX + pieceY] =
                    max(valueMatrix[pieceX * matrixX + pieceY], piecePrice);
            startIndex = min(startIndex, index);
        }
        startIndex++;
        if (startIndex % matrixX == 0)
            startIndex += startIndex / (sheetY + 1);

        result = solveBestValue(valueMatrix, sheetX, sheetY, startIndex);
    }
    cout << result << endl;
    return 0;
}
