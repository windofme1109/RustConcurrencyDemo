// Box<T>，通常简称为 box，提供了在 Rust 中最简单的堆分配形式
// Box 为这个分配提供了所有权，并在超出作用域时释放其内容。Box 还确保它们不会分配超过 isize::MAX 字节的内存




pub fn box_demo() {


    // val 分配在栈内存上  
    let val: u8 = 5;
    // 将 val 从栈上移动到堆上:
    let boxed: Box<u8> = Box::new(val);

    // 通过解引用把值从堆上移动到栈上
    let boxed_2: Box<u8> = Box::new(5); 
    
    let val_2: u8 = *boxed_2;

    println!("val_2：{}", val_2);
}


// 定义一个递归的数据结构，比如链表，下面的方式是不行的，因为 List 的大 小不固定，我们不知道该分配给它多少内存:
// error[E0072]: recursive type `List` has infinite size
// #[derive(Debug)] 
// enum List<T> {
//      Cons(T, List<T>),
//     Nil, 
// }

// 使用 Box 进行包裹
#[derive(Debug)] 
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}



// 使用 Box 类型定义链表

pub fn linked_list_from_box() {


    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    let list_2: List<i32> = List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))));
    println!("{list:?}");
    println!("{:?}", list_2);
}