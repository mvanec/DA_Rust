use std::collections::HashMap;
use std::marker::PhantomData;

use secrecy::ExposeSecret;
use mysql::{Opts, OptsBuilder, Pool};

use crate::data_loader::{DataLoader, DataLoaderConfig, DataLoaderError};
use crate::loadable::Loadable;

pub struct MySqlDataLoader<T> {
    pub pool: Pool,
    _marker: PhantomData<T>,
}

impl<T> MySqlDataLoader<T> {
    pub fn new(config: DataLoaderConfig) -> Result<Self, DataLoaderError> {
        let password = config.password.expose_secret();
        let opts: Opts = OptsBuilder::new()
            .ip_or_hostname(Some(config.source))
            .user(Some(config.username))
            .pass(Some(password))
            .db_name(Some(config.dataset))
            .into();

        let pool = Pool::new(opts)?;
        Ok(Self { pool, _marker: PhantomData })
    }
}

impl<T> DataLoader<T> for MySqlDataLoader<T>
where
    T: Loadable,
{
    fn get_pool(&self) -> &Pool {
        &self.pool
    }

    fn load_data(&self) -> Result<Vec<T>, DataLoaderError> {
        T::load(self)
    }

    fn get_type(&self) -> String {
        return "MySQL".to_string();
    }

    fn get_options(&self) -> HashMap<String, String> {
        todo!()
    }
}
