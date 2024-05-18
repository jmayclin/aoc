use aoc2023::day8::rust_day8_p1;
use pyo3::{prelude::*, types::PyString};
use std::{fs, time::Instant};

fn main() {
    // Initialize Python interpreter
    
    //let result = rust_day8_p1(input_data.as_bytes().as_ptr(), input_data.len() as u64);
    let code = fs::read_to_string("d8.py").unwrap();
    let input_data = fs::read_to_string("resources/d8_input.txt").expect("Failed to read input file");

    Python::with_gil(|py| {
        // Import Python module
        let d8_module = PyModule::from_code_bound(
            py,
            &code,
            "d8.py",
            "d8",
        ).unwrap();

        // let module = PyModule::from_code_bound(py, CODE, "", "")?;
        // let fun = module.getattr("function")?;
        // let args = ("hello",);
        // let kwargs = PyDict::new_bound(py);
        // kwargs.set_item("cruel", "world")?;
        // let result = fun.call(args, Some(&kwargs))?;
        // assert_eq!(result.extract::<String>()?, "called with args and kwargs");
        // Ok(())

        let input_data = fs::read_to_string("resources/d8_input.txt").expect("Failed to read input file");

        let py_input_data = PyString::new_bound(py, &input_data);

        let function = d8_module.getattr("d8_p1").unwrap();
        assert!(function.is_callable());
        //let result = function.call0().unwrap();
        //let result = function.call1(("data here",)).unwrap();
        
        println!("starting the timing loop");
        let start = Instant::now();

        for _ in 0..1_000 {
            let result = function.call1((&py_input_data,)).unwrap();
        }
        let end = start.elapsed() / 1_000;
        println!("finished the timing loop");
        //println!("elapsed time to call python function: {:?}", end);
        let result = function.call1((py_input_data,)).unwrap();
        let result = result.extract::<i32>().unwrap();
        assert_eq!(result, 13019);
        // Get reference to the d8_p1 function

    });
}
