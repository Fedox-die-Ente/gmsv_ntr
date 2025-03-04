extern crate core;
#[macro_use]
extern crate rglua;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;
use std::ffi::CString;
use rglua::lua::LuaState;
use rglua::prelude::*;
use crate::ntr_parser::{parse_ntr_file, NtrData};

mod ntr_parser;

lazy_static! {
    static ref PARSED_FILES: Mutex<HashMap<String, NtrData>> = Mutex::new(HashMap::new());
}

#[gmod_open]
unsafe fn open(l: LuaState) -> i32 {

    luaL_newmetatable(l, cstr!("NTRParser"));
    lua_pushvalue(l, -1);
    lua_setfield(l, -2, cstr!("__index"));
    lua_pushcfunction(l, parse_ntr_file_lua);
    lua_setfield(l, -2, cstr!("ParseFile"));
    lua_pushcfunction(l, get_ntr_value_lua);
    lua_setfield(l, -2, cstr!("GetValue"));

    lua_newtable(l);

    lua_pushcfunction(l, parse_ntr_file_lua);
    lua_setfield(l, -2, cstr!("ParseFile"));

    lua_pushcfunction(l, get_ntr_value_lua);
    lua_setfield(l, -2, cstr!("GetValue"));

    lua_pushcfunction(l, ntr_key_exists);
    lua_setfield(l, -2, cstr!("KeyExists"));

    lua_pushcfunction(l, get_all_keys_lua);
    lua_setfield(l, -2, cstr!("GetAllKeys"));

    lua_pushcfunction(l, clear_cache_lua);
    lua_setfield(l, -2, cstr!("ClearCache"));

    lua_pushcfunction(l, unload_file_lua);
    lua_setfield(l, -2, cstr!("UnloadFile"));

    lua_pushcfunction(l, get_value_from_cache_without_file);
    lua_setfield(l, -2, cstr!("GetValueFromCache"));

    lua_pushcfunction(l, parse_ntr_files_from_directory);
    lua_setfield(l, -2, cstr!("ParseDirectory"));

    lua_pushcfunction(l, get_cached_files);
    lua_setfield(l, -2, cstr!("GetCachedFiles"));

    lua_setglobal(l, cstr!("NTRParser"));

    0
}

#[gmod_close]
fn close(_l: LuaState) -> i32 {
    0
}

#[lua_function]
pub fn parse_ntr_file_lua(l: LuaState) -> i32 {
    let file_path = rstr!(luaL_checkstring(l, 1));

    match parse_ntr_file(&file_path) {
        Ok(ntr_data) => {
            let mut parsed_files = PARSED_FILES.lock().unwrap();
            parsed_files.insert(file_path.to_string(), ntr_data);
            lua_pushboolean(l, true as i32);
        }
        Err(e) => {
            lua_pushboolean(l, false as i32);
            let error_message = CString::new(format!("Error parsing file: {}", e)).unwrap();
            lua_pushstring(l, error_message.as_ptr());
            return 2;
        }
    }

    1
}

#[lua_function]
pub fn get_ntr_value_lua(l: LuaState) -> i32 {
    let file_path = rstr!(luaL_checkstring(l, 1));
    let key = rstr!(luaL_checkstring(l, 2));

    let parsed_files = PARSED_FILES.lock().unwrap();

    if let Some(ntr_data) = parsed_files.get(file_path) {
        if let Some(value) = ntr_data.get(&key) {
            let c_value = CString::new(value.clone()).unwrap();
            lua_pushstring(l, c_value.as_ptr());
        } else {
            lua_pushnil(l);
        }
    } else {
        lua_pushnil(l);
        let error_message = CString::new(format!("File not parsed: {}", file_path)).unwrap();
        lua_pushstring(l, error_message.as_ptr());
        return 2;
    }

    1
}

#[lua_function]
pub fn ntr_key_exists(l: LuaState) -> i32 {
    let file_path = rstr!(luaL_checkstring(l, 1));
    let key = rstr!(luaL_checkstring(l, 2));

    let parsed_files = PARSED_FILES.lock().unwrap();

    if let Some(ntr_data) = parsed_files.get(file_path) {
        if ntr_data.get(&key).is_some() {
            lua_pushboolean(l, true as i32);
        } else {
            lua_pushboolean(l, false as i32);
        }
    } else {
        lua_pushboolean(l, false as i32);
        let error_message = CString::new(format!("File not parsed: {}", file_path)).unwrap();
        lua_pushstring(l, error_message.as_ptr());
        return 2;
    }

    1
}

