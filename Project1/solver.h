#ifndef __SOLVER_H__
#define __SOLVER_H__

uint computeComboX(std::vector<uint> &matrix, uint x, uint line);

uint computeComboY(std::vector<uint> &matrix, uint y, uint column, uint matrixX);

uint solveBestValue(std::vector<uint> &matrix, uint sheetX, uint sheetY,
                    uint sIndex);

#endif