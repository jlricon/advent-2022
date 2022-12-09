with open("../../data/day8.txt", "r") as f:
    lines = f.readlines()
    # Values represent heights
    heights = [[int(j) for j in i.strip("\n")] for i in lines]


# Find the number of visible trees
num_visible_trees = 0
num_rows = len(heights)
num_cols = len(heights[0])

# Iterate over each tree in the grid
for row in range(1, num_rows - 1):
    for col in range(1, num_cols - 1):
        # Check if the current tree is the tallest in its row and column
        if (
            all(heights[row][c] < heights[row][col] for c in range(0, col))
            or all(heights[r][col] < heights[row][col] for r in range(0, row))
            or all(heights[row][c] < heights[row][col] for c in range(col + 1, num_cols))
            or all(heights[r][col] < heights[row][col] for r in range(row + 1, num_rows))
        ):
            num_visible_trees += 1
num_visible_trees += num_cols + (num_rows - 1) + (num_cols - 1) + (num_rows - 2)
print(f"{num_visible_trees} trees are visible from outside the grid.")

max_scenic_score = 0

# Iterate over each tree in the grid
for row in range(1, num_rows - 1):
    for col in range(1, num_cols - 1):
        this_tree = heights[row][col]
        # For each tree, calculate scenic score in each direction
        # Trees to the right
        right_scenic = 0
        for c in range(col + 1, num_cols):
            # if heights[row][c] <= this_tree:
            right_scenic += 1
            if heights[row][c] >= this_tree:
                break
        # Trees to the left
        left_scenic = 0
        for c in range(col - 1, -1, -1):
            # if heights[row][c] <= this_tree:
            left_scenic += 1
            if heights[row][c] >= this_tree:
                break
        # Trees above
        up_scenic = 0
        for r in range(row - 1, -1, -1):
            # if heights[r][col] <= this_tree:
            up_scenic += 1
            if heights[r][col] >= this_tree:
                break
        # Trees below
        down_scenic = 0
        for r in range(row + 1, num_rows):
            # if heights[r][col] <= this_tree:
            down_scenic += 1
            if heights[r][col] >= this_tree:
                break
        scenic_score = right_scenic * left_scenic * up_scenic * down_scenic
        print(
            f"({row}, {col}): {scenic_score} ({up_scenic}, {left_scenic}, {right_scenic}, {down_scenic})"
        )
        max_scenic_score = max(max_scenic_score, scenic_score)


print(f"The highest scenic score is {max_scenic_score}.")
