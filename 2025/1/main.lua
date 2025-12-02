if #arg < 1 then
	print("Usage: main.lua path/to/input_file")
	os.exit(0, true)
end

-- Read file
local pos = 50
local DIAL_SIZE = 100
local zero_count = 0
for line in io.lines(arg[1]) do
	local _, _, dir, val = string.find(line, "(%a)(%d+)")

	-- Convert number and normalize to dial size
	local num = tonumber(val, 10) % DIAL_SIZE

	-- Do math based on number
	if dir == 'L' then
		pos = pos - num
		if pos < 0 then
			pos = pos + DIAL_SIZE
		end
	else
		pos = (pos + num) % DIAL_SIZE
	end

	if pos == 0 then
		zero_count = zero_count + 1
	end
end

print("Zero count is", zero_count)
