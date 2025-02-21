-- Example showing cache management features

require("ntr")

local ntrFile = "example.ntr"

-- Parse the file
print("Parsing file...")
if NTRParser.ParseFile(ntrFile) then
    -- Get some values
    print(NTRParser.GetValue(ntrFile, "messages.success"))
    
    -- Unload specific file from cache
    print("\nUnloading file from cache...")
    if NTRParser.UnloadFile(ntrFile) then
        print("File unloaded successfully")
    end
    
    -- Try to get value after unloading (will return nil)
    print("\nTrying to get value after unload:")
    local value = NTRParser.GetValue(ntrFile, "messages.success")
    print("Value:", value)
    
    -- Parse file again
    print("\nParsing file again...")
    NTRParser.ParseFile(ntrFile)
    
    -- Clear entire cache
    print("\nClearing entire cache...")
    NTRParser.ClearCache()
    
    -- Try to get value after cache clear (will return nil)
    print("\nTrying to get value after cache clear:")
    local value = NTRParser.GetValue(ntrFile, "messages.success")
    print("Value:", value)
end