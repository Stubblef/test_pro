// 引入pyo3库
use pyo3::prelude::*;

// 定义一个斐波那契数列的函数
# [pyfunction]  // 这个标记在 Rust 中通常与 PyO3 库一起使用，用于指示一个 Rust 函数是 Python 中可调用的
fn fibonacci(n:u32) -> PyResult<u64>{
    if n <= 1 {
        return Ok(n as u64);
    } else{
        Ok(fibonacci(n-1)? + fibonacci(n-2)?)  // 递归调用自身
    }
}

// 定义一个Python模块
#[pymodule]  // 这个标记在 Rust 中通常与 PyO3 库一起使用，用于指示一个 Rust 模块是一个 Python 模块
fn fibonacci_rust(_py: Python, m: &PyModule) -> PyResult<()>{
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;  // ? 是一个错误处理运算符，用于处理可能出现的错误
    Ok(())
}
