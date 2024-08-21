// Rc 是 Rust 标准库中的一个智能指针类型，全名是 std::rc::Rc
// 代表 “reference counting’’,它用于在多个地方共享相同数据时，通过引用计数来进行所有权管理

// 特点：
// 1. Rc 使用引用计数来追踪指向数据的引用数量。当引用计数降为零时，数据会被自动释放
// 2. Rc 允许多个 Rc 指针共享相同的数据，而无需担心所有权的转移
// 3. Rc 内部存储的数据是不可变的。如果需要可变性，可以使用 RefCell 或 Mutex 等内部可变性的机制。
// 4. Rc 在处理循环引用时需要额外注意，因为循环引用会导致引用计数无法降为零，从而导致内存泄漏
//    为了解决这个问题，可以使用 Weak 类型


// 注意 Rc 不是线程安全的


use std::cell::Cell;
use std::collections::HashMap;
use std::cell::RefMut;
use std::cell::RefCell;
use std::rc::Rc;

pub fn basic_rc() {
    let data = Rc::new(42);
    // 使用 clone 方法获得新的共享引用
    let reference1 = Rc::clone(&data);
    let reference2 = Rc::clone(&data);

    // data 的引用计数现在为 3
    // reference1 和 reference2 被丢弃的时候，引用计数减少
    // 注意 Rc 允许在多个地方共享不可变数据，通过引用计数来管理所有权

    // 42
    println!("{:?}", reference1);
    // 42
    println!("{:?}", reference2);
}

pub fn rc_refcell_example() {

    // 如果还想修改数据，那么就可以使用上一节的 RefCell 相关类型
    //  使用 Rc<RefCell<HashMap>> 类型
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    
    {
        // 可变借用 RefCell 中的值
        // sharp_map 可以直接调用 RefCell 类型相关的 api 
        let mut map: RefMut<_> = shared_map.borrow_mut();

        // 调整 map 值
        map.insert("africa", 92388);
        map.insert("kyoto", 11837);
        map.insert("piccadilly", 11826);
        map.insert("marbles", 38);

        println!("map: {:?}", map);
    }

    // 不可变借用 shared_map 中保存的 map，然后通过 values 方法获取 map 中的值（value）
    // values 方法的返回值是迭代器（iterator），所以使用 sum 方法对迭代器中的值进行求和
    let total: i32 = shared_map.borrow().values().sum();
    println!("{total}");
}
