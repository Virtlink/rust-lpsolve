Wrapper for lpsolve

![Build status](https://gitlab.com/cmr/rust-lpsolve/badges/master/build.svg)
[![Crates.io](https://img.shields.io/crates/v/lpsolve.svg)](https://crates.io/crates/lpsolve)

lpsolve is a free software (LGPL) solver for mixed integer linear programming problems. The
documentation here is nonexistent when it comes to understanding how to model systems or
precisely what the consequences of various methods are.  The [upstream
documentation](http://lpsolve.sourceforge.net/5.5/) for lpsolve is much more comprehensive.

This wrapper is mostly straightforward.

The performance of lpsolve is mediocre compared to commercial solvers and some other free
software solvers. Performance here is how how long it takes to solve [benchmark
models](http://plato.asu.edu/bench.html).

If you need help chosing a solver, the following is an excellent report:

http://prod.sandia.gov/techlib/access-control.cgi/2013/138847.pdf

# Boolean return values

The boolean return values represent the underlying return value from lpsolve. `true` means
success and `false` means some error occured. There is an error reporting API, although by
default it logs to standard out, and is not yet wrapped.

# Status

This wrapper is not complete. In particular, none of the solver setting or debug functions are
wrapped. Additionally, a few of the model building and solution extraction functions are not
wrapped.

This is not fundamental, merge requests welcome!

# Stability

`lpsolve-sys` is versioned separately from this wrapper. This wrapper is provisionally
unstable, but the functions that are currently wrapped are not likely to change.
