#version 330

out vec4 color;

uniform int num_iterations;
uniform float x_shift;
uniform float y_shift;
uniform float zoom;

struct Complex {
    float a;
    float b;
};

Complex add_complex(Complex a, Complex b) {
    Complex res = Complex(a.a + b.a, a.b + b.b);
    return res;
}

Complex mul_complex(Complex a, Complex b) {
    Complex res = Complex(a.a*b.a - a.b*b.b, a.b*b.a + a.a*b.b);
    return res;
}

float sq_dist(Complex a) {
    return a.a*a.a + a.b*a.b;
}

int diverge(Complex a) {
    Complex z = Complex(0,0);
    for(int i = 0; i < num_iterations; i++) {
        z = add_complex(mul_complex(z,z), a);

        if(sq_dist(z) > 4) {
            return i;
        }

    }
    return num_iterations;
}

void main() {
    float x = (gl_FragCoord.x/zoom + x_shift) * 3;
    float y = (gl_FragCoord.y/zoom + y_shift) * 3;

    int it = diverge(Complex(x, y));
    if(it < num_iterations) {
        color = vec4(float(it*it)/num_iterations, float(it%19)/num_iterations, float(it%21)/num_iterations, 1.0);
    } else {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    }
}
