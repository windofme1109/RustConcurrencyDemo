// Cell 和 RefCell 是 Rust 中用于内部可变性 (interior mutability) 的两个重要类型。
// Cell 和 RefCell 都是可共享的可变容器
// 可共享的可变容器的存在是为了以受控的方式允许可变性，即使存在别名引用
// Cell 和 RefCell 都允许在单线程环境下以这种方式进行
// 但是无论是 Cell 还是 RefCell 都不是线程安全的(它们没有实现 Sync)。

// 1. Cell
//
// Cell<T> 允许在不违反借用规则的前提下, 修改其包含的值:
// 1. Cell 中的值不再拥有所有权, 只能通过 get 和 set 方法访问
// 2. set 方法可以在不获取可变引用的情况下修改
// 3. 适用于简单的单值容器，如整数或字符

use std::cell::Cell;
use std::cell::RefCell;
use std::ops::Deref;

pub fn cell_demo() {
    let x = Cell::new(42);
    // 对 x 的引用
    let y = &x;
    // 修改 x 的值
    x.set(10);
    //
    println!("y: {:?}", y.get());
}

// 2. RefCell
//
// RefCell<T> 提供了更灵活的内部可变性，允许在运行时检查借用规则, 通过运行时借 用检查来实现：
// 1. 通过 borrow 和 borrow_mut 方法进行不可变和可变借用
// 2. 借用必须在作用域结束前归还, 否则会 panic
// 3. 适用于包含多个字段的容器

pub fn refcell_demo() {

    let x = RefCell::new(42);
    
    {
        // 这个作用域内，只能实现不可变借用
        let y = x.borrow();
        //
        println!("y: {:?}", *y);
    }

    {
        let mut z = x.borrow_mut();
        // 这个作用域内可以获得可变引用
        *z = 10;

        println!("z: {:?}", z);
    }

    println!("x: {:?}", x.borrow().deref());
}
