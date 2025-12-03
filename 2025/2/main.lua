if #arg < 1 then
	print("Usage: main.lua path/to/input_file")
	os.exit(0, true)
end

function is_valid_id(
	---@type string
	num
)
	-- Strings must be even in length to be candidate.
	if string.len(num) % 2 == 0 then
		local half_len = string.len(num) / 2
		local valid = 1

		-- For half id length
		for i = 1, half_len do
			-- If corresponding chars don't match, invalid.
			local c1 = string.sub(num, i, i)
			local c2 = string.sub(num, i + half_len, i + half_len)

			if c1 ~= c2 then
				return true
			end
		end
	else
		return true
	end

	return false
end

-- Read file
local line_num = 0
local invalid_ids = 0
local invalid_total = 0
for line in io.lines(arg[1]) do
	for id_pair in string.gmatch(line, "%d+-%d+") do
		hyphen_idx = string.find(id_pair, '-')
		start_id = string.sub(id_pair, 1, hyphen_idx - 1)
		end_id = string.sub(id_pair, hyphen_idx + 1, -1)
		--print(start_id, end_id)
		start_v = tonumber(start_id, 10)
		end_v = tonumber(end_id, 10)

		for id = start_v, end_v do
			-- Check start ID
			if not is_valid_id(tostring(id)) then
				--print(id, "is invalid")
				invalid_ids = invalid_ids + 1
				invalid_total = invalid_total + id
			end
		end
	end

	line_num = line_num + 1
end

print("Invalid ID count:", invalid_ids)
print("Invalid ID total:", invalid_total)
