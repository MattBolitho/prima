//! # PRIMA Native Bindings
//!
//! This crate contains the native C bindings for the
//! [PRIMA](https://github.com/libprima/prima) optimization library,
//! modern Fortran implementations of M. J. D. Powell's derivative-free
//! optimization methods.
//!
//! This crate contains the native C bindings to `primac`, with no idiomatic
//! Rust API. It does however contain functions that are needed to make the
//! PRIMA C APIs work in Rust whilst avoiding undefined behavior pitfalls.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for prima_problem_t {
    fn default() -> Self {
        prima_problem_t {
            n: Default::default(),
            calfun: Default::default(),
            calcfc: Default::default(),
            x0: std::ptr::null_mut(),
            xl: std::ptr::null_mut(),
            xu: std::ptr::null_mut(),
            m_ineq: Default::default(),
            Aineq: std::ptr::null_mut(),
            bineq: std::ptr::null_mut(),
            m_eq: Default::default(),
            Aeq: std::ptr::null_mut(),
            beq: std::ptr::null_mut(),
            m_nlcon: Default::default(),
            f0: Default::default(),
            nlconstr0: std::ptr::null_mut(),
        }
    }
}

impl Default for prima_options_t {
    fn default() -> Self {
        prima_options_t {
            rhobeg: f64::NAN,
            rhoend: f64::NAN,
            maxfun: Default::default(),
            iprint: prima_message_t_PRIMA_MSG_NONE as i32,
            ftarget: f64::NEG_INFINITY,
            npt: Default::default(),
            ctol: f64::NAN,
            data: std::ptr::null_mut(),
            callback: Default::default(),
        }
    }
}

impl Default for prima_result_t {
    fn default() -> Self {
        prima_result_t {
            x: std::ptr::null_mut(),
            f: Default::default(),
            cstrv: Default::default(),
            nlconstr: std::ptr::null_mut(),
            nf: Default::default(),
            status: prima_rc_t_PRIMA_RC_DFT as i32,
            success: false,
            message: std::ptr::null(),
        }
    }
}

/// Gets a new, initialized problem with the given number of variables.
/// `prima_init_problem` will have been called on the return value.
///
/// This function is exported on top of the C bindings because the PRIMA API
/// uses an uninitialized struct function call pattern to to initialize problem
/// structs:
///
/// ```c
/// const int n = 2;
/// prima_problem_t problem;
/// prima_init_problem(&problem, n);
/// ```
///
/// In Rust, structs are always fully initialized, so we provide this function
/// as convenience to correctly initialize a problem whilst avoiding any
/// potential undefined behavior and/or excessive repetition.
///
/// ## Example
///
/// ```rust///
/// let number_of_variables: i32 = 2;
/// let mut problem = prima_sys::new_problem(number_of_variables);
/// // Add your further problem initialization here...
/// ```
///
/// ## Parameters
/// - `number_of_variables`: The number of variables in the problem.
///
/// ## Returns
/// A new, initialized `prima_problem_t` instance..
pub fn new_problem(number_of_variables: i32) -> prima_problem_t {
    let mut problem = Default::default();
    let problem_ptr = std::ptr::addr_of_mut!(problem);
    unsafe {
        prima_init_problem(problem_ptr, number_of_variables as i32);
    }
    problem
}

/// Gets a new, initialized options struct. `prima_init_options` will have been
/// called on the return value.
///
/// This function is exported on top of the C bindings because the PRIMA API
/// uses an uninitialized struct function call pattern to to initialize problem
/// structs:
///
/// ```c
/// prima_options_t options;
/// prima_init_options(&options);
/// ```
///
/// In Rust, structs are always fully initialized, so we provide this function
/// as convenience to correctly initialize options whilst avoiding any
/// potential undefined behavior and/or excessive repetition.
///
/// ## Example
///
/// ```rust
/// let options = prima_sys::new_options();
/// // Add your further options initialization here...
/// ```
///
/// ## Returns
/// A new, initialized `prima_options_t` instance.
pub fn new_options() -> prima_options_t {
    let mut options = Default::default();
    let options_ptr = std::ptr::addr_of_mut!(options);
    unsafe {
        prima_init_options(options_ptr);
    }
    options
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_problem_contains_expected_number_of_variables() {
        let number_of_variables: i32 = 2;
        let problem = new_problem(number_of_variables);
        assert_eq!(number_of_variables, problem.n);
    }
}
