pub trait ConfigDefault {
    fn get_default() -> Self;
}

// These types need default values
pub struct ConnectionTimeout(pub u64);
pub struct MaxConnections(pub u32);
pub struct RetryAttempts(pub u8);
pub struct PostgresPort(pub u16);
pub struct MySQLPort(pub u16);
pub struct MongoPort(pub u16);
pub struct RedisPort(pub u16);

#[macro_export]
macro_rules! config_default_impl {
    ($obj: tt, $val: expr) => {
        impl ConfigDefault for $obj {
            fn get_default() -> Self {
                Self($val)
            }
        }
    };
}

/*
   - `ConnectionTimeout` with a value of `30`.
   - `MaxConnections` with a value of `100`.
   - `RetryAttempts` with a value of `3`.
   - `PostgresPort` with a value of `5432`.
   - `MySQLPort` with a value of `3306`.
   - `MongoPort` with a value of `27017`.
   - `RedisPort` with a value of `6379`.
*/
config_default_impl!(ConnectionTimeout, 30);
config_default_impl!(MaxConnections, 100);
config_default_impl!(RetryAttempts, 3);
config_default_impl!(PostgresPort, 5432);
config_default_impl!(MySQLPort, 3306);
config_default_impl!(MongoPort, 27017);
config_default_impl!(RedisPort, 6379);

// Example usage
pub fn main() {
    // let's say we have a new struct
    struct CustomPort(pub u16);

    // we implement the ConfigDefault trait for CustomPort
    config_default_impl!(CustomPort, 8080);

    // when running the `get_default` method, it should return the default value
    assert_eq!(<CustomPort as ConfigDefault>::get_default().0, 8080);
}
