if #arg < 1 then
	print("Usage: main.lua path/to/input_file")
	os.exit(0, true)
end

-- Read file
local line_num = 0
for line in io.lines(arg[1]) do
	for id_pair in string.gmatch(line, "%d+-%d+") do
		print(id_pair)
	end
	line_num = line_num + 1
end
