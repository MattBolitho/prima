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
