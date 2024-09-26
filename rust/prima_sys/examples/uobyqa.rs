//! Example of using the UOBYQA optimization algorithm.

use prima_sys::*;
use std::os::raw::{c_int, c_void};

unsafe extern "C" fn objective(
    x: *const f64,
    f: *mut f64,
    _data: *const c_void) {
    let x = std::slice::from_raw_parts(x, 2);
    *f = (x[0] - 5.0).powi(2) + (x[1] - 4.0).powi(2);
}

unsafe extern "C" fn intermediate(
    n: c_int,
    x: *const f64,
    f: f64,
    nf: c_int,
    _tr: c_int,
    _cstrv: f64,
    _m_nlcon: c_int,
    _nlconstr: *const f64,
    terminate: *mut bool,
) -> () {
    *terminate = false;

    println!(
        "Intermediate callback called with x = {:?}, f = {}, nf = {}",
        std::slice::from_raw_parts(x, n as usize),
        f,
        nf
    );
}

fn main() -> () {
    let number_of_variables: i32 = 2;
    let mut initial_variables = vec![0.0; number_of_variables as usize];
    let objective_callback: prima_obj_t = Some(objective);
    let intermediate_callback: prima_callback_t = Some(intermediate);

    let mut problem = new_problem(number_of_variables);
    problem.x0 = initial_variables.as_mut_ptr();
    problem.calfun = objective_callback;

    let mut options = new_options();
    options.iprint = prima_message_t_PRIMA_MSG_EXIT as i32;
    options.maxfun = 500 * number_of_variables;
    options.rhoend = 1e-6;
    options.callback = intermediate_callback;

    let mut result = prima_result_t::default();
    let result_ptr = std::ptr::addr_of_mut!(result);
    let _ = unsafe {
        prima_minimize(
            prima_algorithm_t_PRIMA_UOBYQA,
            problem,
            options,
            result_ptr);
    };
    unsafe { prima_free_result(result_ptr); }
}
