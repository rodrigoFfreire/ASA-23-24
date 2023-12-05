#include <iostream>
#include <stdlib.h>
#include <vector>

using namespace std;

uint solveBestValue(vector<uint> &matrix, uint sheetX, uint sheetY,
                    uint sIndex) {
    uint matrixX = sheetX + 1;
    uint x = sIndex % matrixX;
    uint y = sIndex / matrixX;

    while (y <= sheetY) {
        uint line = y * matrixX;
        while (x < sheetX) {
            sIndex++;
            x++;
            uint bestValue = matrix[sIndex];
            for (uint ix = 1; ix <= x >> 1; ix++)
                bestValue = max(bestValue, matrix[line + ix] + matrix[line + x - ix]);

            for (uint iy = 1; iy <= y >> 1; iy++)
                bestValue = max(bestValue, matrix[matrixX * iy + x] + matrix[(y - iy) * matrixX + x]);

            matrix[sIndex] = bestValue;
            if (x > y && x <= sheetY)
                matrix[x * matrixX + y] = bestValue;
        }
        x = y;
        y++;
        sIndex += y;
    }

    return matrix[sheetY * matrixX + sheetX];
}


int main() {
    uint X, Y, N; // Sheet X, Sheet Y, Number of Pieces
    uint result = 0;

    scanf("%u %u", &X, &Y);
    scanf("%u", &N);

    if (N > 0) {
        if (Y > X)
            swap(X, Y);

        uint startIndex = 0xFFFFFFFF;
        uint matrixX = X + 1;
        uint a, b, v; // piece X, piece Y, piece Price

        vector<uint> table(matrixX * (Y + 1), 0);
        for (; N > 0; N--) {
            scanf("%u %u %u", &a, &b, &v);
            if (b > a)
                swap(a, b);

            uint index = b * matrixX + a;
            table[index] = max(table[index], v);
            if (a <= Y)
                table[a * matrixX + b] = max(table[a * matrixX + b], v);
            startIndex = min(startIndex, index);
        }
        startIndex++;
        if (startIndex % matrixX == 0)
            startIndex += startIndex / (Y + 1);

        result = solveBestValue(table, X, Y, startIndex - 1);
    }
    cout << result << endl;
    return 0;
}
