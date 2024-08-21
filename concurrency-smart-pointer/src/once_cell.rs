use core::cell::OnceCell;

// OnceCell 是 Rust 标准库中的一个类型，用于提供一次性写入的单元格
// 它允许在运 行时将值放入单元格，但只允许一次。一旦值被写入，进一步的写入尝试将被忽略
// 
// 主要特点和用途:
// 1. 一次性写入: OnceCell 确保其内部值只能被写入一次。一旦值被写入，后续的写入操作将被忽略
// 2. 懒初始化: OnceCell 支持懒初始化，这意味着它只有在需要时才会进行初始化。这在需要在运行时确定何时初始化值的情况下很有用
// 3. 线程安全: OnceCell 提供了线程安全的一次性写入。在多线程环境中，它确保只有一个线程能够成功写入值，而其他线程的写入尝试将被忽略


pub fn once_cell_example() {
    let cell: OnceCell<String> = OnceCell::new(); 
    // 未初始化的时候，其值为 None
    assert!(cell.get().is_none()); // true

    // get_or_init 接收一个闭包，用来初始化 OnceCell 包裹的值
    let value: &String = cell.get_or_init(|| "Hello, World!".to_string()); 
    assert_eq!(value, "Hello, World!");
    
    assert!(cell.get().is_some()); // true

    println!("{:?}", cell.get());
}