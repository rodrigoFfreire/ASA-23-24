from pulp import LpProblem, LpMaximize, LpVariable, lpSum, GLPK_CMD, value


def solve(toys, packages, toyAmount, packageAmount, maxToys):
    model = LpProblem(name="ASA_Proj3", sense=LpMaximize)
    solver = GLPK_CMD(msg=False)

    # Objective Function
    model += lpSum(toys[i][0] * toys[i][2] for i in range(toyAmount)) + \
                lpSum(packages[i][3] * packages[i][4] for i in range(packageAmount))
    
    # CONSTRAINT: Sum of toys + Sum of toys used in packages must not exceed maxToys
    model += lpSum(toys[i][2] for i in range(toyAmount)) + 3 * lpSum(packages[i][4] for i in range(packageAmount)) <= maxToys
    # CONSTRAINT: Used toys must not exceed the production for said toy
    for i in range(toyAmount):
        model += toys[i][2] + lpSum([info[4] for info in packages if i + 1 in info[:3]]) <= toys[i][1]

    model.solve(solver)
    return value(model.objective)

def main():
    result = 0
    toyAmount, packageAmount, maxToys = [int(i) for i in input().split()]

    if toyAmount > 0 and packageAmount >= 0 and maxToys > 0:
        toys = tuple(
            tuple(map(int, input().split())) + (LpVariable(name=f"t_{t+1}", lowBound=0, cat="Integer"), )
            for t in range(toyAmount))
        packages = tuple(
            tuple(map(int, input().split())) + (LpVariable(name=f"p_{p+1}", lowBound=0, cat="Integer"), )
            for p in range(packageAmount))
        result = solve(toys, packages, toyAmount, packageAmount, maxToys)

    print(result)

if __name__ == "__main__":
    main()

