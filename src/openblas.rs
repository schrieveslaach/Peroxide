#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/openblas.rs"));

pub fn dgecon(
    norm: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
}

pub fn dgeqrf(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
}

pub fn dgetrf(m: i32, n: i32, a: &mut [f64], lda: i32, ipiv: &mut [i32], info: &mut i32) {}

pub fn dgetri(
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
}

pub fn dgetrs(
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
}

pub fn dorgqr(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
}
