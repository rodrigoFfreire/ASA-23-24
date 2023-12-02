#ifndef __SOLVER_H__
#define __SOLVER_H__

#include <vector>

uint computeComboX(vector<uint> &matrix, uint x, uint line);

uint computeComboY(vector<uint> &matrix, uint y, uint column, uint matrixX);

uint solveBestValue(vector<uint> &matrix, uint sheetX, uint sheetY,
					uint sIndex);

#endif