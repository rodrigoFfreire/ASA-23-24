from pulp import LpProblem, LpMaximize, LpVariable, GLPK_CMD, value


def main():
    result = 0
    toyAmount, packAmount, maxToys = [int(i) for i in input().split()]

    model = LpProblem(name="ASA_Proj3", sense=LpMaximize)
    solver = GLPK_CMD(msg=False)

    toys = {}               # Stores data for each toy
    packs = {}              # Stores data for each pack
    usedToysInPacks = {}    # Stores for each toy, which packs are using that toy

    sumToysVal = 0
    sumPackVal = 0
    sumToys = 0
    sumPack = 0

    if toyAmount > 0 and packAmount >= 0 and maxToys > 0:
        for t in range(toyAmount):
            toy_data = tuple(map(int, input().split()))
            toy_data += (LpVariable(name=f"T{t+1}", lowBound=0, upBound=toy_data[1], cat="Integer"), )
            toys[f"T{t+1}"] = toy_data

            sumToysVal += toy_data[0] * toy_data[2]
            sumToys += toy_data[2]

            usedToysInPacks[f"T{t+1}"] = 0

        for p in range(packAmount):
            package_data = tuple(map(int, input().split()))
            package_data += (LpVariable(name=f"P{p+1}", 
                                        lowBound=0, 
                                        upBound=min(toys[f"T{package_data[0]}"][1],
                                                    toys[f"T{package_data[1]}"][1],
                                                    toys[f"T{package_data[2]}"][1]),
                                        cat="Integer"), )
            packs[f"p_{p+1}"] = package_data

            sumPackVal += package_data[3] * package_data[4]
            sumPack += package_data[4]

            usedToysInPacks[f"T{package_data[0]}"] += package_data[4]
            usedToysInPacks[f"T{package_data[1]}"] += package_data[4]
            usedToysInPacks[f"T{package_data[2]}"] += package_data[4]

        # OBJECTIVE FUNCTION
        model += sumToysVal + sumPackVal

        # CONSTRAINT: Sum of toys + Sum of toys used in packs must not exceed maxToys
        model += sumToys + 3 * sumPack <= maxToys

        # CONSTRAINT: Used toys must not exceed the production for said toy
        for toy_key, toy_data in toys.items():
            model += toy_data[2] + usedToysInPacks[toy_key] <= toy_data[1]

        model.solve(solver)
        result = value(model.objective)

    print(result)

if __name__ == "__main__":
    main()
