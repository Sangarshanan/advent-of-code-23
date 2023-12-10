grid = open("input").read().splitlines()

adj_number_index1 = set()
part2_total = 0

for idx, row in enumerate(grid):
    for idy, ch in enumerate(row):
        adj_number_index2 = set()
        # Tis a Symbol
        if not (ch.isdigit() or ch == "."):
            # Check adjacent and diagonal cell
            for sub_row in range(idx - 1, idx + 2):
                for sub_col in range(idy - 1, idy + 2):
                    if (
                        (sub_row < 0)  # Out of Bounds up
                        or (sub_col < 0)  # Out of Bounds left
                        or (sub_row >= len(grid))  # Out of Bounds down
                        or (sub_col >= len(grid[sub_row]))  # Out of Bounds right
                        or not grid[sub_row][sub_col].isdigit()  # Not a digit
                    ):
                        # not a valid index
                        continue
                    # Digit and neighbor is also digit
                    while grid[sub_row][sub_col - 1].isdigit():
                        # keep going left
                        sub_col -= 1
                    # Index of integer neighbors
                    adj_number_index1.add((sub_row, sub_col))
                    adj_number_index2.add((sub_row, sub_col))

        # Part 2
        if ch != "*":
            continue

        if len(adj_number_index2) != 2:
            continue

        ns = []

        for cr, cc in adj_number_index2:
            s = ""
            while cc < len(grid[cr]) and grid[cr][cc].isdigit():
                s += grid[cr][cc]
                cc += 1
            ns.append(int(s))
        part2_total += ns[0] * ns[1]

final_numbers = []

for r, c in adj_number_index1:
    s = ""
    while c < len(grid[r]) and grid[r][c].isdigit():
        s += grid[r][c]
        c += 1
    final_numbers.append(int(s))

print(sum(final_numbers))
print(part2_total)

"""
528799
84907174
"""
