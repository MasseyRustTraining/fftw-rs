#include <fftw3.h>

void dft(int n0, fftw_complex *in, fftw_complex *out, int sign, unsigned flags) {
    fftw_plan plan = fftw_plan_dft_1d(n0, in, out, sign, flags);
    fftw_execute(plan);
    fftw_destroy_plan(plan);
}

fftw_complex data[16];

int main() {
    data[0][0] = 1.0;
    dft(16, data, data, -1, FFTW_ESTIMATE);
    for (int i = 0; i < 16; i++) {
        printf("%f+%fi\n", data[i][0], data[i][1]);
    }
    return 0;
}
