pub mod system;
pub mod uiconf;
use crate::KalturaClient;

trait List<T> {
    fn list(&self) -> T;
}

trait Get<T> {
    fn get(&self, id: i32) -> T;
}

trait Delete<T> {
    fn delete(&self, id: i32) -> T;
}

trait Add<T> {
    fn add(&self, obj: T) -> T;
}

trait Update<T> {
    fn update(&self, id: i32, obj: T) -> T;
}

trait Service {
    fn new(client: &KalturaClient) -> T;
}

impl<T> Service<T> for T {
    fn new(client: &KalturaClient) -> T {
        T { client }
    }
}