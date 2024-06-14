use crate::data_loader::DataLoader;
use crate::data_loader::DataLoaderError;

pub trait Loadable {
    fn load<T>(loader: &dyn DataLoader<T>) -> Result<Vec<Self>, DataLoaderError>
    where
        Self: Sized;
}