#[lua_function]
pub fn get_all_keys_lua(l: LuaState) -> i32 {
    let file_path = rstr!(luaL_checkstring(l, 1));

    let parsed_files = PARSED_FILES.lock().unwrap();

    if let Some(ntr_data) = parsed_files.get(file_path) {
        lua_newtable(l);
        for (i, key) in ntr_data.data.keys().enumerate() {
            lua_pushinteger(l, ((i + 1) as i32).try_into().unwrap());
            let c_key = CString::new(key.clone()).unwrap();
            lua_pushstring(l, c_key.as_ptr());
            lua_settable(l, -3);
        }
        return 1;
    } else {
        lua_pushnil(l);
        let error_message = CString::new(format!("File not parsed: {}", file_path)).unwrap();
        lua_pushstring(l, error_message.as_ptr());
        return 2;
    }

}

#[lua_function]
pub fn clear_cache_lua(l: LuaState) -> i32 {
    let mut parsed_files = PARSED_FILES.lock().unwrap();
    parsed_files.clear();
    lua_pushboolean(l, true as i32);
    return 1;
}

#[lua_function]
pub fn unload_file_lua(l: LuaState) -> i32 {
    let file_path = rstr!(luaL_checkstring(l, 1));

    let mut parsed_files = PARSED_FILES.lock().unwrap();
    
    if parsed_files.remove(file_path).is_some() {
        lua_pushboolean(l, true as i32);
    } else {
        lua_pushboolean(l, false as i32);
        let error_message = CString::new(format!("File not found in cache: {}", file_path)).unwrap();
        lua_pushstring(l, error_message.as_ptr());
        return 2;
    }

    1
}

#[lua_function]
pub fn get_value_from_cache_without_file(l: LuaState) -> i32 {
    let key = rstr!(luaL_checkstring(l, 1));

    let parsed_files = PARSED_FILES.lock().unwrap();

    for (_, ntr_data) in parsed_files.iter() {
        if let Some(value) = ntr_data.get(&key) {
            let c_value = CString::new(value.clone()).unwrap();
            lua_pushstring(l, c_value.as_ptr());
            return 1;
        }
    }

    lua_pushnil(l);
    1
}


#[lua_function]
pub fn parse_ntr_files_from_directory(l: LuaState) -> i32 {
    let directory = rstr!(luaL_checkstring(l, 1));
    let mut files_parsed = 0;

    if let Ok(paths) = std::fs::read_dir(directory) {
        for path_result in paths {
            if let Ok(path) = path_result {
                let path_buf = path.path();

                if path_buf.is_file() {
                    if let Some(extension) = path_buf.extension() {
                        if extension == "ntr" {
                            if let Some(path_str) = path_buf.to_str() {
                                let c_path = CString::new(path_str).unwrap();
                                lua_pushstring(l, c_path.as_ptr());

                                if parse_ntr_file(&path_str).is_ok() {
                                    let mut parsed_files = PARSED_FILES.lock().unwrap();
                                    parsed_files.insert(path_str.to_string(), parse_ntr_file(&path_str).unwrap());
                                    files_parsed += 1;
                                }
                            }
                        }
                    }
                } else if path_buf.is_dir() {
                    if let Some(subdir_str) = path_buf.to_str() {
                        let c_subdir = CString::new(subdir_str).unwrap();
                        lua_pushstring(l, c_subdir.as_ptr());

                        files_parsed += parse_ntr_files_from_directory(l) as usize;
                    }
                }
            }
        }
    } else {
        lua_pushboolean(l, false as i32);
        let error_message = CString::new(format!("Could not read directory: {}", directory)).unwrap();
        lua_pushstring(l, error_message.as_ptr());
        return 2;
    }

    lua_pushinteger(l, files_parsed as LuaInteger);
    1
}

#[lua_function]
pub fn get_cached_files(l: LuaState) -> i32 {
    let parsed_files = PARSED_FILES.lock().unwrap();

    lua_newtable(l);
    for (i, (file_path, _)) in parsed_files.iter().enumerate() {
        lua_pushinteger(l, ((i + 1) as i32).try_into().unwrap());
        let c_file_path = CString::new(file_path.clone()).unwrap();
        lua_pushstring(l, c_file_path.as_ptr());
        lua_settable(l, -3);
    }

    1
}