if #arg < 1 then
	print("Usage: main.lua path/to/input_file")
	os.exit(0, true)
end

-- Read file
local line_num = 0
local joltage_total = 0
for line in io.lines(arg[1]) do
	local line_len = string.len(line)

	-- Start with left-most and right-most for choices
	local left_digit = string.sub(line, 1,1)
	local left_idx = 1
	local right_digit = string.sub(line, -1, -1)
	local right_idx = line_len

	-- From left to right, find highest.
	for i = 2, line_len - 1 do
		c = string.sub(line, i, i)
		if c > left_digit then
			left_digit = c
			left_idx = i
		end
	end

	-- From right to left, find highest.
	--for i = 2, line_len, -1 do
	for i = line_len - 1, left_idx + 1, -1 do
		c = string.sub(line, i, i)
		if c > right_digit then
			right_digit = c
			right_idx = i
		end
	end

	-- Convert digits to number and add to total
	local val = tonumber(left_digit) * 10 + tonumber(right_digit)
	--print("Line", line_num, "highest:", val)

	joltage_total = joltage_total + val

	line_num = line_num + 1
end

print("Total joltage:", joltage_total)
