#include <stdlib.h>
#include <vector>
#include "solver.h"

using namespace std;


uint computeComboX(vector<uint> &matrix, uint x, uint line) {
	if (x <= 1) return 0;

	uint bestValue = 0;
	for (uint ix = 1; ix <= x >> 1; ix++)
		bestValue = max(bestValue, matrix[line + ix] + matrix[line + x - ix]);

	return bestValue;
}

uint computeComboY(vector<uint> &matrix, uint y, uint column, uint matrixX) {
	if (y <= 1) return 0;

	uint bestValue = 0;
	for (uint iy = 1; iy <= y >> 1; iy++)
		bestValue = max(bestValue, matrix[column + iy * matrixX] +
			matrix[column + ((y - iy) * matrixX)]);

	return bestValue;
}

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

			bestValue = max(bestValue, computeComboX(matrix, x, line));
			bestValue = max(bestValue, computeComboY(matrix, y, x, matrixX));

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