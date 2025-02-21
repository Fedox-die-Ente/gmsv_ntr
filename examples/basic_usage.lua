-- Basic usage example of the NTR Parser

-- First, require the module
require("ntr")

-- Path to your NTR file
local ntrFile = "example.ntr"

-- Parse the NTR file
if NTRParser.ParseFile(ntrFile) then
    -- Get a simple value
    local hello = NTRParser.GetValue(ntrFile, "hello")
    print("Simple value:", hello) -- Output: Hello World

    -- Get a nested value
    local errorMsg = NTRParser.GetValue(ntrFile, "errors.404")
    print("Nested value:", errorMsg) -- Output: Not found

    -- Check if a key exists
    local exists = NTRParser.KeyExists(ntrFile, "messages.success")
    print("Key exists:", exists) -- Output: true

    -- Get all available keys
    local keys = NTRParser.GetAllKeys(ntrFile)
    print("\nAll available keys:")
    for _, key in ipairs(keys) do
        print(key)
    end
end