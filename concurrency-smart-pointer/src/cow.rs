use std::borrow::Cow;

// cow 的优点是: - 内存利用率高, 只有进行写时才复制 - 读取性能高, 多个调用者共享同一资源
// cow 的缺点是: - 写时需要复制, 有一定性能损失 - 实现较复杂

// 需要根据实际场景权衡使用。但对于存在大量相同或相似资源的共享情况, 使用 cow 可 以带来显著性能提升

// 标准库中 std::borrow::Cow 类型是一个智能指针，提供了写时克隆(clone-on-write) 的功能
// 它可以封装并提供对借用数据的不可变访问，当需要进行修改或获取所有权时， 它可以惰性地克隆数据

// Cow 实现了 Deref，这意味着你可以直接在其封装的数据上调用不可变方法
// 如果需要进行改变，则 to_mut 将获取到一个对拥有的值的可变引用，必要时进行克隆

pub fn cow_demo() {
    let origin = "hello world";
    // 使用 from 将 origin 包装成 cow
    let mut cow = Cow::from(origin);
    assert_eq!(cow, "hello world");

    // Cow 可以被借用为 字符串切片
    let s: &str = &cow;

    assert_eq!(s, "hello world");
    // 因为 Cow 实现了 Deref，可以自动解引用
    assert_eq!(s.len(), cow.len());

    // Cow 也可以转换为字符串
    let s: String = cow.into();
    assert_eq!(s, "hello world");
}

pub fn cow_demo_2() {

    let origin = "hello world";

    let mut cow = Cow::from(origin);

    // Cow can be borrowed as a mut str
    // 
    // 这里使用 to_mut 得到一个可变引用，一旦 s 有修改，它会从原始数据中 clone 一份， 在克隆的数据上进行修改
    let s: &mut str = cow.to_mut();

    // 对 s 做了修改，此时会复制一份新的数据给 s，这样不会影响原来的 origin
    s.make_ascii_uppercase();
    
    assert_eq!(s, "HELLO WORLD");
    assert_eq!(origin, "hello world");
}
