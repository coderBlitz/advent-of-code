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
	local num = tonumber(val, 10)
	local num_mod = num % DIAL_SIZE
	local rotations = num // DIAL_SIZE

	-- Do math based on number
	--print(pos, num_mod, rotations)
	if dir == 'L' then
		pos = pos - num_mod
		if pos == -num_mod then
			-- If already at 0, not a "pass".
			pos = pos + DIAL_SIZE
		elseif pos < 0 then
			pos = pos + DIAL_SIZE
			zero_count = zero_count + 1
		elseif pos == 0 then
			zero_count = zero_count + 1
		end
	else
		pos = pos + num_mod

		if pos >= DIAL_SIZE then
			pos = pos % DIAL_SIZE
			zero_count = zero_count + 1
		end
	end

	zero_count = zero_count + rotations
end

print("Zero count is", zero_count)
