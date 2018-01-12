// The MIT License (MIT)
//
// Copyright (c) 2014 J.C. Moyer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! Low level bindings to Lua.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// This is more or less in the order it appears in the Lua manual, with the
// exception of constants, which appear scattered throughout the manual text.

// luaconf.h functions
pub use self::luaconf::lua_numtointeger;

// C API types
pub use self::lua::{lua_Alloc, lua_CFunction, lua_Debug, lua_Hook, lua_Integer, lua_KContext,
                    lua_KFunction, lua_Number, lua_Reader, lua_State, lua_Unsigned, lua_Writer};

// C API functions
pub use self::lua::{lua_absindex,
                    lua_arith,
                    lua_atpanic,
                    lua_call,
                    lua_callk,
                    lua_checkstack,
                    lua_close,
                    lua_compare,
                    lua_concat,
                    lua_copy,
                    lua_createtable,
                    lua_dump,
                    lua_error,
                    lua_gc,
                    lua_getallocf,
                    lua_getextraspace,
                    lua_getfield,
                    lua_getglobal,
                    lua_gethook,
                    lua_gethookcount,
                    lua_gethookmask,
                    lua_geti,
                    lua_getinfo,
                    lua_getlocal,
                    lua_getmetatable,
                    lua_getstack,
                    lua_gettable,
                    lua_gettop,
                    lua_getupvalue,
                    lua_getuservalue,
                    lua_insert,
                    lua_isboolean,
                    lua_iscfunction,
                    lua_isfunction,
                    lua_isinteger,
                    lua_islightuserdata,
                    lua_isnil,
                    lua_isnone,
                    lua_isnoneornil,
                    lua_isnumber,
                    lua_isstring,
                    lua_istable,
                    lua_isthread,
                    lua_isuserdata,
                    lua_isyieldable,
                    lua_len,
                    lua_load,
                    lua_newstate,
                    lua_newtable,
                    lua_newthread,
                    lua_newuserdata,
                    lua_next,
                    lua_pcall,
                    lua_pcallk,
                    lua_pop,
                    lua_pushboolean,
                    lua_pushcclosure,
                    lua_pushcfunction,
                    lua_pushfstring,
                    lua_pushglobaltable,
                    lua_pushinteger,
                    lua_pushlightuserdata,
                    lua_pushliteral,
                    lua_pushlstring,
                    lua_pushnil,
                    lua_pushnumber,
                    lua_pushstring,
                    lua_pushthread,
                    lua_pushvalue,
                    // omitted: lua_pushvfstring
                    lua_rawequal,
                    lua_rawget,
                    lua_rawgeti,
                    lua_rawgetp,
                    lua_rawlen,
                    lua_rawset,
                    lua_rawseti,
                    lua_rawsetp,
                    lua_register,
                    lua_remove,
                    lua_replace,
                    lua_resume,
                    lua_rotate,
                    lua_setallocf,
                    lua_setfield,
                    lua_setglobal,
                    lua_sethook,
                    lua_seti,
                    lua_setlocal,
                    lua_setmetatable,
                    lua_settable,
                    lua_settop,
                    lua_setupvalue,
                    lua_setuservalue,
                    lua_status,
                    lua_stringtonumber,
                    lua_toboolean,
                    lua_tocfunction,
                    lua_tointeger,
                    lua_tointegerx,
                    lua_tolstring,
                    lua_tonumber,
                    lua_tonumberx,
                    lua_topointer,
                    lua_tostring,
                    lua_tothread,
                    lua_touserdata,
                    lua_type,
                    lua_typename,
                    lua_upvalueid,
                    lua_upvalueindex,
                    lua_upvaluejoin,
                    lua_version,
                    lua_xmove,
                    lua_yield,
                    lua_yieldk};

// auxiliary library types
pub use self::lauxlib::{luaL_Buffer, luaL_Reg, luaL_Stream};

