//! Wrapper for lpsolve
//!
//! ![Build status](https://gitlab.com/cmr/rust-lpsolve/badges/master/build.svg)
//! [![Crates.io](https://img.shields.io/crates/v/lpsolve.svg)](https://crates.io/crates/lpsolve)
//!
//! lpsolve is a free software (LGPL) solver for mixed integer linear programming problems. The
//! documentation here is nonexistent when it comes to understanding how to model systems or
//! precisely what the consequences of various methods are.  The [upstream
//! documentation](http://lpsolve.sourceforge.net/5.5/) for lpsolve is much more comprehensive.
//!
//! This wrapper is mostly straightforward.
//!
//! The performance of lpsolve is mediocre compared to commercial solvers and some other free
//! software solvers. Performance here is how how long it takes to solve [benchmark
//! models](http://plato.asu.edu/bench.html).
//! 
//! If you need help chosing a solver, the following is an excellent report:
//!
//! http://prod.sandia.gov/techlib/access-control.cgi/2013/138847.pdf
//!
//! # Boolean return values
//!
//! The boolean return values represent the underlying return value from lpsolve. `true` means
//! success and `false` means some error occured. There is an error reporting API, although by
//! default it logs to standard out, and is not yet wrapped.
//!
//! # Status
//!
//! This wrapper is not complete. In particular, none of the solver setting or debug functions are
//! wrapped. Additionally, a few of the model building and solution extraction functions are not
//! wrapped.
//!
//! This is not fundamental, merge requests welcome!
//!
//! # Stability
//!
//! `lpsolve-sys` is versioned separately from this wrapper. This wrapper is provisionally
//! unstable, but the functions that are currently wrapped are not likely to change.
//!
//! # License
//! 
//! This crate and `lpsolve-sys` are licensed under either of
//! 
//!  * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//!  * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//! 
//! at your option. However, please note that lpsolve itself is LGPL. The default configuration right
//! now builds a bundled copy of lpsolve and links to it statically.


extern crate lpsolve_sys as lp;
extern crate libc;
#[macro_use] extern crate bitflags;

