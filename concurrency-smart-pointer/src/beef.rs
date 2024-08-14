use beef::Cow;
use std::mem::size_of;
// beef 是一个第三方 crate，它提供了一个更快，更紧凑的 Cow 类型, 它的使用方法和标准库的 Cow 使用方法类似

pub fn beef_cow() {

    // 生成 beef::Cow 的三种方法 
    // Cow::borrowed
    // Cow::from、 
    // Cow::owned
    // 标准库 Cow 也有这三个方法，它们的区别是：
    // borrowed: 借用已有资源
    // from: 从已有资源复制创建 Owned 
    // owned: 自己提供资源内容
    let borrowed: Cow<str> = Cow::borrowed("Hello");
    
    let _ = beef::Cow::from("Hello");

    let owned: Cow<str> = Cow::owned(String::from("World"));

    

    assert_eq!(format!("{} {}!", borrowed, owned), "Hello World!",);

    // size_of 方法返回一个类型占用的字节数
    // usize 占据 8 个字节，所以 WORD 为 8
    const WORD: usize = size_of::<usize>();

    // 对比了标准库 Cow 和 beef::Cow 以及更紧凑的 beef::lean::Cow 所占内存的大小
    // 可以看到对于数据是 str 类型的 Cow
    // 现在的标准库的 Cow 占三个 WORD
    // 和 beef::Cow 相当
    // 而进一步压缩的 beef::lean::Cow 只占了两个 Word
    assert_eq!(size_of::<std::borrow::Cow<str>>(), 3 * WORD);
    assert_eq!(size_of::<beef::Cow<str>>(), 3 * WORD);
    assert_eq!(size_of::<beef::lean::Cow<str>>(), 2 * WORD);
}
