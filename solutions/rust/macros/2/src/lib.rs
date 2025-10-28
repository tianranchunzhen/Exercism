#[macro_export(local_inner_macros)]
macro_rules! hashmap {
    ( $( $key:expr => $value:expr, )+ ) => {
        hashmap!($($key => $value),+)
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