use std::mem::transmute;
use std::io::Write;
use std::ffi::CStr;
use std::ops::Deref;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Verbosity {
    Neutral = 0,
    Critical = 1,
    Severe = 2,
    Important = 3,
    Normal = 4,
    Detailed = 5,
    Full = 6,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ConstraintType {
    Le = 1,
    Eq = 3,
    Ge = 2,
    Free = 0,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum SOSType {
    Type1 = 1,
    Type2 = 2,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum VarType {
    Binary = 1,
    Float = 0,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum BoundsMode {
    Restrictive = 1,
    None = 0,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum SolveStatus {
    OutOfMemory = -2,
    NotRun = -1,
    Optimal = 0,
    Suboptimal = 1,
    Infeasible = 2,
    Unbounded = 3,
    Degenerate = 4,
    NumericalFailure = 5,
    UserAbort = 6,
    Timeout = 7,
    Presolved = 8,
    ProcFail = 9,
    ProcBreak = 11,
    FeasibleFound = 12,
    NoFeasibleFound = 13,
}

bitflags! {
    pub flags MPSOptions: ::libc::c_int {
        const CRITICAL = 1,
        const SEVERE = 2,
        const IMPORTANT = 3,
        const NORMAL = 4,
        const DETAILED = 5,
        const FULL = 6,
        const FREE = 8,
        const IBM = 16,
        const NEGOBJCONST = 32,
    }
}

/// A linear programming problem.
pub struct Problem {
    lprec: *mut lp::lprec,
}

macro_rules! cptr {
    ($e:expr) => { if $e.is_null() { None } else { Some(Problem { lprec: $e }) } }
}

#[cfg(not(windows))]
unsafe extern "C" fn write_modeldata(val: *mut libc::c_void, buf: *mut libc::c_char) -> libc::c_int {
    let val = transmute::<_, &mut &mut Write>(val);
    let buf = CStr::from_ptr(buf);
    match val.write(buf.to_bytes()) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

#[cfg(windows)]
unsafe extern "stdcall" fn write_modeldata(val: *mut libc::c_void, buf: *mut libc::c_char) -> libc::c_int {
    let val = transmute::<_, &mut &mut Write>(val);
    let buf = CStr::from_ptr(buf);
    match val.write(buf.to_bytes()) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

impl Problem {

    /// Initialize an empty problem with space for `rows` and `cols`.
    pub fn new(rows: libc::c_int, cols: libc::c_int) -> Option<Problem> {
        let ptr = unsafe { lp::make_lp(rows, cols) };
        cptr!(ptr)
    }

    /// Reads an lp-format model from `path`.
    pub fn read_lp<P: Deref<Target=CStr>, C: Deref<Target=CStr>>(path: &P, verbosity: Verbosity, initial_name: &C) -> Option<Problem> {
        let ptr = unsafe { lp::read_LP(path.as_ptr() as *mut _, verbosity as libc::c_int, initial_name.as_ptr() as *mut _) };
        cptr!(ptr)
    }

    /// Read an mps-format model from `path` using the "free" formatting.
    pub fn read_freemps<P: Deref<Target=CStr>>(path: &P, options: MPSOptions) -> Option<Problem> {
        let ptr = unsafe { lp::read_freeMPS(path.as_ptr() as *mut _, options.bits) };
        cptr!(ptr)
    }

    /// Read an mps-format model from `path` using the fixed formatting.
    pub fn read_fixedmps<P: Deref<Target=CStr>>(path: &P, options: MPSOptions) -> Option<Problem> {
        let ptr = unsafe { lp::read_MPS(path.as_ptr() as *mut _, options.bits) };
        cptr!(ptr)
    }

    /// Write an lp-format model into `out`.
    ///
    /// If there are any errors writing to `out`, `false` will be returned. Otherwise, `true`.
    pub fn write_lp(&self, out: &mut Write) -> bool {
        1 == unsafe { lp::write_lpex(self.lprec, transmute::<_, *mut libc::c_void>(&out), write_modeldata) }
    }

    /// Write an mps-format model into `out` using the fixed formatting.
    ///
    /// If there are any errors writing to `out`, `false` will be returned. Otherwise, `true`.
    pub fn write_fixedmps(&self, out: &mut Write) -> bool {
        self.write_mps(out, 1)
    }

    /// Write an mps-format model into `out` using the "free" formatting.
    ///
    /// If there are any errors writing to `out`, `false` will be returned. Otherwise, `true`.
    pub fn write_freemps(&self, out: &mut Write) -> bool {
        self.write_mps(out, 2)
    }

    /// Write an mps-format model into `out` using `formatting`.
    ///
    /// If there are any errors writing to `out`, `false` will be returned. Otherwise, `true`.
    ///
    /// `formatting` must be 1 for fixed or 2 for free.
    pub fn write_mps(&self, out: &mut Write, formatting: libc::c_int) -> bool {
        debug_assert!(formatting == 1 || formatting == 2);
        1 == unsafe { lp::MPS_writefileex(self.lprec, formatting, transmute::<_, *mut libc::c_void>(&out), write_modeldata) }
    }

    /// Reserve enough memory for `rows` and `cols`.
    ///
    /// If `rows` or `cols` are less than the current number of rows or columns, the additional
    /// rows and columns will be deleted.
    ///
    /// Returns `true` if successful.
    pub fn resize(&mut self, rows: libc::c_int, cols: libc::c_int) -> bool {
        1 == unsafe { lp::resize_lp(self.lprec, rows, cols) }
    }

    /// Add a column to the model.
    ///
    /// If `values` is empty, an all-zero column will be added.
    ///
    /// This will assert that `values` has at least as many elements as the underlying model.
    pub fn add_column(&mut self, values: &[f64]) -> bool {
        assert!(values.len() == self.num_rows() as usize + 1);
        1 == unsafe { lp::add_column(self.lprec, values.as_ptr() as *mut _) }
    }

    /// Add a column to the model, scattering `values` by `indices`.
    ///
    /// The values for the column are taken from `values`. The value from `values[i]` will be
    /// placed into row `indices[i]`. The length used is the max of the lengths of `values` and
    /// `indices`. There is a debug_assert that these are equal.
    pub fn add_column_scatter(&mut self, values: &[f64], indices: &[libc::c_int]) -> bool {
        debug_assert!(values.len() == indices.len());
        let len = std::cmp::max(values.len(), indices.len());
        1 == unsafe { lp::add_columnex(self.lprec, len as libc::c_int, values.as_ptr() as *mut _, indices.as_ptr() as *mut _) }
    }

    /// Read a column from the model.
    pub fn get_column(&self, values: &mut [f64], column: libc::c_int) -> bool {
        assert!(values.len() >= self.num_rows() as usize + 1);
        1 == unsafe { lp::get_column(self.lprec, column, values.as_mut_ptr()) }
    }

    /// Read a row from the model.
    pub fn get_row(&self, values: &mut [f64], row: libc::c_int) -> bool {
        assert!(values.len() >= self.num_cols() as usize + 1);
        1 == unsafe { lp::get_row(self.lprec, row, values.as_mut_ptr()) }
    }

    /// Sets the verbosity of the output.
    pub fn set_verbose(&mut self, verbosity: Verbosity) {
        unsafe { lp::set_verbose(self.lprec, verbosity as libc::c_int) }
    }

    /// Sets the objective to maximize R0.
    pub fn set_maxim(&mut self) {
        unsafe { lp::set_maxim(self.lprec) }
    }

    /// Sets the objective to minimize R0.
    pub fn set_minim(&mut self) {
        unsafe { lp::set_minim(self.lprec) }
    }

    /// Gets the value of the objective function.
    pub fn get_objective(&self) -> f64 {
        unsafe { lp::get_objective(self.lprec) }
    }

    /// Add a constraint to the model.
    /// 
    /// The constraint is that `coeffs * vars OP target`, where `OP` is specified by `kind`.
    /// 
    /// For optimal performance, use the `matrix_builder` method and add the objective function
    /// first. This method is otherwise very slow for large models.
    ///
    /// Asserts that `coeffs` has at least as many elements as the underlying model.
    pub fn add_constraint(&mut self, coeffs: &[f64], target: f64, kind: ConstraintType) -> bool {
        assert!(coeffs.len() >= self.num_cols() as usize + 1);
        1 == unsafe { lp::add_constraint(self.lprec, coeffs.as_ptr() as *mut _, kind as libc::c_int, target) }
    }

    /// Add a [Special Ordered Set](http://lpsolve.sourceforge.net/5.5/SOS.htm) constraint.
    ///
    /// The `weights` are scattered by `variables`, that is, `weights[i]` will be specified for
    /// column `variables[i]`.
    ///
    /// The length used is the max of the lengths of `values` and `indices`. There is a
    /// debug_assert that these are equal.
    pub fn add_sos_constraint(&mut self, name: &CStr, sostype: SOSType, priority: libc::c_int,
                              weights: &[f64], variables: &[libc::c_int]) -> bool {
        let len = std::cmp::max(weights.len(), variables.len());
        debug_assert!(weights.len() == variables.len());
        0 != unsafe { lp::add_SOS(self.lprec, name.as_ptr() as *mut _, sostype as libc::c_int, priority, 
                             len as libc::c_int, variables.as_ptr() as *mut _, weights.as_ptr() as *mut _) }
    }

    /// Delete a column from the model.
    ///
    /// The other columns are shifted leftward. `col` cannot be 0, as that column represents the
    /// RHS, which must always be present.
    pub fn del_column(&mut self, col: libc::c_int) -> bool {
        1 == unsafe { lp::del_column(self.lprec, col) }
    }

    /// Delete a constraint from the model.
    ///
    /// The other constraints are shifted upwards. `row` cannot be 0, as that row represents the
    /// objective function, which must always be present.
    pub fn del_constraint(&mut self, row: libc::c_int) -> bool {
        1 == unsafe { lp::del_constraint(self.lprec, row) }
    }

    /// Returns `Some(true)` if the specified column can be negative, `Some(false)` otherwise.
    ///
    /// If the column is out-of-bounds, `None` is returned.
    pub fn is_negative(&self, col: libc::c_int) -> Option<bool> {
        if col > self.num_cols()+1 {
            None
        } else {
            Some(unsafe { lp::is_negative(self.lprec, col) } == 1)
        }
    }

    /// Set a variable to be either binary or floating point.
    pub fn set_variable_type(&mut self, col: libc::c_int, vartype: VarType) -> bool {
        1 == unsafe { lp::set_binary(self.lprec, col, vartype as libc::c_uchar) }
    }

    /// Get the type of a variable, `None` if `col` is out of bounds.
    pub fn get_variable_type(&self, col: libc::c_int) -> Option<VarType> {
        if col > self.num_cols()+1 {
            None
        } else {
            let res = if unsafe { lp::is_binary(self.lprec, col) } == 1 {
                VarType::Binary
            } else {
                VarType::Float
            };
            Some(res)
        }
    }

    /// Set the upper and lower bounds of a variable.
    pub fn set_bounds(&mut self, col: libc::c_int, lower: f64, upper: f64) -> bool {
        1 == unsafe { lp::set_bounds(self.lprec, col, lower, upper) }
    }

    /// Set the bounds mode to 'tighten'.
    ///
    /// If the bounds mode is `true`, then when `set_bounds`, `set_lower_bound`, or
    /// `set_upper_bound` is used and the provided bound is less restrictive than the current bound
    /// (ie, its absolute value is larger), then the request will be ignored. However, if the new
    /// bound is more restrictive (ie, its absolute value is smaller) the request will go through.
    ///
    /// If the bounds mode is `false`, the bounds will always be set as specified.
    pub fn set_bounds_mode(&mut self, tighten: BoundsMode) {
        unsafe { lp::set_bounds_tighter(self.lprec, tighten as libc::c_uchar) };
    }

    /// Get the bounds mode.
    ///
    /// See `set_bounds_mode` for what this value means.
    pub fn get_bounds_mode(&self) -> BoundsMode {
        if unsafe { lp::get_bounds_tighter(self.lprec) } == 1 {
            BoundsMode::Restrictive
        } else {
            BoundsMode::None
        }
    }

    /// Set the type of a constraint.
    pub fn set_constraint_type(&mut self, row: libc::c_int, contype: ConstraintType) -> bool {
        1 == unsafe { lp::set_constr_type(self.lprec, row, contype as libc::c_int) }
    }

    /// Get the type of a constraint, or `None` if out of bounds or another error occurs.
    pub fn get_constraint_type(&self, row: libc::c_int) -> Option<ConstraintType> {
        if row > self.num_rows()+1 {
            None
        } else {
            let res = unsafe { lp::get_constr_type(self.lprec, row) };
            match res {
                1 => Some(ConstraintType::Le),
                2 => Some(ConstraintType::Ge),
                3 => Some(ConstraintType::Eq),
                _ => None,
            }
        }
    }

    /// Set a variable to be unbounded.
    pub fn set_unbounded(&mut self, col: libc::c_int) -> bool {
        1 == unsafe { lp::set_unbounded(self.lprec, col) }
    }

    /// Check if a variable is unbounded.
    pub fn is_unbounded(&self, col: libc::c_int) -> Option<bool> {
        if col > self.num_cols()+1 {
            None
        } else {
            Some(unsafe { lp::is_unbounded(self.lprec, col) } == 1)
        }
    }

    /// Set the practical value for "infinite"
    ///
    /// This is the bound of the absolute value of all variables. If the absolute value of a
    /// variable is larger than this, it is considered to have diverged.
    pub fn set_infinite(&mut self, infinity: f64) {
        unsafe { lp::set_infinite(self.lprec, infinity) };
    }

    /// Get the value of "infinite"
    ///
    /// See set_infinite for more details.
    pub fn get_infinite(&self) -> f64 {
        unsafe { lp::get_infinite(self.lprec) }
    }

    /// Set a variable's integer type.
    pub fn set_integer(&mut self, col: libc::c_int, must_be_integer: bool) -> bool {
        1 == unsafe { lp::set_int(self.lprec, col, if must_be_integer { 1 } else { 0 }) }
    }

    /// Check if a variable is an integer.
    pub fn is_integer(&self, col: libc::c_int) -> Option<bool> {
        if col > self.num_cols()+1 {
            None
        } else {
            Some(unsafe { lp::is_int(self.lprec, col) } == 1)
        }
    }

    /// Sets the objective function.
    ///
    /// Asserts that `coeffs` has at least as many elements as the underlying model.
    pub fn set_objective_function(&mut self, coeffs: &[f64]) -> bool {
        assert!(coeffs.len() >= self.num_cols() as usize + 1);
        1 == unsafe { lp::set_obj_fn(self.lprec, coeffs.as_ptr() as *mut _) }
    }

    /// Scatters `coeffs` into the objective function coefficients with `indices`.
    ///
    /// The length used is the max of the lengths of `coeffs` and `indices`. There is a
    /// debug_assert that these are equal.
    pub fn scatter_objective_function(&mut self, coeffs: &[f64], indices: &[libc::c_int]) -> bool {
        let len = std::cmp::max(coeffs.len(), indices.len());
        debug_assert!(coeffs.len() == indices.len());
        1 == unsafe { lp::set_obj_fnex(self.lprec, len as libc::c_int, coeffs.as_ptr() as *mut _, indices.as_ptr() as *mut _) }
    }

    /// Sets the range of a constraint.
    ///
    /// If the constraint type is `<=`, then the "actual" constraint will be `RHS - range <= v <= RHS`.
    ///
    /// If the constraint type is `>=`, then the "actual" constraint will be `RHS <= v <= RHS + range`.
    ///
    /// This puts a bound on the constraint and can give the solver more freedom, and is more
    /// efficient than adding an extra constraint.
    pub fn set_constraint_range(&mut self, row: libc::c_int, range: f64) -> bool {
        1 == unsafe { lp::set_rh_range(self.lprec, row, range) }
    }

    /// Get the range on a constraint if one is set, otherwise `None`.
    pub fn get_constraint_range(&self, row: libc::c_int) -> Option<f64> {
        if row > self.num_rows() + 1 {
            None
        } else {
            let delta = unsafe { lp::get_rh_range(self.lprec, row) };
            if delta == self.get_infinite() {
                None
            } else {
                Some(delta)
            }
        }
    }

    /// Solve the model.
    pub fn solve(&mut self) -> SolveStatus {
        use SolveStatus::*;
        match unsafe { lp::solve(self.lprec) } {
            -2 => OutOfMemory,
            -1 => NotRun,
            0 => Optimal,
            1 => Suboptimal,
            2 => Infeasible,
            3 => Unbounded,
            4 => Degenerate,
            5 => NumericalFailure,
            6 => UserAbort,
            7 => Timeout,
            9 => Presolved,
            10 => ProcFail,
            11 => ProcBreak,
            12 => FeasibleFound,
            13 => NoFeasibleFound,
            status => panic!("unknown solve status {}", status)
        }
    }

    /// Read out the values assigned to variables from the most recent `solve`.
    ///
    /// Returns `None` if `vars` does not have at least as many elements as the underlying model
    /// has columns. Otherwise, returns `Some` with the slice truncated to the number of columns.
    pub fn get_solution_variables<'a>(&self, vars: &'a mut [f64]) -> Option<&'a mut [f64]> {
        let cols = self.num_cols();
        if vars.len() < cols as usize {
            None
        } else {
            unsafe { lp::get_variables(self.lprec, vars.as_mut_ptr()) };
            Some(&mut vars[..cols as usize])
        }
    }

    /// Construct a wrapper for a pre-existing `lprec`.
    ///
    /// This is unsafe as the pointer is not null-checked etc.
    pub unsafe fn from_lprec(lprec: *mut lp::lprec) -> Problem {
        Problem {
            lprec: lprec
        }
    }

    /// Get the `lprec` that this wraps.
    ///
    /// Don't `delete_lp` it, please.
    pub fn to_lprec(&self) -> *mut lp::lprec {
        self.lprec
    }

    pub fn num_cols(&self) -> libc::c_int {
        unsafe { lp::get_Ncolumns(self.lprec) }
    }

    pub fn num_rows(&self) -> libc::c_int {
        unsafe { lp::get_Nrows(self.lprec) }
    }
}

impl Drop for Problem {
    fn drop(&mut self) {
        unsafe { lp::delete_lp(self.lprec) }
    }
}

impl Clone for Problem {
    fn clone(&self) -> Problem {
        let ptr = unsafe { lp::copy_lp(self.lprec) };
        if ptr.is_null() {
            panic!("OOM when trying to copy_lp")
        }
        Problem { lprec: ptr }
    }
}

unsafe impl Send for Problem { }

#[cfg(test)]
mod tests {
    use Problem;
    #[test]
    fn smoke() {
        let mut lp = Problem::new(0, 0).unwrap();
        assert_eq!(lp.solve(), ::SolveStatus::NotRun);
    }
}
