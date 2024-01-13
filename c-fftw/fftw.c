#include <fftw3.h>

void dft(int n0, fftw_complex *in, fftw_complex *out, int sign, unsigned flags) {
    fftw_plan plan = fftw_plan_dft_1d(n0, in, out, sign, flags);
    fftw_execute(plan);
    fftw_destroy_plan(plan);
}

fftw_complex in[16];
fftw_complex out[16];

int main() {
    in[0][0] = 1.0;
    dft(16, in, out, -1, 0);
    for (int i = 0; i < 16; i++) {
        printf("%f+%fi\n", out[i][0], out[i][1]);
    }
    return 0;
}
