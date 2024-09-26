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

    let mut uninit_problem = std::mem::MaybeUninit::<prima_problem_t>::uninit();
    let mut uninit_options = std::mem::MaybeUninit::<prima_options_t>::uninit();
    let mut uninit_result = std::mem::MaybeUninit::<prima_result_t>::uninit();

    unsafe {
        let uninit_problem_ptr = uninit_problem.as_mut_ptr();
        let uninit_options_ptr = uninit_options.as_mut_ptr();
        let uninit_result_ptr = uninit_result.as_mut_ptr();

        prima_init_problem(uninit_problem_ptr, number_of_variables);
        (*uninit_problem_ptr).x0 = initial_variables.as_mut_ptr();
        (*uninit_problem_ptr).calfun = objective_callback;

        prima_init_options(uninit_options_ptr);
        (*uninit_options_ptr).iprint = prima_message_t_PRIMA_MSG_EXIT as i32;
        (*uninit_options_ptr).maxfun = 500 * number_of_variables;
        (*uninit_options_ptr).rhoend = 1e-6;
        (*uninit_options_ptr).callback = intermediate_callback;

        let problem = uninit_problem.assume_init();
        let options = uninit_options.assume_init();

        let _return_code = prima_minimize(
            prima_algorithm_t_PRIMA_UOBYQA,
            problem,
            options,
            uninit_result_ptr);

        prima_free_result(uninit_result_ptr);
    }
}
