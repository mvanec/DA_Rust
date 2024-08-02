use std::rc::Rc;
use std::cell::RefCell;
use mlua::{UserData, FromLua, Lua, ToLua, Table, Value, Chunk};

struct Ctx<'a, 'b> {
    first: &'a A,
    second: &'b B<'b>,
    // ...
}

#[derive(Clone)]
struct CtxUserData {
    ctx: Rc<RefCell<Option<Ctx<'_, '_>>>>,
}

impl UserData for CtxUserData {
    fn add_fields<'lua, F: FnOnce(&mut Table<'lua>)>(fields: &mut F) {
        fields();
    }
}

impl FromLua<'_> for CtxUserData {
    fn from_lua(lua: &'lua Lua, value: Value<'lua>) -> Result<Self, LuaError> {
        unimplemented!(); // Implement this based on how you pass the context
    }
}

impl ToLua for CtxUserData {
    fn to_lua(self, lua: &Lua) -> Result<Value<'_>, LuaError> {
        // This will never be called directly since we don't expose CtxUserData to Lua
        unimplemented!();
    }
}

fn my_function(lua: Lua, ctx_data: CtxUserData) -> Result<(), LuaError> {
    let ctx = ctx_data.ctx.borrow().clone().ok_or(LuaError::new("Context is missing"))?;
    // Now you can use `ctx` within your function
    Ok(())
}

fn run_lua_with_ctx(lua: Lua, code: Chunk, context: &Ctx) {
    let ctx_ref = Rc::new(RefCell::new(Some(context.clone()))); // Clone context
    let ctx_data = CtxUserData { ctx: ctx_ref.clone() };

    // Register `my_function` with the Lua state, passing the context userdata
    lua.globals().set("my_function", lua.create_function(move |_, ctx_data: CtxUserData| {
        my_function(lua.clone(), ctx_data)
    })?)?;

    // Execute Lua code
    let result = lua.load(code).exec()?;
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lua = Lua::new();

        let map_table = lua.create_table()?;
        map_table.set(1, "one")?;
        map_table.set("two", 2)?;

        lua.globals().set("map_table", map_table)?;

        lua.load("for k,v in pairs(map_table) do print(k,v) end").exec()?;

        Ok(())
    }
}
