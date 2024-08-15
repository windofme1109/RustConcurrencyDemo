// cow 不是那个单词 cow，而是 clone-on-write 或者 copy-on-write 的缩写
// cow(Copy-on-write) 是一种优化内存和提高性能的技术, 通常应用在资源共享的场景
// 其基本思想是, 当有多个调用者 (callers) 同时请求相同的资源时, 都会共享同一份资源
// 直到有调用者试图修改资源内容时, 系统才会真正复制一份副本出来给该调用者, 而其 他调用者仍然使用原来的资源。

mod cow;
mod beef;
mod box_pointer;
mod cell_type_pointer;

use cow::*;

use beef::*;

use box_pointer::*;
use cell_type_pointer::*;

fn main() {
    // println!("Hello, world!");

    // Rust 中的 String 和 Vec 等类型就利用了 COW
    let s1 = String::from("hello");
    // s1 和 s2 共享一份内存
    let mut s2 = s1; 
    // s2 会进行写操作，于是系统复制一份新的内存给 s2
    s2.push_str(" world"); 

    // cow_demo();

    // cow_demo_2();

    // box_demo();

    // linked_list_from_box();

    cell_demo();

    refcell_demo();
}
