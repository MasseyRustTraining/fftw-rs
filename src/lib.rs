use num_complex::Complex;
use std::ffi;

pub type FftwComplex = Complex<f64>;

extern "C" {
    fn fftw_plan_dft_1d(
        n0: ffi::c_int,
        inp: *const FftwComplex,
        outp: *mut FftwComplex,
        sign: ffi::c_int,
        flags: ffi::c_uint,
    ) -> *mut ffi::c_void;

    fn fftw_execute(plan: *mut ffi::c_void);

    fn fftw_destroy_plan(plan: *mut ffi::c_void);
}

pub fn fftw_forward_dft(inp: &[FftwComplex], outp: &mut [FftwComplex]) {
    let n0 = inp.len();
    assert!(outp.len() == inp.len());
    unsafe {
        let plan = fftw_plan_dft_1d(
            n0 as ffi::c_int,
            inp.as_ptr(),
            outp.as_mut_ptr(),
            -1,
            0,
        );
        assert!(!plan.is_null());
        fftw_execute(plan);
        fftw_destroy_plan(plan);
    }
}

#[test]
fn test_try_fftw() {
    let inp = [Complex::new(0.0, 0.0); 16];
    let mut outp = [Complex::default(); 16];
    fftw_forward_dft(&inp, &mut outp);
}
