#version 330

out vec4 color;

uniform int num_iterations;
uniform float x_shift;
uniform float y_shift;
uniform float zoom;

uniform int window_size;

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

vec3 diverge(Complex a) {
    Complex z = Complex(0,0);
    int i;
    for(i = 0; i < num_iterations; i++) {
        z = add_complex(mul_complex(z,z), a);

        if(sq_dist(z) > 9) {
            break;
        }

    }
    vec3 res;
    res.x = i + 1 - log(log(sq_dist(z))) / log(2.0);
    res.x = 0.2f*res.x/num_iterations;

    res.y = i + 1 - log(log(sqrt(sq_dist(z)))) / log(2.0);
    res.y = res.y/num_iterations;

    res.z = i + 1 - log(log(sq_dist(z))) / log(2.0);
    res.z = 0.5f*res.z/num_iterations;

    if(i == num_iterations) {
         res = vec3(0.0f);
    }

    return res;
}

void main() {
    float x = ((gl_FragCoord.x/window_size)-0.5)/zoom + x_shift;
    float y = ((gl_FragCoord.y/window_size)-0.5)/zoom + y_shift;

    vec3 c = diverge(Complex(x, y));
    color = vec4(c, 1.0);
}
