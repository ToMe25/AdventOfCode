import InputReader


def main():
    ingredients = {}
    allergens = {}
    for line in InputReader.readInputFileLines(21):
        ingr = []
        for ingredient in line[:line.index('(') - 1].split(" "):
            ingr.append(ingredient)
            if ingredient in ingredients:
                ingredients[ingredient] += 1
            else:
                ingredients[ingredient] = 1

        for allergen in line[line.index('(') + 10:line.index(')')].split(", "):
            if not allergen in allergens:
                allergens[allergen] = ingr.copy()
            else:
                allergens[allergen] = list(set(allergens[allergen]) & set(ingr))

    allergenFree = 0
    for ingredient in ingredients.keys():
        if not any(ingredient in ingredients for ingredients in allergens.values()):
            allergenFree += ingredients[ingredient]

    print(f"All the ingredients that can't be allergens appear a combined total of {allergenFree} times.")

    while any(len(ingr) > 1 for ingr in allergens.values()):
        for ingreds in allergens.values():
            if len(ingreds) == 1:
                for ing in allergens.values():
                    if ing != ingreds and ingreds[0] in ing:
                        ing.remove(ingreds[0])

    dangerList = ""
    for allergen in sorted(allergens.keys()):
        dangerList += "," + allergens[allergen][0]
    dangerList = dangerList[1:]

    print(f"The danger list is \"{dangerList}\".")


if __name__ == '__main__':
    main()
