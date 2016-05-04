pub mod gl330 {
    // fragment shader
    pub const FRAG: &'static str =
        "
        #version 330
        #define PI 3.14159265358979323846

        // Inputs
        in vec3 g_normal;
        vec3 fNormal;

        // Material
        float reflectance = 1.0; // 0 to 1
        float roughness = 0.5;
        vec3 specularColor = vec3(1.0, 1.0, 1.0); // f0

        // Values
        vec3 lightVector = vec3(1, 1, 1); // Light (l)
        vec3 eyeVector = vec3(2.75, 1.25, 1.25); // Camera (v)
        vec3 halfVector = normalize(lightVector + eyeVector); // L + V / |L + V|

        out vec4 fColor; // Output Color

        // Specular Functions
        vec3 D(vec3 h) { // Normal Distribution Function - GGX/Trowbridge-Reitz
            float alpha = roughness * roughness;
            float alpha2 = alpha * alpha;
            float NoH = dot(fNormal, h);
            float finalTerm = ((NoH * NoH) * (alpha2 - 1.0) + 1.0);
            return vec3(alpha2 / (PI * (finalTerm * finalTerm)));
        }
        vec3 Gsub(vec3 v) { // Sub Function of G
            float k = ((roughness + 1.0) * (roughness + 1.0)) / 8;
            return vec3(dot(fNormal, v) / ((dot(fNormal, v)) * (1.0 - k) + k));
        }
        vec3 G(vec3 l, vec3 v, vec3 h) { // Geometric Attenuation Term - Schlick Modified (k = a/2)
            return Gsub(l) * Gsub(v);
        }
        vec3 F(vec3 v, vec3 h) { // Fresnel - Schlick Modified (Spherical Gaussian Approximation)
            vec3 f0 = specularColor; // right?
            return f0 + (1.0 - f0) * pow(2, (-5.55473 * (dot(v, h)) - 6.98316) * (dot(v, h)));
        }

        vec3 specular() {
            return (D(halfVector) * F(eyeVector, halfVector) * G(lightVector, eyeVector, halfVector)) / 4 * ((dot(fNormal, lightVector)) * (dot(fNormal, eyeVector)));
        }
        vec3 diffuse() {
            float NoL = dot(fNormal, lightVector);
            vec3 result = vec3(reflectance / PI);
            return result * NoL;
        }
        void main() {
            fNormal = normalize(g_normal);
            fColor = vec4(diffuse() + specular(), 1.0);
            //fColor = vec4(D(halfVector), 1.0);
        }
    ";
}
