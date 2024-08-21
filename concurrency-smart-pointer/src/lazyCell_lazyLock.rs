// 有时候我们想实现懒 (惰性) 初始化的效果，当然 lazy_static 库可以实现这个效 果，但是 Rust 标准库也提供了一个功能，不过目前还处于不稳定的状态，
// 你需要设 置 #![feature(lazy_cell)] 使能它。

// 需要注意的是：stable 版本的 Rust 不能使用 feature 这个属性：#![feature(lazy_cell)]
// 需要将 stable 版本替换为 nightly 版本
// 这样可以使得我们使用 不稳定的（unstable） 的特性或者是第三方 crate

#![feature(lazy_cell)]

// pub fn lazy_cell() {
//     use std::cell::LazyCell;
//     let lazy: LazyCell<i32> = LazyCell::new(|| {
//         println!("initializing");
//         46
//     });
//     // 注意它是懒初始化的，也就是你在第一次访问它的时候它才会调用初始化函数进行初始化
//     println!("ready");
//     println!("{}", *lazy); // 46
//     println!("{}", *lazy); // 46
// }

// lazyCell 不是线程安全的，如果想使用线程安全的版本， 你可以使用 lazyLock

// use std::collections::HashMap;
// use std::sync::LazyLock;

// pub static HASHMAP: LazyLock<HashMap<i32, String>> = LazyLock::new(|| {
//     println!("initializing");
//     let mut m = HashMap::new();
//     m.insert(13, "Spica".to_string());
//     m.insert(74, "Hoyten".to_string());
//     m
// });
