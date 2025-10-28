#[macro_export]
macro_rules! hashmap {
    ( $( $key:expr => $value:expr, )+ ) => {
        {
            let mut temp_hashmap = ::std::collections::HashMap::new();
            $(
                temp_hashmap.insert($key, $value);
            )*
            temp_hashmap
        }
    };
    ( $( $key:expr => $value:expr ),* ) => {
        {
            let mut temp_hashmap = ::std::collections::HashMap::new();
            $(
                temp_hashmap.insert($key, $value);
            )*
            temp_hashmap
        }
    };
}