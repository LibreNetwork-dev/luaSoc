use mlua::{Lua, Result, Function, Table, Value, lua_module};
use std::os::unix::net::UnixStream;
use std::io::Write;

#[lua_module]
fn luaSoc(lua: &Lua) -> Result<Table> {
    let exports = lua.create_table()?;

    let sendData = lua.create_function(|_, (path, data): (String, String)| {
        let mut stream = UnixStream::connect(path)
            .map_err(|e| mlua::Error::external(format!("Failed to connect socket: {}", e)))?;

        stream.write_all(data.as_bytes())
            .map_err(|e| mlua::Error::external(format!("Failed to write data: {}", e)))?;

        Ok(Value::Nil)
    })?;

    exports.set("sendData", sendData)?;

    Ok(exports)
}
