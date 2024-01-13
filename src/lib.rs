extern crate num_complex;
use std::ffi;

#[repr(C)]
pub enum PlannerFlags {
    Measure = 0,
    DestroyInput = 1,
    Unaligned = 2,
    ConserveMemory = 4,
    Exhaustive = 8,
    PreserveInput = 16,
    Patient = 32,
    Estimate = 64,
    WisdomOnly = 1 << 21,
}

pub type Complex = num_complex::Complex<f64>;

extern "C" {
    fn fftw_plan_dft_1d(
        n0: ffi::c_int,
        inp: *const Complex,
        outp: *mut Complex,
        sign: ffi::c_int,
        flags: ffi::c_uint,
    ) -> *mut ffi::c_void;

    fn fftw_execute(plan: *mut ffi::c_void);

    fn fftw_destroy_plan(plan: *mut ffi::c_void);
}

pub fn fftw_dft(data: &mut [Complex]) {
    let n0 = data.len();
    unsafe {
        let data = data.as_mut_ptr();
        let plan = fftw_plan_dft_1d(
            n0 as ffi::c_int,
            data,
            data,
            -1,
            PlannerFlags::Estimate as ffi::c_uint,
        );
        assert!(!plan.is_null());
        fftw_execute(plan);
        fftw_destroy_plan(plan);
    }
}

#[test]
fn test_try_fftw() {
    let unit = Complex::new(1.0, 0.0);
    let mut data = [Complex::new(0.0, 0.0); 16];
    data[0] = unit;
    fftw_dft(&mut data);
    for d in data {
        assert_eq!(unit, d);
    }
}