// auxiliary library functions
pub use self::lauxlib::{luaL_addchar, luaL_addlstring, luaL_addsize, luaL_addstring,
                        luaL_addvalue, luaL_argcheck, luaL_argerror, luaL_buffinit,
                        luaL_buffinitsize, luaL_callmeta, luaL_checkany, luaL_checkint,
                        luaL_checkinteger, luaL_checklong, luaL_checklstring, luaL_checknumber,
                        luaL_checkoption, luaL_checkstack, luaL_checkstring, luaL_checktype,
                        luaL_checkudata, luaL_checkversion, luaL_dofile, luaL_dostring,
                        luaL_error, luaL_execresult, luaL_fileresult, luaL_getmetafield,
                        luaL_getmetatable, luaL_getsubtable, luaL_gsub, luaL_len, luaL_loadbuffer,
                        luaL_loadbufferx, luaL_loadfile, luaL_loadfilex, luaL_loadstring,
                        luaL_newlib, luaL_newlibtable, luaL_newmetatable, luaL_newstate,
                        luaL_optint, luaL_optinteger, luaL_optlong, luaL_optlstring,
                        luaL_optnumber, luaL_optstring, luaL_prepbuffer, luaL_prepbuffsize,
                        luaL_pushresult, luaL_pushresultsize, luaL_ref, luaL_requiref,
                        luaL_setfuncs, luaL_setmetatable, luaL_testudata, luaL_tolstring,
                        luaL_traceback, luaL_typename, luaL_unref, luaL_where};

// lualib.h functions
pub use self::lualib::{luaopen_base, luaopen_coroutine, luaopen_debug, luaopen_io, luaopen_math,
                       luaopen_os, luaopen_package, luaopen_string, luaopen_table, luaL_openlibs,
                       luaopen_bit32, luaopen_utf8};

// constants from lua.h
pub use self::lua::{LUA_ERRERR, LUA_ERRGCMM, LUA_ERRMEM, LUA_ERRRUN, LUA_ERRSYNTAX, LUA_GCCOLLECT,
                    LUA_GCCOUNT, LUA_GCCOUNTB, LUA_GCISRUNNING, LUA_GCRESTART, LUA_GCSETPAUSE,
                    LUA_GCSETSTEPMUL, LUA_GCSTEP, LUA_GCSTOP, LUA_HOOKCALL, LUA_HOOKCOUNT,
                    LUA_HOOKLINE, LUA_HOOKRET, LUA_HOOKTAILCALL, LUA_MASKCALL, LUA_MASKCOUNT,
                    LUA_MASKLINE, LUA_MASKRET, LUA_MULTRET, LUA_OK, LUA_OPADD, LUA_OPBAND,
                    LUA_OPBNOT, LUA_OPBOR, LUA_OPBXOR, LUA_OPDIV, LUA_OPEQ, LUA_OPIDIV, LUA_OPLE,
                    LUA_OPLT, LUA_OPMOD, LUA_OPMUL, LUA_OPPOW, LUA_OPSHL, LUA_OPSHR, LUA_OPSUB,
                    LUA_OPUNM, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS, LUA_RIDX_MAINTHREAD,
                    LUA_TBOOLEAN, LUA_TFUNCTION, LUA_TLIGHTUSERDATA, LUA_TNIL, LUA_TNONE,
                    LUA_TNUMBER, LUA_TSTRING, LUA_TTABLE, LUA_TTHREAD, LUA_TUSERDATA, LUA_YIELD};

// constants from lauxlib.h
pub use self::lauxlib::{LUA_ERRFILE, LUA_FILEHANDLE, LUA_NOREF, LUA_REFNIL};

// constants from lualib.h
pub use self::lualib::{LUA_UTF8LIBNAME, LUA_BITLIBNAME, LUA_COLIBNAME, LUA_DBLIBNAME,
                       LUA_IOLIBNAME, LUA_LOADLIBNAME, LUA_MATHLIBNAME, LUA_OSLIBNAME,
                       LUA_STRLIBNAME, LUA_TABLIBNAME};

#[allow(unused_imports, dead_code, non_camel_case_types)]
mod glue {
    include!(concat!(env!("OUT_DIR"), "/glue.rs"));
}

mod luaconf;
mod lua;
mod lauxlib;
mod lualib;
