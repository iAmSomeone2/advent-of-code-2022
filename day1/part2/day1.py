#!/usr/bin/env python3

def make_calorie_groups(calorie_data: list[str]) -> list[list[int]]:
    groups = []
    group = []
    for line in calorie_data:
        line = line.strip()
        if line == "":
            groups.append(group)
            group = []
            continue
        
        value = int(line)
        group.append(value)

    # Append the last group before returning
    groups.append(group)

    return groups

def total_calories(calorie_groups: list[list[int]]) -> list[int]:
    group_totals = []

    for group in calorie_groups:
        total = 0
        for value in group:
            total += value
        group_totals.append(total)

    return group_totals

def get_max(group_totals: list[int]) -> int:
    max = 0

    for calories in group_totals:
        if calories > max:
            max = calories
    
    return max

def top_three_total(group_totals: list[int]) -> int:
    group_totals.sort(reverse=True)
    top_three = group_totals[:3]

    total = 0
    for value in top_three:
        total += value

    return total



if __name__ == "__main__":
    # Load input data
    with open("input.txt", mode="r", encoding="utf-8") as input_file:
        input_data = input_file.readlines()
    
    calorie_groups = make_calorie_groups(input_data)
    group_totals = total_calories(calorie_groups)

    print(top_three_total(group_totals))
