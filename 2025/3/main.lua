if #arg < 1 then
	print("Usage: main.lua path/to/input_file")
	os.exit(0, true)
end

local REQ_BATS = 12

-- Read file
local line_num = 0
local joltage_total = 0
for line in io.lines(arg[1]) do
	local line_len = string.len(line)

	local batteries = {
		[0] = {
			idx = 0,
			value = '0'
		}
	}

	for bat = 1, REQ_BATS do
		-- Next battery must be at least 1 digit to right of last battery.
		p = batteries[bat - 1].idx + 1
		--print("Bat", bat, "starting at", p)
		battery = {
			idx = p,
			value = string.sub(line, p, p)
		}

		-- For remainder of batteries, find highest
		local bats_remaining = REQ_BATS - bat
		for i = p, line_len - bats_remaining do
			-- If this battery has a strictly higher value, replace.
			-- This ensures left-most value is kept if same.
			b = string.sub(line, i, i)
			if b > battery.value then
				battery.value = b
				battery.idx = i
			end
		end

		table.insert(batteries, battery)
	end

	-- Debug print
	bat_digits = ''
	for _,b in ipairs(batteries) do
		bat_digits = bat_digits .. b.value
	end
	--print("Digits:", bat_digits)

	local partial_sum = 0
	for i = 1, REQ_BATS do
		partial_sum = partial_sum + 10 ^ (REQ_BATS - i) * tonumber(batteries[i].value)
	end
	--print("Bat value:", partial_sum)
	joltage_total = joltage_total + partial_sum

	line_num = line_num + 1
end

print(string.format("Total joltage: %d", joltage_total))
