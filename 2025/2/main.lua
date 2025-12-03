if #arg < 1 then
	print("Usage: main.lua path/to/input_file")
	os.exit(0, true)
end

function is_valid_id(
	---@type string
	num
)
	local len = string.len(num)
	local half_len = len // 2

	-- Check all possible sequence lengths
	for seq_size = 1, half_len do
		local valid = false

		-- If length divisible by sequence length, check.
		-- Else try next size.
		if len % seq_size == 0 then
			seq = string.sub(num, 1, seq_size)
			n = len // seq_size

			-- If loop finishes, invalid
			for i = 1, n-1 do
				seq2 = string.sub(num, seq_size * i + 1, seq_size * (i + 1))

				-- Mismatch
				if seq ~= seq2 then
					valid = true
					break
				end
			end

			-- If number passes loop above, then definitely invalid.
			if valid == false then
				return false
			end
		end
	end

	return true
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
				invalid_ids = invalid_ids + 1
				invalid_total = invalid_total + id
			end
		end
	end

	line_num = line_num + 1
end

print("Invalid ID count:", invalid_ids)
print("Invalid ID total:", invalid_total)
