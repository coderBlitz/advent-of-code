if #arg < 1 then
	print("Usage: main.lua path/to/input_file")
	os.exit(0, true)
end

-- Read file
local line_num = 0
local collecting_ranges = true
local ranges = {}
local available = {}
for line in io.lines(arg[1]) do
    if string.len(line) == 0 then
        collecting_ranges = false
    end

    if collecting_ranges then
        id_pair = string.match(line, "%d+-%d+")
        hyphen_idx = string.find(id_pair, '-')
        start_id = string.sub(id_pair, 1, hyphen_idx - 1)
        end_id = string.sub(id_pair, hyphen_idx + 1, -1)

        table.insert(ranges, {tonumber(start_id), tonumber(end_id)})
    else
        id = string.match(line, "%d+")
        if id ~= nil then
            table.insert(available, tonumber(id))
        end
    end
end

local fresh_avail = 0
for _, av in ipairs(available) do
    for _,r in ipairs(ranges) do
        if av >= r[1] and av <= r[2] then
            fresh_avail = fresh_avail + 1
            break
        end
    end
end

print("Fresh & avail =", fresh_avail)

-- Part 2
function range_sort(a,b)
    return a[1] < b[1]
end
table.sort(ranges, range_sort)

-- Simplify ranges
local combo_ranges = {}
local total_fresh = 0
local cur_start = ranges[1][1]
local cur_end = ranges[1][2]
for _,r in ipairs(ranges) do
    if r[1] >= cur_start and r[1] <= cur_end then
        if r[2] > cur_end then
            cur_end = r[2]
        end
    elseif r[1] > cur_end then
        table.insert(combo_ranges, {cur_start, cur_end})
        cur_start = r[1]
        cur_end = r[2]
    end
end
table.insert(combo_ranges, {cur_start, cur_end})

for _,r in ipairs(combo_ranges) do
    total_fresh = total_fresh + (r[2] - r[1]) + 1
end
print(string.format("Total fresh = %d", total_fresh))
