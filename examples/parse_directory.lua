--- Example for parsing an directory

require("ntr")

-- Path to your NTR directory
local ntrDirectory = "full/path/to/your/directory"

-- Parse the NTR directory
NTRParser.ParseDirectory(ntrDirectory)

-- Check if file is cached
PrintTable(NTRParser.GetCachedFiles())