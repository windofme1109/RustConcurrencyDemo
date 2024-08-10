use affinity::*;

// 使用第三方 crate affinity 允许我们将线程绑定在一个核上或者几个核上

// 绑核是在极端情况提升性能的有效手段之一，将某几个核只给我们的应用使用
// 可以让这些核专门提供给我们的业务服务，既提供了 CPU 资源隔离，还提升了性能
#[cfg(not(target_os = "macos"))]
pub fn bind_even_cores() {

    // Select every second core
    let cores: Vec<usize> = (0..get_core_num()).step_by(2).collect();
    println!("Binding thread to cores : {:?}", &cores);
    // Output : "Binding thread to cores : [0, 2, 4, 6]"



    // 将线程绑定到指定的偶数核心上
    set_thread_affinity(&cores).unwrap();
    println!("Current thread affinity : {:?}", get_thread_affinity().unwrap());
    // Output : "Current thread affinity : [0, 2, 4, 6]"
}