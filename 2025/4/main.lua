if #arg < 1 then
	print("Usage: main.lua path/to/input_file")
	os.exit(0, true)
end

-- Read file
local line_num = 0
local total_rolls = 0
grid = {}
for line in io.lines(arg[1]) do
    for c = 1, string.len(line) do
        if string.sub(line, c, c) == '@' then
            table.insert(grid, true)
        else
            table.insert(grid, false)
        end

        total_rolls = total_rolls + 1
    end

	line_num = line_num + 1
end

num_rows = line_num
num_cols = math.floor(#grid / line_num)

function adjacent_rolls(i)
    local count = 0
    -- Check previous row if it exists
    if i > num_cols then
        -- Check UL if not in first col
        if i % num_cols ~= 1 then
            if grid[i - num_cols - 1] then
                count = count + 1
            end
        end

        -- UC guaranteed exists
        if grid[i - num_cols] then
            count = count + 1
        end

        -- Check UR if not in last col
        if i % num_cols ~= 0 then
            if grid[i - num_cols + 1] then
                count = count + 1
            end
        end
    end

    -- Check L if not in first column
    if i % num_cols ~= 1 then
        if grid[i - 1] then
            count = count + 1
        end
    end

    -- Check R if not in last column
    if i % num_cols ~= 0 then
        if grid[i + 1] then
            count = count + 1
        end
    end

    -- Check next row if not in last row
    if i <= (#grid - num_cols) then
        -- Check BL if not in first col
        if i % num_cols ~= 1 then
            if grid[i + num_cols - 1] then
                count = count + 1
            end
        end

        -- BC guaranteed exists
        if grid[i + num_cols] then
            count = count + 1
        end

        -- Check BR if not in last col
        if i % num_cols ~= 0 then
            if grid[i + num_cols + 1] then
                count = count + 1
            end
        end
    end

    return count
end

function remove_accessible()
    local accessible_rolls = {}
    for i = 1,#grid do
        if grid[i] then
            if adjacent_rolls(i) < 4 then
                table.insert(accessible_rolls, i)
            end
        end
    end

    -- Remove rolls
    for i = 1, #accessible_rolls do
        grid[accessible_rolls[i]] = false
    end

    return #accessible_rolls
end

local total_removed = 0
local rolls_removed = remove_accessible()
while rolls_removed > 0 do
    total_removed  = total_removed + rolls_removed
    rolls_removed = remove_accessible()
end
print("Accessible rolls:", total_removed)
