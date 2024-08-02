use mlua::prelude::*;
use std::cell::RefCell;

thread_local! {
    static CONTEXT: RefCell<Option<Ctx<'static, 'static>>> = RefCell::new(None);
}

struct Ctx<'a, 'b> {
    first: &'a A,
    second: &'b B<'b>,
    // ...
}

fn my_function(lua: &Lua) -> LuaResult<()> {
    CONTEXT.with(|ctx| {
        let ctx = ctx.borrow();
        let ctx = ctx.as_ref().ok_or_else(|| {
            LuaError::RuntimeError("Context not available".to_string())
        })?;
        // do something with ctx
        Ok(())
    })
}

fn set_exports(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();
    globals.set("my_function", lua.create_function(my_function)?)?;
    Ok(())
}

fn run_lua_with_ctx(lua: &Lua, code: Chunk, context: Ctx<'_, '_>) -> LuaResult<()> {
    // Set the context in TLS
    CONTEXT.with(|ctx| {
        *ctx.borrow_mut() = Some(unsafe { std::mem::transmute(context) });
    });

    // Execute the Lua code
    lua.load(code).exec()?;

    // Clear the context from TLS
    CONTEXT.with(|ctx| {
        *ctx.borrow_mut() = None;
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
