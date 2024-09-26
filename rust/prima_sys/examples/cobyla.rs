//! Example of using the COBYLA optimization algorithm.

use prima_sys::*;
use std::os::raw::{c_int, c_void};

unsafe extern "C" fn objective_and_constraints(
    x: *const f64,
    f: *mut f64,
    constr: *mut f64,
    _data: *const c_void) {
    let x = std::slice::from_raw_parts(x, 2);
    let x1 = x[0];
    let x2 = x[1];
    *f = (x1 - 5.0).powi(2) + (x2 - 4.0).powi(2);

    let constraints = std::slice::from_raw_parts_mut(constr, 1);
    constraints[0] = x1.powi(2) - 9.0;
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
    let number_of_constraints: i32 = 1;
    let mut initial_solution = vec![0.0; number_of_variables as usize];
    let mut initial_constraints = vec![0.0; number_of_constraints as usize];
    let intermediate_callback: prima_callback_t = Some(intermediate);
    let objective_and_constraints_callback: prima_objcon_t = Some(objective_and_constraints);

    let mut problem = new_problem(number_of_variables);
    problem.x0 = initial_solution.as_mut_ptr();
    problem.m_nlcon = number_of_constraints;
    problem.calcfc = objective_and_constraints_callback;

    let initial_objective = std::ptr::addr_of_mut!(problem.f0);
    unsafe {
        objective_and_constraints(
            initial_solution.as_ptr(),
            initial_objective,
            initial_constraints.as_mut_ptr(),
            std::ptr::null());
    }

    let mut options = new_options();
    options.iprint = prima_message_t_PRIMA_MSG_EXIT as i32;
    options.maxfun = 500 * number_of_variables;
    options.rhoend = 1e-6;
    options.callback = intermediate_callback;

    let mut result = prima_result_t::default();
    let result_ptr = std::ptr::addr_of_mut!(result);
    let _ = unsafe {
        prima_minimize(
        prima_algorithm_t_PRIMA_COBYLA,
        problem,
        options,
        result_ptr);
    };
    unsafe { prima_free_result(result_ptr); }
}
