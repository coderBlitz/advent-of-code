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
