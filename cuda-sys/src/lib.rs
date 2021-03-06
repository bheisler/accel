pub mod vector_types;
pub mod cuda;
pub mod cudart;
pub mod cublas;

#[test]
fn cuda_version() {
    let mut d_ver = 0;
    unsafe {
        cuda::cuDriverGetVersion(&mut d_ver as *mut i32);
    }
    println!("driver version = {}", d_ver);
}

mod cuda_tests;
mod cudart_tests;
