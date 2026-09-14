// glsl_validator - docs.rs
// Comprehensive OpenGL 4.6 / GLSL built-in function database with docs.gl documentation

pub struct BuiltinOverload {
    pub label: &'static str,
    pub params: &'static [&'static str],
}

pub struct BuiltinFunction {
    pub name: &'static str,
    pub description: &'static str,
    pub overloads: &'static [BuiltinOverload],
}

static BUILTIN_FUNCTIONS: &[BuiltinFunction] = &[
    // ------------------------------------------------------------------------
    // Geometric Functions
    // ------------------------------------------------------------------------
    BuiltinFunction {
        name: "length",
        description: "### `length`\n*docs.gl / OpenGL 4.6*\n\nCalculates the length of a vector.\n\n$$\\text{length}(x) = \\sqrt{x[0]^2 + x[1]^2 + \\dots}$$\n\n**Parameters:**\n* `x`: Specifies a vector to calculate the length of.",
        overloads: &[
            BuiltinOverload { label: "float length(float x)", params: &["float x"] },
            BuiltinOverload { label: "float length(vec2 x)", params: &["vec2 x"] },
            BuiltinOverload { label: "float length(vec3 x)", params: &["vec3 x"] },
            BuiltinOverload { label: "float length(vec4 x)", params: &["vec4 x"] },
            BuiltinOverload { label: "double length(double x)", params: &["double x"] },
            BuiltinOverload { label: "double length(dvec2 x)", params: &["dvec2 x"] },
            BuiltinOverload { label: "double length(dvec3 x)", params: &["dvec3 x"] },
            BuiltinOverload { label: "double length(dvec4 x)", params: &["dvec4 x"] },
        ],
    },
    BuiltinFunction {
        name: "distance",
        description: "### `distance`\n*docs.gl / OpenGL 4.6*\n\nCalculates the distance between two points.\n\n$$\\text{distance}(p0, p1) = \\text{length}(p0 - p1)$$\n\n**Parameters:**\n* `p0`: Specifies the first point.\n* `p1`: Specifies the second point.",
        overloads: &[
            BuiltinOverload { label: "float distance(float p0, float p1)", params: &["float p0", "float p1"] },
            BuiltinOverload { label: "float distance(vec2 p0, vec2 p1)", params: &["vec2 p0", "vec2 p1"] },
            BuiltinOverload { label: "float distance(vec3 p0, vec3 p1)", params: &["vec3 p0", "vec3 p1"] },
            BuiltinOverload { label: "float distance(vec4 p0, vec4 p1)", params: &["vec4 p0", "vec4 p1"] },
            BuiltinOverload { label: "double distance(double p0, double p1)", params: &["double p0", "double p1"] },
            BuiltinOverload { label: "double distance(dvec2 p0, dvec2 p1)", params: &["dvec2 p0", "dvec2 p1"] },
            BuiltinOverload { label: "double distance(dvec3 p0, dvec3 p1)", params: &["dvec3 p0", "dvec3 p1"] },
            BuiltinOverload { label: "double distance(dvec4 p0, dvec4 p1)", params: &["dvec4 p0", "dvec4 p1"] },
        ],
    },
    BuiltinFunction {
        name: "dot",
        description: "### `dot`\n*docs.gl / OpenGL 4.6*\n\nCalculates the dot product (scalar product) of two vectors.\n\n$$\\text{dot}(x, y) = x[0] \\cdot y[0] + x[1] \\cdot y[1] + \\dots$$\n\n**Parameters:**\n* `x`: Specifies the first vector.\n* `y`: Specifies the second vector.",
        overloads: &[
            BuiltinOverload { label: "float dot(float x, float y)", params: &["float x", "float y"] },
            BuiltinOverload { label: "float dot(vec2 x, vec2 y)", params: &["vec2 x", "vec2 y"] },
            BuiltinOverload { label: "float dot(vec3 x, vec3 y)", params: &["vec3 x", "vec3 y"] },
            BuiltinOverload { label: "float dot(vec4 x, vec4 y)", params: &["vec4 x", "vec4 y"] },
            BuiltinOverload { label: "double dot(double x, double y)", params: &["double x", "double y"] },
            BuiltinOverload { label: "double dot(dvec2 x, dvec2 y)", params: &["dvec2 x", "dvec2 y"] },
            BuiltinOverload { label: "double dot(dvec3 x, dvec3 y)", params: &["dvec3 x", "dvec3 y"] },
            BuiltinOverload { label: "double dot(dvec4 x, dvec4 y)", params: &["dvec4 x", "dvec4 y"] },
        ],
    },
    BuiltinFunction {
        name: "cross",
        description: "### `cross`\n*docs.gl / OpenGL 4.6*\n\nCalculates the cross product of two 3D vectors.\n\n$$\\text{cross}(x, y) = \\begin{pmatrix} x[1]y[2] - y[1]x[2] \\\\ x[2]y[0] - y[2]x[0] \\\\ x[0]y[1] - y[0]x[1] \\end{pmatrix}$$\n\n**Parameters:**\n* `x`: Specifies the first vector.\n* `y`: Specifies the second vector.",
        overloads: &[
            BuiltinOverload { label: "vec3 cross(vec3 x, vec3 y)", params: &["vec3 x", "vec3 y"] },
            BuiltinOverload { label: "dvec3 cross(dvec3 x, dvec3 y)", params: &["dvec3 x", "dvec3 y"] },
        ],
    },
    BuiltinFunction {
        name: "normalize",
        description: "### `normalize`\n*docs.gl / OpenGL 4.6*\n\nCalculates the unit vector in the same direction as the original vector.\n\n$$\\text{normalize}(x) = \\frac{x}{\\text{length}(x)}$$\n\n**Parameters:**\n* `x`: Specifies the vector to normalize.",
        overloads: &[
            BuiltinOverload { label: "float normalize(float x)", params: &["float x"] },
            BuiltinOverload { label: "vec2 normalize(vec2 x)", params: &["vec2 x"] },
            BuiltinOverload { label: "vec3 normalize(vec3 x)", params: &["vec3 x"] },
            BuiltinOverload { label: "vec4 normalize(vec4 x)", params: &["vec4 x"] },
            BuiltinOverload { label: "double normalize(double x)", params: &["double x"] },
            BuiltinOverload { label: "dvec2 normalize(dvec2 x)", params: &["dvec2 x"] },
            BuiltinOverload { label: "dvec3 normalize(dvec3 x)", params: &["dvec3 x"] },
            BuiltinOverload { label: "dvec4 normalize(dvec4 x)", params: &["dvec4 x"] },
        ],
    },
    BuiltinFunction {
        name: "faceforward",
        description: "### `faceforward`\n*docs.gl / OpenGL 4.6*\n\nReturns a vector pointing in the same direction as another.\n\nIf `dot(Nref, I) < 0.0`, returns `N`, otherwise returns `-N`.\n\n**Parameters:**\n* `N`: Specifies the vector to orient.\n* `I`: Specifies the incident vector.\n* `Nref`: Specifies the reference vector.",
        overloads: &[
            BuiltinOverload { label: "float faceforward(float N, float I, float Nref)", params: &["float N", "float I", "float Nref"] },
            BuiltinOverload { label: "vec2 faceforward(vec2 N, vec2 I, vec2 Nref)", params: &["vec2 N", "vec2 I", "vec2 Nref"] },
            BuiltinOverload { label: "vec3 faceforward(vec3 N, vec3 I, vec3 Nref)", params: &["vec3 N", "vec3 I", "vec3 Nref"] },
            BuiltinOverload { label: "vec4 faceforward(vec4 N, vec4 I, vec4 Nref)", params: &["vec4 N", "vec4 I", "vec4 Nref"] },
            BuiltinOverload { label: "double faceforward(double N, double I, double Nref)", params: &["double N", "double I", "double Nref"] },
            BuiltinOverload { label: "dvec2 faceforward(dvec2 N, dvec2 I, dvec2 Nref)", params: &["dvec2 N", "dvec2 I", "dvec2 Nref"] },
            BuiltinOverload { label: "dvec3 faceforward(dvec3 N, dvec3 I, dvec3 Nref)", params: &["dvec3 N", "dvec3 I", "dvec3 Nref"] },
            BuiltinOverload { label: "dvec4 faceforward(dvec4 N, dvec4 I, dvec4 Nref)", params: &["dvec4 N", "dvec4 I", "dvec4 Nref"] },
        ],
    },
    BuiltinFunction {
        name: "reflect",
        description: "### `reflect`\n*docs.gl / OpenGL 4.6*\n\nCalculates the reflection direction for an incident vector.\n\n$$\\text{reflect}(I, N) = I - 2 \\cdot \\text{dot}(N, I) \\cdot N$$\n\n`N` must be normalized.\n\n**Parameters:**\n* `I`: Specifies the incident vector.\n* `N`: Specifies the normal vector.",
        overloads: &[
            BuiltinOverload { label: "float reflect(float I, float N)", params: &["float I", "float N"] },
            BuiltinOverload { label: "vec2 reflect(vec2 I, vec2 N)", params: &["vec2 I", "vec2 N"] },
            BuiltinOverload { label: "vec3 reflect(vec3 I, vec3 N)", params: &["vec3 I", "vec3 N"] },
            BuiltinOverload { label: "vec4 reflect(vec4 I, vec4 N)", params: &["vec4 I", "vec4 N"] },
            BuiltinOverload { label: "double reflect(double I, double N)", params: &["double I", "double N"] },
            BuiltinOverload { label: "dvec2 reflect(dvec2 I, dvec2 N)", params: &["dvec2 I", "dvec2 N"] },
            BuiltinOverload { label: "dvec3 reflect(dvec3 I, dvec3 N)", params: &["dvec3 I", "dvec3 N"] },
            BuiltinOverload { label: "dvec4 reflect(dvec4 I, dvec4 N)", params: &["dvec4 I", "dvec4 N"] },
        ],
    },
    BuiltinFunction {
        name: "refract",
        description: "### `refract`\n*docs.gl / OpenGL 4.6*\n\nCalculates the refraction direction for an incident vector.\n\nUses Snell's law: calculates refraction of incident vector `I` with surface normal `N` and ratio of indices of refraction `eta`.\n\n**Parameters:**\n* `I`: Specifies the incident vector.\n* `N`: Specifies the normal vector.\n* `eta`: Specifies the ratio of indices of refraction.",
        overloads: &[
            BuiltinOverload { label: "float refract(float I, float N, float eta)", params: &["float I", "float N", "float eta"] },
            BuiltinOverload { label: "vec2 refract(vec2 I, vec2 N, float eta)", params: &["vec2 I", "vec2 N", "float eta"] },
            BuiltinOverload { label: "vec3 refract(vec3 I, vec3 N, float eta)", params: &["vec3 I", "vec3 N", "float eta"] },
            BuiltinOverload { label: "vec4 refract(vec4 I, vec4 N, float eta)", params: &["vec4 I", "vec4 N", "float eta"] },
            BuiltinOverload { label: "double refract(double I, double N, float eta)", params: &["double I", "double N", "float eta"] },
            BuiltinOverload { label: "dvec2 refract(dvec2 I, dvec2 N, float eta)", params: &["dvec2 I", "dvec2 N", "float eta"] },
            BuiltinOverload { label: "dvec3 refract(dvec3 I, dvec3 N, float eta)", params: &["dvec3 I", "dvec3 N", "float eta"] },
            BuiltinOverload { label: "dvec4 refract(dvec4 I, dvec4 N, float eta)", params: &["dvec4 I", "dvec4 N", "float eta"] },
        ],
    },

    // ------------------------------------------------------------------------
    // Common Math Functions
    // ------------------------------------------------------------------------
    BuiltinFunction {
        name: "clamp",
        description: "### `clamp`\n*docs.gl / OpenGL 4.6*\n\nConstrains a value to lie between two further values.\n\n$$\\text{clamp}(x, \\minVal, \\maxVal) = \\min(\\max(x, \\minVal), \\maxVal)$$\n\n**Parameters:**\n* `x`: Specify the value to constrain.\n* `minVal`: Specify the lower bound of the range.\n* `maxVal`: Specify the upper bound of the range.",
        overloads: &[
            BuiltinOverload { label: "float clamp(float x, float minVal, float maxVal)", params: &["float x", "float minVal", "float maxVal"] },
            BuiltinOverload { label: "vec2 clamp(vec2 x, vec2 minVal, vec2 maxVal)", params: &["vec2 x", "vec2 minVal", "vec2 maxVal"] },
            BuiltinOverload { label: "vec3 clamp(vec3 x, vec3 minVal, vec3 maxVal)", params: &["vec3 x", "vec3 minVal", "vec3 maxVal"] },
            BuiltinOverload { label: "vec4 clamp(vec4 x, vec4 minVal, vec4 maxVal)", params: &["vec4 x", "vec4 minVal", "vec4 maxVal"] },
            BuiltinOverload { label: "vec2 clamp(vec2 x, float minVal, float maxVal)", params: &["vec2 x", "float minVal", "float maxVal"] },
            BuiltinOverload { label: "vec3 clamp(vec3 x, float minVal, float maxVal)", params: &["vec3 x", "float minVal", "float maxVal"] },
            BuiltinOverload { label: "vec4 clamp(vec4 x, float minVal, float maxVal)", params: &["vec4 x", "float minVal", "float maxVal"] },
            BuiltinOverload { label: "int clamp(int x, int minVal, int maxVal)", params: &["int x", "int minVal", "int maxVal"] },
            BuiltinOverload { label: "uint clamp(uint x, uint minVal, uint maxVal)", params: &["uint x", "uint minVal", "uint maxVal"] },
        ],
    },
    BuiltinFunction {
        name: "mix",
        description: "### `mix`\n*docs.gl / OpenGL 4.6*\n\nLinearly interpolates between two values.\n\n$$\\text{mix}(x, y, a) = x \\cdot (1 - a) + y \\cdot a$$\n\n**Parameters:**\n* `x`: Specify the start of the range.\n* `y`: Specify the end of the range.\n* `a`: Specify the interpolation factor (or boolean mask).",
        overloads: &[
            BuiltinOverload { label: "float mix(float x, float y, float a)", params: &["float x", "float y", "float a"] },
            BuiltinOverload { label: "vec2 mix(vec2 x, vec2 y, vec2 a)", params: &["vec2 x", "vec2 y", "vec2 a"] },
            BuiltinOverload { label: "vec3 mix(vec3 x, vec3 y, vec3 a)", params: &["vec3 x", "vec3 y", "vec3 a"] },
            BuiltinOverload { label: "vec4 mix(vec4 x, vec4 y, vec4 a)", params: &["vec4 x", "vec4 y", "vec4 a"] },
            BuiltinOverload { label: "vec2 mix(vec2 x, vec2 y, float a)", params: &["vec2 x", "vec2 y", "float a"] },
            BuiltinOverload { label: "vec3 mix(vec3 x, vec3 y, float a)", params: &["vec3 x", "vec3 y", "float a"] },
            BuiltinOverload { label: "vec4 mix(vec4 x, vec4 y, float a)", params: &["vec4 x", "vec4 y", "float a"] },
            BuiltinOverload { label: "genType mix(genType x, genType y, genBType a)", params: &["genType x", "genType y", "genBType a"] },
        ],
    },
    BuiltinFunction {
        name: "step",
        description: "### `step`\n*docs.gl / OpenGL 4.6*\n\nGenerates a step function by comparing two values.\n\nReturns `0.0` if `x < edge`, otherwise `1.0`.\n\n**Parameters:**\n* `edge`: Specifies the location of the edge of the step function.\n* `x`: Specifies the value to be evaluated.",
        overloads: &[
            BuiltinOverload { label: "float step(float edge, float x)", params: &["float edge", "float x"] },
            BuiltinOverload { label: "vec2 step(vec2 edge, vec2 x)", params: &["vec2 edge", "vec2 x"] },
            BuiltinOverload { label: "vec3 step(vec3 edge, vec3 x)", params: &["vec3 edge", "vec3 x"] },
            BuiltinOverload { label: "vec4 step(vec4 edge, vec4 x)", params: &["vec4 edge", "vec4 x"] },
            BuiltinOverload { label: "vec2 step(float edge, vec2 x)", params: &["float edge", "vec2 x"] },
            BuiltinOverload { label: "vec3 step(float edge, vec3 x)", params: &["float edge", "vec3 x"] },
            BuiltinOverload { label: "vec4 step(float edge, vec4 x)", params: &["float edge", "vec4 x"] },
        ],
    },
    BuiltinFunction {
        name: "smoothstep",
        description: "### `smoothstep`\n*docs.gl / OpenGL 4.6*\n\nPerforms smooth Hermite interpolation between 0 and 1 when $edge0 < x < edge1$.\n\nUseful for thresholding and anti-aliased transitions.\n\n**Parameters:**\n* `edge0`: Specifies the lower edge.\n* `edge1`: Specifies the upper edge.\n* `x`: Specifies the source value for interpolation.",
        overloads: &[
            BuiltinOverload { label: "float smoothstep(float edge0, float edge1, float x)", params: &["float edge0", "float edge1", "float x"] },
            BuiltinOverload { label: "vec2 smoothstep(vec2 edge0, vec2 edge1, vec2 x)", params: &["vec2 edge0", "vec2 edge1", "vec2 x"] },
            BuiltinOverload { label: "vec3 smoothstep(vec3 edge0, vec3 edge1, vec3 x)", params: &["vec3 edge0", "vec3 edge1", "vec3 x"] },
            BuiltinOverload { label: "vec4 smoothstep(vec4 edge0, vec4 edge1, vec4 x)", params: &["vec4 edge0", "vec4 edge1", "vec4 x"] },
            BuiltinOverload { label: "vec2 smoothstep(float edge0, float edge1, vec2 x)", params: &["float edge0", "float edge1", "vec2 x"] },
            BuiltinOverload { label: "vec3 smoothstep(float edge0, float edge1, vec3 x)", params: &["float edge0", "float edge1", "vec3 x"] },
            BuiltinOverload { label: "vec4 smoothstep(float edge0, float edge1, vec4 x)", params: &["float edge0", "float edge1", "vec4 x"] },
        ],
    },
    BuiltinFunction {
        name: "abs",
        description: "### `abs`\n*docs.gl / OpenGL 4.6*\n\nReturns the absolute value of the parameter.\n\n**Parameters:**\n* `x`: Specifies the value of which to return the absolute value.",
        overloads: &[
            BuiltinOverload { label: "float abs(float x)", params: &["float x"] },
            BuiltinOverload { label: "vec2 abs(vec2 x)", params: &["vec2 x"] },
            BuiltinOverload { label: "vec3 abs(vec3 x)", params: &["vec3 x"] },
            BuiltinOverload { label: "vec4 abs(vec4 x)", params: &["vec4 x"] },
            BuiltinOverload { label: "int abs(int x)", params: &["int x"] },
            BuiltinOverload { label: "ivec2 abs(ivec2 x)", params: &["ivec2 x"] },
            BuiltinOverload { label: "ivec3 abs(ivec3 x)", params: &["ivec3 x"] },
            BuiltinOverload { label: "ivec4 abs(ivec4 x)", params: &["ivec4 x"] },
        ],
    },
    BuiltinFunction {
        name: "sign",
        description: "### `sign`\n*docs.gl / OpenGL 4.6*\n\nExtracts the sign of the parameter.\n\nReturns `1.0` if `x > 0`, `0.0` if `x == 0`, and `-1.0` if `x < 0`.\n\n**Parameters:**\n* `x`: Specifies the value from which to extract the sign.",
        overloads: &[
            BuiltinOverload { label: "float sign(float x)", params: &["float x"] },
            BuiltinOverload { label: "vec2 sign(vec2 x)", params: &["vec2 x"] },
            BuiltinOverload { label: "vec3 sign(vec3 x)", params: &["vec3 x"] },
            BuiltinOverload { label: "vec4 sign(vec4 x)", params: &["vec4 x"] },
            BuiltinOverload { label: "int sign(int x)", params: &["int x"] },
        ],
    },
    BuiltinFunction {
        name: "floor",
        description: "### `floor`\n*docs.gl / OpenGL 4.6*\n\nFinds the nearest integer less than or equal to the parameter.\n\n**Parameters:**\n* `x`: Specifies the value to evaluate.",
        overloads: &[
            BuiltinOverload { label: "float floor(float x)", params: &["float x"] },
            BuiltinOverload { label: "vec2 floor(vec2 x)", params: &["vec2 x"] },
            BuiltinOverload { label: "vec3 floor(vec3 x)", params: &["vec3 x"] },
            BuiltinOverload { label: "vec4 floor(vec4 x)", params: &["vec4 x"] },
        ],
    },
    BuiltinFunction {
        name: "ceil",
        description: "### `ceil`\n*docs.gl / OpenGL 4.6*\n\nFinds the nearest integer greater than or equal to the parameter.\n\n**Parameters:**\n* `x`: Specifies the value to evaluate.",
        overloads: &[
            BuiltinOverload { label: "float ceil(float x)", params: &["float x"] },
            BuiltinOverload { label: "vec2 ceil(vec2 x)", params: &["vec2 x"] },
            BuiltinOverload { label: "vec3 ceil(vec3 x)", params: &["vec3 x"] },
            BuiltinOverload { label: "vec4 ceil(vec4 x)", params: &["vec4 x"] },
        ],
    },
    BuiltinFunction {
        name: "fract",
        description: "### `fract`\n*docs.gl / OpenGL 4.6*\n\nComputes the fractional part of the argument.\n\n$$\\text{fract}(x) = x - \\text{floor}(x)$$\n\n**Parameters:**\n* `x`: Specifies the value from which to obtain the fractional part.",
        overloads: &[
            BuiltinOverload { label: "float fract(float x)", params: &["float x"] },
            BuiltinOverload { label: "vec2 fract(vec2 x)", params: &["vec2 x"] },
            BuiltinOverload { label: "vec3 fract(vec3 x)", params: &["vec3 x"] },
            BuiltinOverload { label: "vec4 fract(vec4 x)", params: &["vec4 x"] },
        ],
    },
    BuiltinFunction {
        name: "mod",
        description: "### `mod`\n*docs.gl / OpenGL 4.6*\n\nComputes value of one parameter modulo another.\n\n$$\\text{mod}(x, y) = x - y \\cdot \\text{floor}(x / y)$$\n\n**Parameters:**\n* `x`: Specifies the numerator.\n* `y`: Specifies the denominator.",
        overloads: &[
            BuiltinOverload { label: "float mod(float x, float y)", params: &["float x", "float y"] },
            BuiltinOverload { label: "vec2 mod(vec2 x, vec2 y)", params: &["vec2 x", "vec2 y"] },
            BuiltinOverload { label: "vec3 mod(vec3 x, vec3 y)", params: &["vec3 x", "vec3 y"] },
            BuiltinOverload { label: "vec4 mod(vec4 x, vec4 y)", params: &["vec4 x", "vec4 y"] },
            BuiltinOverload { label: "vec2 mod(vec2 x, float y)", params: &["vec2 x", "float y"] },
            BuiltinOverload { label: "vec3 mod(vec3 x, float y)", params: &["vec3 x", "float y"] },
            BuiltinOverload { label: "vec4 mod(vec4 x, float y)", params: &["vec4 x", "float y"] },
        ],
    },
    BuiltinFunction {
        name: "min",
        description: "### `min`\n*docs.gl / OpenGL 4.6*\n\nReturns the minimum of two values.\n\n**Parameters:**\n* `x`: Specifies the first value.\n* `y`: Specifies the second value.",
        overloads: &[
            BuiltinOverload { label: "float min(float x, float y)", params: &["float x", "float y"] },
            BuiltinOverload { label: "vec2 min(vec2 x, vec2 y)", params: &["vec2 x", "vec2 y"] },
            BuiltinOverload { label: "vec3 min(vec3 x, vec3 y)", params: &["vec3 x", "vec3 y"] },
            BuiltinOverload { label: "vec4 min(vec4 x, vec4 y)", params: &["vec4 x", "vec4 y"] },
            BuiltinOverload { label: "vec2 min(vec2 x, float y)", params: &["vec2 x", "float y"] },
            BuiltinOverload { label: "vec3 min(vec3 x, float y)", params: &["vec3 x", "float y"] },
            BuiltinOverload { label: "vec4 min(vec4 x, float y)", params: &["vec4 x", "float y"] },
            BuiltinOverload { label: "int min(int x, int y)", params: &["int x", "int y"] },
            BuiltinOverload { label: "uint min(uint x, uint y)", params: &["uint x", "uint y"] },
        ],
    },
    BuiltinFunction {
        name: "max",
        description: "### `max`\n*docs.gl / OpenGL 4.6*\n\nReturns the maximum of two values.\n\n**Parameters:**\n* `x`: Specifies the first value.\n* `y`: Specifies the second value.",
        overloads: &[
            BuiltinOverload { label: "float max(float x, float y)", params: &["float x", "float y"] },
            BuiltinOverload { label: "vec2 max(vec2 x, vec2 y)", params: &["vec2 x", "vec2 y"] },
            BuiltinOverload { label: "vec3 max(vec3 x, vec3 y)", params: &["vec3 x", "vec3 y"] },
            BuiltinOverload { label: "vec4 max(vec4 x, vec4 y)", params: &["vec4 x", "vec4 y"] },
            BuiltinOverload { label: "vec2 max(vec2 x, float y)", params: &["vec2 x", "float y"] },
            BuiltinOverload { label: "vec3 max(vec3 x, float y)", params: &["vec3 x", "float y"] },
            BuiltinOverload { label: "vec4 max(vec4 x, float y)", params: &["vec4 x", "float y"] },
            BuiltinOverload { label: "int max(int x, int y)", params: &["int x", "int y"] },
            BuiltinOverload { label: "uint max(uint x, uint y)", params: &["uint x", "uint y"] },
        ],
    },

    // ------------------------------------------------------------------------
    // Trigonometry & Exponential Functions
    // ------------------------------------------------------------------------
    BuiltinFunction {
        name: "sin",
        description: "### `sin`\n*docs.gl / OpenGL 4.6*\n\nReturns the sine of the parameter (in radians).\n\n**Parameters:**\n* `angle`: Specifies the angle, in radians.",
        overloads: &[
            BuiltinOverload { label: "float sin(float angle)", params: &["float angle"] },
            BuiltinOverload { label: "vec2 sin(vec2 angle)", params: &["vec2 angle"] },
            BuiltinOverload { label: "vec3 sin(vec3 angle)", params: &["vec3 angle"] },
            BuiltinOverload { label: "vec4 sin(vec4 angle)", params: &["vec4 angle"] },
        ],
    },
    BuiltinFunction {
        name: "cos",
        description: "### `cos`\n*docs.gl / OpenGL 4.6*\n\nReturns the cosine of the parameter (in radians).\n\n**Parameters:**\n* `angle`: Specifies the angle, in radians.",
        overloads: &[
            BuiltinOverload { label: "float cos(float angle)", params: &["float angle"] },
            BuiltinOverload { label: "vec2 cos(vec2 angle)", params: &["vec2 angle"] },
            BuiltinOverload { label: "vec3 cos(vec3 angle)", params: &["vec3 angle"] },
            BuiltinOverload { label: "vec4 cos(vec4 angle)", params: &["vec4 angle"] },
        ],
    },
    BuiltinFunction {
        name: "tan",
        description: "### `tan`\n*docs.gl / OpenGL 4.6*\n\nReturns the tangent of the parameter (in radians).\n\n**Parameters:**\n* `angle`: Specifies the angle, in radians.",
        overloads: &[
            BuiltinOverload { label: "float tan(float angle)", params: &["float angle"] },
            BuiltinOverload { label: "vec2 tan(vec2 angle)", params: &["vec2 angle"] },
            BuiltinOverload { label: "vec3 tan(vec3 angle)", params: &["vec3 angle"] },
            BuiltinOverload { label: "vec4 tan(vec4 angle)", params: &["vec4 angle"] },
        ],
    },
    BuiltinFunction {
        name: "asin",
        description: "### `asin`\n*docs.gl / OpenGL 4.6*\n\nReturns the arcsine of the parameter in range $[-\\pi/2, \\pi/2]$.\n\n**Parameters:**\n* `x`: Specifies the value whose arcsine to return.",
        overloads: &[
            BuiltinOverload { label: "float asin(float x)", params: &["float x"] },
            BuiltinOverload { label: "vec2 asin(vec2 x)", params: &["vec2 x"] },
            BuiltinOverload { label: "vec3 asin(vec3 x)", params: &["vec3 x"] },
            BuiltinOverload { label: "vec4 asin(vec4 x)", params: &["vec4 x"] },
        ],
    },
    BuiltinFunction {
        name: "acos",
        description: "### `acos`\n*docs.gl / OpenGL 4.6*\n\nReturns the arccosine of the parameter in range $[0, \\pi]$.\n\n**Parameters:**\n* `x`: Specifies the value whose arccosine to return.",
        overloads: &[
            BuiltinOverload { label: "float acos(float x)", params: &["float x"] },
            BuiltinOverload { label: "vec2 acos(vec2 x)", params: &["vec2 x"] },
            BuiltinOverload { label: "vec3 acos(vec3 x)", params: &["vec3 x"] },
            BuiltinOverload { label: "vec4 acos(vec4 x)", params: &["vec4 x"] },
        ],
    },
    BuiltinFunction {
        name: "atan",
        description: "### `atan`\n*docs.gl / OpenGL 4.6*\n\nReturns the arc-tangent of the parameter.\n\n* 1-argument variant: returns $\\arctan(y)$ in range $[-\\pi/2, \\pi/2]$.\n* 2-argument variant: returns $\\text{atan2}(y, x)$ in range $[-\\pi, \\pi]$.\n\n**Parameters:**\n* `y`: Specifies the numerator of tangent (or opposite side).\n* `x`: Specifies the denominator of tangent (or adjacent side).",
        overloads: &[
            BuiltinOverload { label: "float atan(float y, float x)", params: &["float y", "float x"] },
            BuiltinOverload { label: "vec2 atan(vec2 y, vec2 x)", params: &["vec2 y", "vec2 x"] },
            BuiltinOverload { label: "vec3 atan(vec3 y, vec3 x)", params: &["vec3 y", "vec3 x"] },
            BuiltinOverload { label: "vec4 atan(vec4 y, vec4 x)", params: &["vec4 y", "vec4 x"] },
            BuiltinOverload { label: "float atan(float y_over_x)", params: &["float y_over_x"] },
            BuiltinOverload { label: "vec2 atan(vec2 y_over_x)", params: &["vec2 y_over_x"] },
            BuiltinOverload { label: "vec3 atan(vec3 y_over_x)", params: &["vec3 y_over_x"] },
            BuiltinOverload { label: "vec4 atan(vec4 y_over_x)", params: &["vec4 y_over_x"] },
        ],
    },
    BuiltinFunction {
        name: "pow",
        description: "### `pow`\n*docs.gl / OpenGL 4.6*\n\nReturns the value of the first parameter raised to the power of the second ($x^y$).\n\nResults are undefined if $x < 0$ or if $x = 0$ and $y \\le 0$.\n\n**Parameters:**\n* `x`: Specifies the base.\n* `y`: Specifies the exponent.",
        overloads: &[
            BuiltinOverload { label: "float pow(float x, float y)", params: &["float x", "float y"] },
            BuiltinOverload { label: "vec2 pow(vec2 x, vec2 y)", params: &["vec2 x", "vec2 y"] },
            BuiltinOverload { label: "vec3 pow(vec3 x, vec3 y)", params: &["vec3 x", "vec3 y"] },
            BuiltinOverload { label: "vec4 pow(vec4 x, vec4 y)", params: &["vec4 x", "vec4 y"] },
        ],
    },
    BuiltinFunction {
        name: "exp",
        description: "### `exp`\n*docs.gl / OpenGL 4.6*\n\nReturns the natural exponentiation of the parameter ($e^x$).\n\n**Parameters:**\n* `x`: Specifies the exponent value.",
        overloads: &[
            BuiltinOverload { label: "float exp(float x)", params: &["float x"] },
            BuiltinOverload { label: "vec2 exp(vec2 x)", params: &["vec2 x"] },
            BuiltinOverload { label: "vec3 exp(vec3 x)", params: &["vec3 x"] },
            BuiltinOverload { label: "vec4 exp(vec4 x)", params: &["vec4 x"] },
        ],
    },
    BuiltinFunction {
        name: "log",
        description: "### `log`\n*docs.gl / OpenGL 4.6*\n\nReturns the natural logarithm of the parameter ($\\ln(x)$).\n\n**Parameters:**\n* `x`: Specifies the value of which to calculate natural log.",
        overloads: &[
            BuiltinOverload { label: "float log(float x)", params: &["float x"] },
            BuiltinOverload { label: "vec2 log(vec2 x)", params: &["vec2 x"] },
            BuiltinOverload { label: "vec3 log(vec3 x)", params: &["vec3 x"] },
            BuiltinOverload { label: "vec4 log(vec4 x)", params: &["vec4 x"] },
        ],
    },
    BuiltinFunction {
        name: "sqrt",
        description: "### `sqrt`\n*docs.gl / OpenGL 4.6*\n\nReturns the square root of the parameter ($\\sqrt{x}$).\n\n**Parameters:**\n* `x`: Specifies the value of which to calculate square root.",
        overloads: &[
            BuiltinOverload { label: "float sqrt(float x)", params: &["float x"] },
            BuiltinOverload { label: "vec2 sqrt(vec2 x)", params: &["vec2 x"] },
            BuiltinOverload { label: "vec3 sqrt(vec3 x)", params: &["vec3 x"] },
            BuiltinOverload { label: "vec4 sqrt(vec4 x)", params: &["vec4 x"] },
        ],
    },
    BuiltinFunction {
        name: "inversesqrt",
        description: "### `inversesqrt`\n*docs.gl / OpenGL 4.6*\n\nReturns the inverse square root of the parameter ($1 / \\sqrt{x}$).\n\n**Parameters:**\n* `x`: Specifies the value of which to calculate inverse square root.",
        overloads: &[
            BuiltinOverload { label: "float inversesqrt(float x)", params: &["float x"] },
            BuiltinOverload { label: "vec2 inversesqrt(vec2 x)", params: &["vec2 x"] },
            BuiltinOverload { label: "vec3 inversesqrt(vec3 x)", params: &["vec3 x"] },
            BuiltinOverload { label: "vec4 inversesqrt(vec4 x)", params: &["vec4 x"] },
        ],
    },

    // ------------------------------------------------------------------------
    // Matrix Functions
    // ------------------------------------------------------------------------
    BuiltinFunction {
        name: "transpose",
        description: "### `transpose`\n*docs.gl / OpenGL 4.6*\n\nCalculates the transpose of a matrix.\n\n**Parameters:**\n* `m`: Specifies the matrix to transpose.",
        overloads: &[
            BuiltinOverload { label: "mat2 transpose(mat2 m)", params: &["mat2 m"] },
            BuiltinOverload { label: "mat3 transpose(mat3 m)", params: &["mat3 m"] },
            BuiltinOverload { label: "mat4 transpose(mat4 m)", params: &["mat4 m"] },
            BuiltinOverload { label: "mat2x3 transpose(mat3x2 m)", params: &["mat3x2 m"] },
            BuiltinOverload { label: "mat3x2 transpose(mat2x3 m)", params: &["mat2x3 m"] },
            BuiltinOverload { label: "mat2x4 transpose(mat4x2 m)", params: &["mat4x2 m"] },
            BuiltinOverload { label: "mat4x2 transpose(mat2x4 m)", params: &["mat2x4 m"] },
            BuiltinOverload { label: "mat3x4 transpose(mat4x3 m)", params: &["mat4x3 m"] },
            BuiltinOverload { label: "mat4x3 transpose(mat3x4 m)", params: &["mat3x4 m"] },
        ],
    },
    BuiltinFunction {
        name: "determinant",
        description: "### `determinant`\n*docs.gl / OpenGL 4.6*\n\nCalculates the determinant of a square matrix.\n\n**Parameters:**\n* `m`: Specifies the matrix of which to calculate the determinant.",
        overloads: &[
            BuiltinOverload { label: "float determinant(mat2 m)", params: &["mat2 m"] },
            BuiltinOverload { label: "float determinant(mat3 m)", params: &["mat3 m"] },
            BuiltinOverload { label: "float determinant(mat4 m)", params: &["mat4 m"] },
            BuiltinOverload { label: "double determinant(dmat2 m)", params: &["dmat2 m"] },
            BuiltinOverload { label: "double determinant(dmat3 m)", params: &["dmat3 m"] },
            BuiltinOverload { label: "double determinant(dmat4 m)", params: &["dmat4 m"] },
        ],
    },
    BuiltinFunction {
        name: "inverse",
        description: "### `inverse`\n*docs.gl / OpenGL 4.6*\n\nCalculates the inverse of a square matrix such that $M \\cdot M^{-1} = I$.\n\n**Parameters:**\n* `m`: Specifies the matrix of which to calculate inverse.",
        overloads: &[
            BuiltinOverload { label: "mat2 inverse(mat2 m)", params: &["mat2 m"] },
            BuiltinOverload { label: "mat3 inverse(mat3 m)", params: &["mat3 m"] },
            BuiltinOverload { label: "mat4 inverse(mat4 m)", params: &["mat4 m"] },
            BuiltinOverload { label: "dmat2 inverse(dmat2 m)", params: &["dmat2 m"] },
            BuiltinOverload { label: "dmat3 inverse(dmat3 m)", params: &["dmat3 m"] },
            BuiltinOverload { label: "dmat4 inverse(dmat4 m)", params: &["dmat4 m"] },
        ],
    },

    // ------------------------------------------------------------------------
    // Texture Sampling Functions
    // ------------------------------------------------------------------------
    BuiltinFunction {
        name: "texture",
        description: "### `texture`\n*docs.gl / OpenGL 4.6*\n\nRetrieves texels from a texture.\n\nSamples texture `sampler` at texture coordinates `P`.\n\n**Parameters:**\n* `sampler`: Specifies the sampler to which the texture is bound.\n* `P`: Specifies texture coordinates (e.g. `vec2` for 2D, `vec3` for cube/3D).\n* `bias`: Optional mipmap LOD bias.",
        overloads: &[
            BuiltinOverload { label: "vec4 texture(sampler1D sampler, float P, [float bias])", params: &["sampler1D sampler", "float P", "[float bias]"] },
            BuiltinOverload { label: "vec4 texture(sampler2D sampler, vec2 P, [float bias])", params: &["sampler2D sampler", "vec2 P", "[float bias]"] },
            BuiltinOverload { label: "vec4 texture(sampler3D sampler, vec3 P, [float bias])", params: &["sampler3D sampler", "vec3 P", "[float bias]"] },
            BuiltinOverload { label: "vec4 texture(samplerCube sampler, vec3 P, [float bias])", params: &["samplerCube sampler", "vec3 P", "[float bias]"] },
            BuiltinOverload { label: "float texture(sampler1DShadow sampler, vec3 P, [float bias])", params: &["sampler1DShadow sampler", "vec3 P", "[float bias]"] },
            BuiltinOverload { label: "float texture(sampler2DShadow sampler, vec3 P, [float bias])", params: &["sampler2DShadow sampler", "vec3 P", "[float bias]"] },
            BuiltinOverload { label: "float texture(samplerCubeShadow sampler, vec4 P, [float bias])", params: &["samplerCubeShadow sampler", "vec4 P", "[float bias]"] },
            BuiltinOverload { label: "vec4 texture(sampler2DArray sampler, vec3 P, [float bias])", params: &["sampler2DArray sampler", "vec3 P", "[float bias]"] },
        ],
    },
    BuiltinFunction {
        name: "textureLod",
        description: "### `textureLod`\n*docs.gl / OpenGL 4.6*\n\nRetrieves texels from a texture with explicit Level of Detail (LOD).\n\n**Parameters:**\n* `sampler`: Specifies the sampler bound to texture.\n* `P`: Specifies texture coordinates.\n* `lod`: Specifies explicit mipmap level of detail.",
        overloads: &[
            BuiltinOverload { label: "vec4 textureLod(sampler1D sampler, float P, float lod)", params: &["sampler1D sampler", "float P", "float lod"] },
            BuiltinOverload { label: "vec4 textureLod(sampler2D sampler, vec2 P, float lod)", params: &["sampler2D sampler", "vec2 P", "float lod"] },
            BuiltinOverload { label: "vec4 textureLod(sampler3D sampler, vec3 P, float lod)", params: &["sampler3D sampler", "vec3 P", "float lod"] },
            BuiltinOverload { label: "vec4 textureLod(samplerCube sampler, vec3 P, float lod)", params: &["samplerCube sampler", "vec3 P", "float lod"] },
            BuiltinOverload { label: "vec4 textureLod(sampler2DArray sampler, vec3 P, float lod)", params: &["sampler2DArray sampler", "vec3 P", "float lod"] },
        ],
    },
    BuiltinFunction {
        name: "texelFetch",
        description: "### `texelFetch`\n*docs.gl / OpenGL 4.6*\n\nPerforms an unfiltered single-texel lookup using non-normalized integer coordinates.\n\n**Parameters:**\n* `sampler`: Specifies the sampler.\n* `P`: Specifies integer texel coordinates.\n* `lod`: Specifies mipmap level to look up.",
        overloads: &[
            BuiltinOverload { label: "vec4 texelFetch(sampler1D sampler, int P, int lod)", params: &["sampler1D sampler", "int P", "int lod"] },
            BuiltinOverload { label: "vec4 texelFetch(sampler2D sampler, ivec2 P, int lod)", params: &["sampler2D sampler", "ivec2 P", "int lod"] },
            BuiltinOverload { label: "vec4 texelFetch(sampler3D sampler, ivec3 P, int lod)", params: &["sampler3D sampler", "ivec3 P", "int lod"] },
            BuiltinOverload { label: "vec4 texelFetch(sampler2DMS sampler, ivec2 P, int sample)", params: &["sampler2DMS sampler", "ivec2 P", "int sample"] },
            BuiltinOverload { label: "vec4 texelFetch(samplerBuffer sampler, int P)", params: &["samplerBuffer sampler", "int P"] },
        ],
    },
    BuiltinFunction {
        name: "textureSize",
        description: "### `textureSize`\n*docs.gl / OpenGL 4.6*\n\nRetrieves the dimensions of a texture level in texels.\n\n**Parameters:**\n* `sampler`: Specifies the sampler.\n* `lod`: Specifies the mipmap level.",
        overloads: &[
            BuiltinOverload { label: "int textureSize(sampler1D sampler, int lod)", params: &["sampler1D sampler", "int lod"] },
            BuiltinOverload { label: "ivec2 textureSize(sampler2D sampler, int lod)", params: &["sampler2D sampler", "int lod"] },
            BuiltinOverload { label: "ivec3 textureSize(sampler3D sampler, int lod)", params: &["sampler3D sampler", "int lod"] },
            BuiltinOverload { label: "ivec2 textureSize(samplerCube sampler, int lod)", params: &["samplerCube sampler", "int lod"] },
            BuiltinOverload { label: "ivec3 textureSize(sampler2DArray sampler, int lod)", params: &["sampler2DArray sampler", "int lod"] },
            BuiltinOverload { label: "ivec2 textureSize(sampler2DMS sampler)", params: &["sampler2DMS sampler"] },
            BuiltinOverload { label: "int textureSize(samplerBuffer sampler)", params: &["samplerBuffer sampler"] },
        ],
    },

    // ------------------------------------------------------------------------
    // Derivative & Screening Functions
    // ------------------------------------------------------------------------
    BuiltinFunction {
        name: "dFdx",
        description: "### `dFdx`\n*docs.gl / OpenGL 4.6*\n\nReturns the partial derivative with respect to screen-space $x$ coordinate.\n\n$$\\frac{\\partial p}{\\partial x}$$\n\n**Parameters:**\n* `p`: Specifies the expression to evaluate derivative of.",
        overloads: &[
            BuiltinOverload { label: "float dFdx(float p)", params: &["float p"] },
            BuiltinOverload { label: "vec2 dFdx(vec2 p)", params: &["vec2 p"] },
            BuiltinOverload { label: "vec3 dFdx(vec3 p)", params: &["vec3 p"] },
            BuiltinOverload { label: "vec4 dFdx(vec4 p)", params: &["vec4 p"] },
        ],
    },
    BuiltinFunction {
        name: "dFdy",
        description: "### `dFdy`\n*docs.gl / OpenGL 4.6*\n\nReturns the partial derivative with respect to screen-space $y$ coordinate.\n\n$$\\frac{\\partial p}{\\partial y}$$\n\n**Parameters:**\n* `p`: Specifies the expression to evaluate derivative of.",
        overloads: &[
            BuiltinOverload { label: "float dFdy(float p)", params: &["float p"] },
            BuiltinOverload { label: "vec2 dFdy(vec2 p)", params: &["vec2 p"] },
            BuiltinOverload { label: "vec3 dFdy(vec3 p)", params: &["vec3 p"] },
            BuiltinOverload { label: "vec4 dFdy(vec4 p)", params: &["vec4 p"] },
        ],
    },
    BuiltinFunction {
        name: "fwidth",
        description: "### `fwidth`\n*docs.gl / OpenGL 4.6*\n\nReturns the sum of the absolute values of derivatives in $x$ and $y$.\n\n$$\\text{fwidth}(p) = |\\text{dFdx}(p)| + |\\text{dFdy}(p)|$$\n\nUseful for computing screen-space filter widths and anti-aliased wireframes/edges.\n\n**Parameters:**\n* `p`: Specifies the expression to evaluate.",
        overloads: &[
            BuiltinOverload { label: "float fwidth(float p)", params: &["float p"] },
            BuiltinOverload { label: "vec2 fwidth(vec2 p)", params: &["vec2 p"] },
            BuiltinOverload { label: "vec3 fwidth(vec3 p)", params: &["vec3 p"] },
            BuiltinOverload { label: "vec4 fwidth(vec4 p)", params: &["vec4 p"] },
        ],
    },

    // ------------------------------------------------------------------------
    // Compute & Barrier Functions
    // ------------------------------------------------------------------------
    BuiltinFunction {
        name: "barrier",
        description: "### `barrier`\n*docs.gl / OpenGL 4.6*\n\nSynchronizes execution of all invocations in the current work group or compute shader workgroup.\n\nAll invocations in the work group must execute this function before any are allowed to continue.",
        overloads: &[
            BuiltinOverload { label: "void barrier()", params: &[] },
        ],
    },
    BuiltinFunction {
        name: "memoryBarrier",
        description: "### `memoryBarrier`\n*docs.gl / OpenGL 4.6*\n\nControls the ordering of memory transactions issued by a shader invocation.\n\nWaits on the completion of all memory accesses.",
        overloads: &[
            BuiltinOverload { label: "void memoryBarrier()", params: &[] },
        ],
    },
    BuiltinFunction {
        name: "memoryBarrierShared",
        description: "### `memoryBarrierShared`\n*docs.gl / OpenGL 4.6*\n\nControls the ordering of memory transactions issued for variables declared `shared` in a compute shader.",
        overloads: &[
            BuiltinOverload { label: "void memoryBarrierShared()", params: &[] },
        ],
    },
    BuiltinFunction {
        name: "groupMemoryBarrier",
        description: "### `groupMemoryBarrier`\n*docs.gl / OpenGL 4.6*\n\nControls the ordering of all memory transactions issued by all invocations in the work group.",
        overloads: &[
            BuiltinOverload { label: "void groupMemoryBarrier()", params: &[] },
        ],
    },
    // ------------------------------------------------------------------------
    // Matrix Functions
    // ------------------------------------------------------------------------
    BuiltinFunction {
        name: "transpose",
        description: "### `transpose`\n*docs.gl / OpenGL 4.6*\n\nCalculates the transpose of a matrix.\n\n$$\\text{transpose}(m)_{ij} = m_{ji}$$\n\n**Parameters:**\n* `m`: Specifies the matrix to transpose.",
        overloads: &[
            BuiltinOverload { label: "mat2 transpose(mat2 m)", params: &["mat2 m"] },
            BuiltinOverload { label: "mat3 transpose(mat3 m)", params: &["mat3 m"] },
            BuiltinOverload { label: "mat4 transpose(mat4 m)", params: &["mat4 m"] },
            BuiltinOverload { label: "mat2x3 transpose(mat3x2 m)", params: &["mat3x2 m"] },
            BuiltinOverload { label: "mat3x2 transpose(mat2x3 m)", params: &["mat2x3 m"] },
            BuiltinOverload { label: "mat2x4 transpose(mat4x2 m)", params: &["mat4x2 m"] },
            BuiltinOverload { label: "mat4x2 transpose(mat2x4 m)", params: &["mat2x4 m"] },
            BuiltinOverload { label: "mat3x4 transpose(mat4x3 m)", params: &["mat4x3 m"] },
            BuiltinOverload { label: "mat4x3 transpose(mat3x4 m)", params: &["mat3x4 m"] },
            BuiltinOverload { label: "dmat2 transpose(dmat2 m)", params: &["dmat2 m"] },
            BuiltinOverload { label: "dmat3 transpose(dmat3 m)", params: &["dmat3 m"] },
            BuiltinOverload { label: "dmat4 transpose(dmat4 m)", params: &["dmat4 m"] },
        ],
    },
    BuiltinFunction {
        name: "inverse",
        description: "### `inverse`\n*docs.gl / OpenGL 4.6*\n\nCalculates the inverse of a matrix.\n\n$$\\text{inverse}(m) \\cdot m = I$$\n\n**Parameters:**\n* `m`: Specifies the matrix of which to take the inverse.",
        overloads: &[
            BuiltinOverload { label: "mat2 inverse(mat2 m)", params: &["mat2 m"] },
            BuiltinOverload { label: "mat3 inverse(mat3 m)", params: &["mat3 m"] },
            BuiltinOverload { label: "mat4 inverse(mat4 m)", params: &["mat4 m"] },
            BuiltinOverload { label: "dmat2 inverse(dmat2 m)", params: &["dmat2 m"] },
            BuiltinOverload { label: "dmat3 inverse(dmat3 m)", params: &["dmat3 m"] },
            BuiltinOverload { label: "dmat4 inverse(dmat4 m)", params: &["dmat4 m"] },
        ],
    },
    BuiltinFunction {
        name: "determinant",
        description: "### `determinant`\n*docs.gl / OpenGL 4.6*\n\nCalculates the determinant of a square matrix.\n\n**Parameters:**\n* `m`: Specifies the square matrix to evaluate.",
        overloads: &[
            BuiltinOverload { label: "float determinant(mat2 m)", params: &["mat2 m"] },
            BuiltinOverload { label: "float determinant(mat3 m)", params: &["mat3 m"] },
            BuiltinOverload { label: "float determinant(mat4 m)", params: &["mat4 m"] },
            BuiltinOverload { label: "double determinant(dmat2 m)", params: &["dmat2 m"] },
            BuiltinOverload { label: "double determinant(dmat3 m)", params: &["dmat3 m"] },
            BuiltinOverload { label: "double determinant(dmat4 m)", params: &["dmat4 m"] },
        ],
    },
    BuiltinFunction {
        name: "matrixCompMult",
        description: "### `matrixCompMult`\n*docs.gl / OpenGL 4.6*\n\nPerforms component-wise multiplication of two matrices.\n\n$$z_{ij} = x_{ij} \\cdot y_{ij}$$\n\n**Parameters:**\n* `x`: Specifies the first matrix.\n* `y`: Specifies the second matrix.",
        overloads: &[
            BuiltinOverload { label: "mat2 matrixCompMult(mat2 x, mat2 y)", params: &["mat2 x", "mat2 y"] },
            BuiltinOverload { label: "mat3 matrixCompMult(mat3 x, mat3 y)", params: &["mat3 x", "mat3 y"] },
            BuiltinOverload { label: "mat4 matrixCompMult(mat4 x, mat4 y)", params: &["mat4 x", "mat4 y"] },
        ],
    },
    BuiltinFunction {
        name: "outerProduct",
        description: "### `outerProduct`\n*docs.gl / OpenGL 4.6*\n\nCalculates the outer product of a column vector and row vector.\n\n$$m_{ij} = c_i \\cdot r_j$$\n\n**Parameters:**\n* `c`: Specifies the column vector.\n* `r`: Specifies the row vector.",
        overloads: &[
            BuiltinOverload { label: "mat2 outerProduct(vec2 c, vec2 r)", params: &["vec2 c", "vec2 r"] },
            BuiltinOverload { label: "mat3 outerProduct(vec3 c, vec3 r)", params: &["vec3 c", "vec3 r"] },
            BuiltinOverload { label: "mat4 outerProduct(vec4 c, vec4 r)", params: &["vec4 c", "vec4 r"] },
        ],
    },

    // ------------------------------------------------------------------------
    // Vector Relational Functions
    // ------------------------------------------------------------------------
    BuiltinFunction {
        name: "lessThan",
        description: "### `lessThan`\n*docs.gl / OpenGL 4.6*\n\nPerforms a component-wise less-than comparison ($x < y$).",
        overloads: &[
            BuiltinOverload { label: "bvec2 lessThan(vec2 x, vec2 y)", params: &["vec2 x", "vec2 y"] },
            BuiltinOverload { label: "bvec3 lessThan(vec3 x, vec3 y)", params: &["vec3 x", "vec3 y"] },
            BuiltinOverload { label: "bvec4 lessThan(vec4 x, vec4 y)", params: &["vec4 x", "vec4 y"] },
            BuiltinOverload { label: "bvec2 lessThan(ivec2 x, ivec2 y)", params: &["ivec2 x", "ivec2 y"] },
            BuiltinOverload { label: "bvec3 lessThan(ivec3 x, ivec3 y)", params: &["ivec3 x", "ivec3 y"] },
            BuiltinOverload { label: "bvec4 lessThan(ivec4 x, ivec4 y)", params: &["ivec4 x", "ivec4 y"] },
            BuiltinOverload { label: "bvec2 lessThan(uvec2 x, uvec2 y)", params: &["uvec2 x", "uvec2 y"] },
            BuiltinOverload { label: "bvec3 lessThan(uvec3 x, uvec3 y)", params: &["uvec3 x", "uvec3 y"] },
            BuiltinOverload { label: "bvec4 lessThan(uvec4 x, uvec4 y)", params: &["uvec4 x", "uvec4 y"] },
        ],
    },
    BuiltinFunction {
        name: "lessThanEqual",
        description: "### `lessThanEqual`\n*docs.gl / OpenGL 4.6*\n\nPerforms a component-wise less-than-or-equal comparison ($x \\le y$).",
        overloads: &[
            BuiltinOverload { label: "bvec2 lessThanEqual(vec2 x, vec2 y)", params: &["vec2 x", "vec2 y"] },
            BuiltinOverload { label: "bvec3 lessThanEqual(vec3 x, vec3 y)", params: &["vec3 x", "vec3 y"] },
            BuiltinOverload { label: "bvec4 lessThanEqual(vec4 x, vec4 y)", params: &["vec4 x", "vec4 y"] },
            BuiltinOverload { label: "bvec2 lessThanEqual(ivec2 x, ivec2 y)", params: &["ivec2 x", "ivec2 y"] },
            BuiltinOverload { label: "bvec3 lessThanEqual(ivec3 x, ivec3 y)", params: &["ivec3 x", "ivec3 y"] },
            BuiltinOverload { label: "bvec4 lessThanEqual(ivec4 x, ivec4 y)", params: &["ivec4 x", "ivec4 y"] },
        ],
    },
    BuiltinFunction {
        name: "greaterThan",
        description: "### `greaterThan`\n*docs.gl / OpenGL 4.6*\n\nPerforms a component-wise greater-than comparison ($x > y$).",
        overloads: &[
            BuiltinOverload { label: "bvec2 greaterThan(vec2 x, vec2 y)", params: &["vec2 x", "vec2 y"] },
            BuiltinOverload { label: "bvec3 greaterThan(vec3 x, vec3 y)", params: &["vec3 x", "vec3 y"] },
            BuiltinOverload { label: "bvec4 greaterThan(vec4 x, vec4 y)", params: &["vec4 x", "vec4 y"] },
            BuiltinOverload { label: "bvec2 greaterThan(ivec2 x, ivec2 y)", params: &["ivec2 x", "ivec2 y"] },
            BuiltinOverload { label: "bvec3 greaterThan(ivec3 x, ivec3 y)", params: &["ivec3 x", "ivec3 y"] },
            BuiltinOverload { label: "bvec4 greaterThan(ivec4 x, ivec4 y)", params: &["ivec4 x", "ivec4 y"] },
        ],
    },
    BuiltinFunction {
        name: "greaterThanEqual",
        description: "### `greaterThanEqual`\n*docs.gl / OpenGL 4.6*\n\nPerforms a component-wise greater-than-or-equal comparison ($x \\ge y$).",
        overloads: &[
            BuiltinOverload { label: "bvec2 greaterThanEqual(vec2 x, vec2 y)", params: &["vec2 x", "vec2 y"] },
            BuiltinOverload { label: "bvec3 greaterThanEqual(vec3 x, vec3 y)", params: &["vec3 x", "vec3 y"] },
            BuiltinOverload { label: "bvec4 greaterThanEqual(vec4 x, vec4 y)", params: &["vec4 x", "vec4 y"] },
            BuiltinOverload { label: "bvec2 greaterThanEqual(ivec2 x, ivec2 y)", params: &["ivec2 x", "ivec2 y"] },
            BuiltinOverload { label: "bvec3 greaterThanEqual(ivec3 x, ivec3 y)", params: &["ivec3 x", "ivec3 y"] },
            BuiltinOverload { label: "bvec4 greaterThanEqual(ivec4 x, ivec4 y)", params: &["ivec4 x", "ivec4 y"] },
        ],
    },
    BuiltinFunction {
        name: "equal",
        description: "### `equal`\n*docs.gl / OpenGL 4.6*\n\nReturns a component-wise boolean vector comparing if $x == y$.",
        overloads: &[
            BuiltinOverload { label: "bvec2 equal(vec2 x, vec2 y)", params: &["vec2 x", "vec2 y"] },
            BuiltinOverload { label: "bvec3 equal(vec3 x, vec3 y)", params: &["vec3 x", "vec3 y"] },
            BuiltinOverload { label: "bvec4 equal(vec4 x, vec4 y)", params: &["vec4 x", "vec4 y"] },
            BuiltinOverload { label: "bvec2 equal(ivec2 x, ivec2 y)", params: &["ivec2 x", "ivec2 y"] },
            BuiltinOverload { label: "bvec3 equal(ivec3 x, ivec3 y)", params: &["ivec3 x", "ivec3 y"] },
            BuiltinOverload { label: "bvec4 equal(ivec4 x, ivec4 y)", params: &["ivec4 x", "ivec4 y"] },
            BuiltinOverload { label: "bvec2 equal(bvec2 x, bvec2 y)", params: &["bvec2 x", "bvec2 y"] },
            BuiltinOverload { label: "bvec3 equal(bvec3 x, bvec3 y)", params: &["bvec3 x", "bvec3 y"] },
            BuiltinOverload { label: "bvec4 equal(bvec4 x, bvec4 y)", params: &["bvec4 x", "bvec4 y"] },
        ],
    },
    BuiltinFunction {
        name: "notEqual",
        description: "### `notEqual`\n*docs.gl / OpenGL 4.6*\n\nReturns a component-wise boolean vector comparing if $x \\ne y$.",
        overloads: &[
            BuiltinOverload { label: "bvec2 notEqual(vec2 x, vec2 y)", params: &["vec2 x", "vec2 y"] },
            BuiltinOverload { label: "bvec3 notEqual(vec3 x, vec3 y)", params: &["vec3 x", "vec3 y"] },
            BuiltinOverload { label: "bvec4 notEqual(vec4 x, vec4 y)", params: &["vec4 x", "vec4 y"] },
            BuiltinOverload { label: "bvec2 notEqual(ivec2 x, ivec2 y)", params: &["ivec2 x", "ivec2 y"] },
            BuiltinOverload { label: "bvec3 notEqual(ivec3 x, ivec3 y)", params: &["ivec3 x", "ivec3 y"] },
            BuiltinOverload { label: "bvec4 notEqual(ivec4 x, ivec4 y)", params: &["ivec4 x", "ivec4 y"] },
            BuiltinOverload { label: "bvec2 notEqual(bvec2 x, bvec2 y)", params: &["bvec2 x", "bvec2 y"] },
            BuiltinOverload { label: "bvec3 notEqual(bvec3 x, bvec3 y)", params: &["bvec3 x", "bvec3 y"] },
            BuiltinOverload { label: "bvec4 notEqual(bvec4 x, bvec4 y)", params: &["bvec4 x", "bvec4 y"] },
        ],
    },
    BuiltinFunction {
        name: "any",
        description: "### `any`\n*docs.gl / OpenGL 4.6*\n\nReturns `true` if any component of a boolean vector is true.",
        overloads: &[
            BuiltinOverload { label: "bool any(bvec2 x)", params: &["bvec2 x"] },
            BuiltinOverload { label: "bool any(bvec3 x)", params: &["bvec3 x"] },
            BuiltinOverload { label: "bool any(bvec4 x)", params: &["bvec4 x"] },
        ],
    },
    BuiltinFunction {
        name: "all",
        description: "### `all`\n*docs.gl / OpenGL 4.6*\n\nReturns `true` only if all components of a boolean vector are true.",
        overloads: &[
            BuiltinOverload { label: "bool all(bvec2 x)", params: &["bvec2 x"] },
            BuiltinOverload { label: "bool all(bvec3 x)", params: &["bvec3 x"] },
            BuiltinOverload { label: "bool all(bvec4 x)", params: &["bvec4 x"] },
        ],
    },
    BuiltinFunction {
        name: "not",
        description: "### `not`\n*docs.gl / OpenGL 4.6*\n\nPerforms a component-wise logical NOT operation on a boolean vector.",
        overloads: &[
            BuiltinOverload { label: "bvec2 not(bvec2 x)", params: &["bvec2 x"] },
            BuiltinOverload { label: "bvec3 not(bvec3 x)", params: &["bvec3 x"] },
            BuiltinOverload { label: "bvec4 not(bvec4 x)", params: &["bvec4 x"] },
        ],
    },

    // ------------------------------------------------------------------------
    // Common Math & Bitwise Functions
    // ------------------------------------------------------------------------
    BuiltinFunction {
        name: "fma",
        description: "### `fma`\n*docs.gl / OpenGL 4.6*\n\nPerforms fused multiply-add operation: $a \\cdot b + c$ with a single rounding step.",
        overloads: &[
            BuiltinOverload { label: "float fma(float a, float b, float c)", params: &["float a", "float b", "float c"] },
            BuiltinOverload { label: "vec2 fma(vec2 a, vec2 b, vec2 c)", params: &["vec2 a", "vec2 b", "vec2 c"] },
            BuiltinOverload { label: "vec3 fma(vec3 a, vec3 b, vec3 c)", params: &["vec3 a", "vec3 b", "vec3 c"] },
            BuiltinOverload { label: "vec4 fma(vec4 a, vec4 b, vec4 c)", params: &["vec4 a", "vec4 b", "vec4 c"] },
        ],
    },
    BuiltinFunction {
        name: "bitfieldExtract",
        description: "### `bitfieldExtract`\n*docs.gl / OpenGL 4.6*\n\nExtracts a range of bits from an integer.",
        overloads: &[
            BuiltinOverload { label: "int bitfieldExtract(int value, int offset, int bits)", params: &["int value", "int offset", "int bits"] },
            BuiltinOverload { label: "uint bitfieldExtract(uint value, int offset, int bits)", params: &["uint value", "int offset", "int bits"] },
        ],
    },
    BuiltinFunction {
        name: "bitfieldInsert",
        description: "### `bitfieldInsert`\n*docs.gl / OpenGL 4.6*\n\nInserts a range of bits into an integer.",
        overloads: &[
            BuiltinOverload { label: "int bitfieldInsert(int base, int insert, int offset, int bits)", params: &["int base", "int insert", "int offset", "int bits"] },
            BuiltinOverload { label: "uint bitfieldInsert(uint base, uint insert, int offset, int bits)", params: &["uint base", "uint insert", "int offset", "int bits"] },
        ],
    },
    BuiltinFunction {
        name: "bitfieldReverse",
        description: "### `bitfieldReverse`\n*docs.gl / OpenGL 4.6*\n\nReverses the order of bits in an integer.",
        overloads: &[
            BuiltinOverload { label: "int bitfieldReverse(int value)", params: &["int value"] },
            BuiltinOverload { label: "uint bitfieldReverse(uint value)", params: &["uint value"] },
        ],
    },
    BuiltinFunction {
        name: "bitCount",
        description: "### `bitCount`\n*docs.gl / OpenGL 4.6*\n\nCounts the number of one-bits (population count) in an integer.",
        overloads: &[
            BuiltinOverload { label: "int bitCount(int value)", params: &["int value"] },
            BuiltinOverload { label: "int bitCount(uint value)", params: &["uint value"] },
        ],
    },
    BuiltinFunction {
        name: "findLSB",
        description: "### `findLSB`\n*docs.gl / OpenGL 4.6*\n\nFinds the index of the least significant set bit.",
        overloads: &[
            BuiltinOverload { label: "int findLSB(int value)", params: &["int value"] },
            BuiltinOverload { label: "int findLSB(uint value)", params: &["uint value"] },
        ],
    },
    BuiltinFunction {
        name: "findMSB",
        description: "### `findMSB`\n*docs.gl / OpenGL 4.6*\n\nFinds the index of the most significant set bit.",
        overloads: &[
            BuiltinOverload { label: "int findMSB(int value)", params: &["int value"] },
            BuiltinOverload { label: "int findMSB(uint value)", params: &["uint value"] },
        ],
    },
    BuiltinFunction {
        name: "floatBitsToInt",
        description: "### `floatBitsToInt`\n*docs.gl / OpenGL 4.6*\n\nReturns the IEEE 754 bit representation of a floating-point value as an integer.",
        overloads: &[
            BuiltinOverload { label: "int floatBitsToInt(float value)", params: &["float value"] },
            BuiltinOverload { label: "ivec2 floatBitsToInt(vec2 value)", params: &["vec2 value"] },
            BuiltinOverload { label: "ivec3 floatBitsToInt(vec3 value)", params: &["vec3 value"] },
            BuiltinOverload { label: "ivec4 floatBitsToInt(vec4 value)", params: &["vec4 value"] },
        ],
    },
    BuiltinFunction {
        name: "floatBitsToUint",
        description: "### `floatBitsToUint`\n*docs.gl / OpenGL 4.6*\n\nReturns the IEEE 754 bit representation of a floating-point value as an unsigned integer.",
        overloads: &[
            BuiltinOverload { label: "uint floatBitsToUint(float value)", params: &["float value"] },
            BuiltinOverload { label: "uvec2 floatBitsToUint(vec2 value)", params: &["vec2 value"] },
            BuiltinOverload { label: "uvec3 floatBitsToUint(vec3 value)", params: &["vec3 value"] },
            BuiltinOverload { label: "uvec4 floatBitsToUint(vec4 value)", params: &["vec4 value"] },
        ],
    },
    BuiltinFunction {
        name: "intBitsToFloat",
        description: "### `intBitsToFloat`\n*docs.gl / OpenGL 4.6*\n\nInterprets the bit pattern of an integer as an IEEE 754 floating-point value.",
        overloads: &[
            BuiltinOverload { label: "float intBitsToFloat(int value)", params: &["int value"] },
            BuiltinOverload { label: "vec2 intBitsToFloat(ivec2 value)", params: &["vec2 value"] },
            BuiltinOverload { label: "vec3 intBitsToFloat(ivec3 value)", params: &["vec3 value"] },
            BuiltinOverload { label: "vec4 intBitsToFloat(ivec4 value)", params: &["vec4 value"] },
        ],
    },
    BuiltinFunction {
        name: "uintBitsToFloat",
        description: "### `uintBitsToFloat`\n*docs.gl / OpenGL 4.6*\n\nInterprets the bit pattern of an unsigned integer as an IEEE 754 floating-point value.",
        overloads: &[
            BuiltinOverload { label: "float uintBitsToFloat(uint value)", params: &["uint value"] },
            BuiltinOverload { label: "vec2 uintBitsToFloat(uvec2 value)", params: &["uvec2 value"] },
            BuiltinOverload { label: "vec3 uintBitsToFloat(uvec3 value)", params: &["uvec3 value"] },
            BuiltinOverload { label: "vec4 uintBitsToFloat(uvec4 value)", params: &["uvec4 value"] },
        ],
    },
    BuiltinFunction {
        name: "atomicAdd",
        description: "### `atomicAdd`\n*docs.gl / OpenGL 4.6*\n\nPerforms an atomic addition on a shared variable or shader storage buffer object.",
        overloads: &[
            BuiltinOverload { label: "uint atomicAdd(inout uint mem, uint data)", params: &["inout uint mem", "uint data"] },
            BuiltinOverload { label: "int atomicAdd(inout int mem, int data)", params: &["inout int mem", "int data"] },
        ],
    },
];

pub fn lookup_builtin_function(name: &str) -> Option<&'static BuiltinFunction> {
    BUILTIN_FUNCTIONS.iter().find(|f| f.name == name)
}

pub fn get_all_builtins() -> &'static [BuiltinFunction] {
    BUILTIN_FUNCTIONS
}

// ------------------------------------------------------------------------
// GLSL Builtin Types
// ------------------------------------------------------------------------

pub struct BuiltinType {
    pub name: &'static str,
    pub detail: &'static str,
    pub description: &'static str,
    pub has_constructor: bool,
}

static BUILTIN_TYPES: &[BuiltinType] = &[
    // Scalars
    BuiltinType { name: "float", detail: "float", description: "### `float`\n*GLSL 4.6*\n\nIEEE 754 32-bit single-precision floating-point scalar type.", has_constructor: true },
    BuiltinType { name: "double", detail: "double", description: "### `double`\n*GLSL 4.6*\n\nIEEE 754 64-bit double-precision floating-point scalar type.", has_constructor: true },
    BuiltinType { name: "int", detail: "int", description: "### `int`\n*GLSL 4.6*\n\nSigned 32-bit two's complement integer scalar type.", has_constructor: true },
    BuiltinType { name: "uint", detail: "uint", description: "### `uint`\n*GLSL 4.6*\n\nUnsigned 32-bit integer scalar type.", has_constructor: true },
    BuiltinType { name: "bool", detail: "bool", description: "### `bool`\n*GLSL 4.6*\n\nBoolean scalar type (`true` or `false`).", has_constructor: true },
    BuiltinType { name: "void", detail: "void", description: "### `void`\n*GLSL 4.6*\n\nIndicates that a function does not return a value.", has_constructor: false },

    // Floating-Point Vectors
    BuiltinType { name: "vec2", detail: "vec2", description: "### `vec2`\n*GLSL 4.6*\n\n2-component 32-bit floating-point vector `(x, y)`.", has_constructor: true },
    BuiltinType { name: "vec3", detail: "vec3", description: "### `vec3`\n*GLSL 4.6*\n\n3-component 32-bit floating-point vector `(x, y, z)`.", has_constructor: true },
    BuiltinType { name: "vec4", detail: "vec4", description: "### `vec4`\n*GLSL 4.6*\n\n4-component 32-bit floating-point vector `(x, y, z, w)`.", has_constructor: true },

    // Double-Precision Vectors
    BuiltinType { name: "dvec2", detail: "dvec2", description: "### `dvec2`\n*GLSL 4.6*\n\n2-component 64-bit double-precision floating-point vector.", has_constructor: true },
    BuiltinType { name: "dvec3", detail: "dvec3", description: "### `dvec3`\n*GLSL 4.6*\n\n3-component 64-bit double-precision floating-point vector.", has_constructor: true },
    BuiltinType { name: "dvec4", detail: "dvec4", description: "### `dvec4`\n*GLSL 4.6*\n\n4-component 64-bit double-precision floating-point vector.", has_constructor: true },

    // Integer Vectors
    BuiltinType { name: "ivec2", detail: "ivec2", description: "### `ivec2`\n*GLSL 4.6*\n\n2-component 32-bit signed integer vector.", has_constructor: true },
    BuiltinType { name: "ivec3", detail: "ivec3", description: "### `ivec3`\n*GLSL 4.6*\n\n3-component 32-bit signed integer vector.", has_constructor: true },
    BuiltinType { name: "ivec4", detail: "ivec4", description: "### `ivec4`\n*GLSL 4.6*\n\n4-component 32-bit signed integer vector.", has_constructor: true },

    // Unsigned Integer Vectors
    BuiltinType { name: "uvec2", detail: "uvec2", description: "### `uvec2`\n*GLSL 4.6*\n\n2-component 32-bit unsigned integer vector.", has_constructor: true },
    BuiltinType { name: "uvec3", detail: "uvec3", description: "### `uvec3`\n*GLSL 4.6*\n\n3-component 32-bit unsigned integer vector.", has_constructor: true },
    BuiltinType { name: "uvec4", detail: "uvec4", description: "### `uvec4`\n*GLSL 4.6*\n\n4-component 32-bit unsigned integer vector.", has_constructor: true },

    // Boolean Vectors
    BuiltinType { name: "bvec2", detail: "bvec2", description: "### `bvec2`\n*GLSL 4.6*\n\n2-component boolean vector.", has_constructor: true },
    BuiltinType { name: "bvec3", detail: "bvec3", description: "### `bvec3`\n*GLSL 4.6*\n\n3-component boolean vector.", has_constructor: true },
    BuiltinType { name: "bvec4", detail: "bvec4", description: "### `bvec4`\n*GLSL 4.6*\n\n4-component boolean vector.", has_constructor: true },

    // Square Matrices
    BuiltinType { name: "mat2", detail: "mat2", description: "### `mat2`\n*GLSL 4.6*\n\n2x2 single-precision floating-point matrix.", has_constructor: true },
    BuiltinType { name: "mat3", detail: "mat3", description: "### `mat3`\n*GLSL 4.6*\n\n3x3 single-precision floating-point matrix.", has_constructor: true },
    BuiltinType { name: "mat4", detail: "mat4", description: "### `mat4`\n*GLSL 4.6*\n\n4x4 single-precision floating-point matrix.", has_constructor: true },

    // Non-Square Matrices
    BuiltinType { name: "mat2x2", detail: "mat2x2", description: "### `mat2x2`\n*GLSL 4.6*\n\n2 columns x 2 rows floating-point matrix.", has_constructor: true },
    BuiltinType { name: "mat2x3", detail: "mat2x3", description: "### `mat2x3`\n*GLSL 4.6*\n\n2 columns x 3 rows floating-point matrix.", has_constructor: true },
    BuiltinType { name: "mat2x4", detail: "mat2x4", description: "### `mat2x4`\n*GLSL 4.6*\n\n2 columns x 4 rows floating-point matrix.", has_constructor: true },
    BuiltinType { name: "mat3x2", detail: "mat3x2", description: "### `mat3x2`\n*GLSL 4.6*\n\n3 columns x 2 rows floating-point matrix.", has_constructor: true },
    BuiltinType { name: "mat3x3", detail: "mat3x3", description: "### `mat3x3`\n*GLSL 4.6*\n\n3 columns x 3 rows floating-point matrix.", has_constructor: true },
    BuiltinType { name: "mat3x4", detail: "mat3x4", description: "### `mat3x4`\n*GLSL 4.6*\n\n3 columns x 4 rows floating-point matrix.", has_constructor: true },
    BuiltinType { name: "mat4x2", detail: "mat4x2", description: "### `mat4x2`\n*GLSL 4.6*\n\n4 columns x 2 rows floating-point matrix.", has_constructor: true },
    BuiltinType { name: "mat4x3", detail: "mat4x3", description: "### `mat4x3`\n*GLSL 4.6*\n\n4 columns x 3 rows floating-point matrix.", has_constructor: true },
    BuiltinType { name: "mat4x4", detail: "mat4x4", description: "### `mat4x4`\n*GLSL 4.6*\n\n4 columns x 4 rows floating-point matrix.", has_constructor: true },

    // Double Matrices
    BuiltinType { name: "dmat2", detail: "dmat2", description: "### `dmat2`\n*GLSL 4.6*\n\n2x2 double-precision floating-point matrix.", has_constructor: true },
    BuiltinType { name: "dmat3", detail: "dmat3", description: "### `dmat3`\n*GLSL 4.6*\n\n3x3 double-precision floating-point matrix.", has_constructor: true },
    BuiltinType { name: "dmat4", detail: "dmat4", description: "### `dmat4`\n*GLSL 4.6*\n\n4x4 double-precision floating-point matrix.", has_constructor: true },
    BuiltinType { name: "dmat2x2", detail: "dmat2x2", description: "### `dmat2x2`\n*GLSL 4.6*\n\n2 columns x 2 rows double-precision matrix.", has_constructor: true },
    BuiltinType { name: "dmat2x3", detail: "dmat2x3", description: "### `dmat2x3`\n*GLSL 4.6*\n\n2 columns x 3 rows double-precision matrix.", has_constructor: true },
    BuiltinType { name: "dmat2x4", detail: "dmat2x4", description: "### `dmat2x4`\n*GLSL 4.6*\n\n2 columns x 4 rows double-precision matrix.", has_constructor: true },
    BuiltinType { name: "dmat3x2", detail: "dmat3x2", description: "### `dmat3x2`\n*GLSL 4.6*\n\n3 columns x 2 rows double-precision matrix.", has_constructor: true },
    BuiltinType { name: "dmat3x3", detail: "dmat3x3", description: "### `dmat3x3`\n*GLSL 4.6*\n\n3 columns x 3 rows double-precision matrix.", has_constructor: true },
    BuiltinType { name: "dmat3x4", detail: "dmat3x4", description: "### `dmat3x4`\n*GLSL 4.6*\n\n3 columns x 4 rows double-precision matrix.", has_constructor: true },
    BuiltinType { name: "dmat4x2", detail: "dmat4x2", description: "### `dmat4x2`\n*GLSL 4.6*\n\n4 columns x 2 rows double-precision matrix.", has_constructor: true },
    BuiltinType { name: "dmat4x3", detail: "dmat4x3", description: "### `dmat4x3`\n*GLSL 4.6*\n\n4 columns x 3 rows double-precision matrix.", has_constructor: true },
    BuiltinType { name: "dmat4x4", detail: "dmat4x4", description: "### `dmat4x4`\n*GLSL 4.6*\n\n4 columns x 4 rows double-precision matrix.", has_constructor: true },

    // Floating-Point Samplers
    BuiltinType { name: "sampler1D", detail: "sampler1D", description: "### `sampler1D`\n*GLSL 4.6*\n\nHandle for 1D texture sampling.", has_constructor: false },
    BuiltinType { name: "sampler2D", detail: "sampler2D", description: "### `sampler2D`\n*GLSL 4.6*\n\nHandle for 2D texture sampling.", has_constructor: false },
    BuiltinType { name: "sampler3D", detail: "sampler3D", description: "### `sampler3D`\n*GLSL 4.6*\n\nHandle for 3D texture sampling.", has_constructor: false },
    BuiltinType { name: "samplerCube", detail: "samplerCube", description: "### `samplerCube`\n*GLSL 4.6*\n\nHandle for cubemap texture sampling.", has_constructor: false },
    BuiltinType { name: "sampler2DShadow", detail: "sampler2DShadow", description: "### `sampler2DShadow`\n*GLSL 4.6*\n\nHandle for 2D depth texture sampling with comparison.", has_constructor: false },
    BuiltinType { name: "samplerCubeShadow", detail: "samplerCubeShadow", description: "### `samplerCubeShadow`\n*GLSL 4.6*\n\nHandle for cubemap depth texture sampling with comparison.", has_constructor: false },
    BuiltinType { name: "sampler2DArray", detail: "sampler2DArray", description: "### `sampler2DArray`\n*GLSL 4.6*\n\nHandle for 2D array texture sampling.", has_constructor: false },
    BuiltinType { name: "sampler2DArrayShadow", detail: "sampler2DArrayShadow", description: "### `sampler2DArrayShadow`\n*GLSL 4.6*\n\nHandle for 2D array depth texture sampling with comparison.", has_constructor: false },
    BuiltinType { name: "sampler1DArray", detail: "sampler1DArray", description: "### `sampler1DArray`\n*GLSL 4.6*\n\nHandle for 1D array texture sampling.", has_constructor: false },
    BuiltinType { name: "sampler1DArrayShadow", detail: "sampler1DArrayShadow", description: "### `sampler1DArrayShadow`\n*GLSL 4.6*\n\nHandle for 1D array depth texture sampling with comparison.", has_constructor: false },
    BuiltinType { name: "sampler2DMS", detail: "sampler2DMS", description: "### `sampler2DMS`\n*GLSL 4.6*\n\nHandle for multisample 2D texture fetching.", has_constructor: false },
    BuiltinType { name: "sampler2DMSArray", detail: "sampler2DMSArray", description: "### `sampler2DMSArray`\n*GLSL 4.6*\n\nHandle for multisample 2D array texture fetching.", has_constructor: false },
    BuiltinType { name: "samplerBuffer", detail: "samplerBuffer", description: "### `samplerBuffer`\n*GLSL 4.6*\n\nHandle for buffer texture fetching.", has_constructor: false },
    BuiltinType { name: "sampler2DRect", detail: "sampler2DRect", description: "### `sampler2DRect`\n*GLSL 4.6*\n\nHandle for rectangular (unnormalized) 2D texture sampling.", has_constructor: false },
    BuiltinType { name: "sampler2DRectShadow", detail: "sampler2DRectShadow", description: "### `sampler2DRectShadow`\n*GLSL 4.6*\n\nHandle for rectangular 2D depth texture sampling with comparison.", has_constructor: false },

    // Integer Samplers
    BuiltinType { name: "isampler1D", detail: "isampler1D", description: "### `isampler1D`\n*GLSL 4.6*\n\nHandle for integer 1D texture sampling.", has_constructor: false },
    BuiltinType { name: "isampler2D", detail: "isampler2D", description: "### `isampler2D`\n*GLSL 4.6*\n\nHandle for integer 2D texture sampling.", has_constructor: false },
    BuiltinType { name: "isampler3D", detail: "isampler3D", description: "### `isampler3D`\n*GLSL 4.6*\n\nHandle for integer 3D texture sampling.", has_constructor: false },
    BuiltinType { name: "isamplerCube", detail: "isamplerCube", description: "### `isamplerCube`\n*GLSL 4.6*\n\nHandle for integer cubemap texture sampling.", has_constructor: false },
    BuiltinType { name: "isampler2DArray", detail: "isampler2DArray", description: "### `isampler2DArray`\n*GLSL 4.6*\n\nHandle for integer 2D array texture sampling.", has_constructor: false },
    BuiltinType { name: "isampler2DMS", detail: "isampler2DMS", description: "### `isampler2DMS`\n*GLSL 4.6*\n\nHandle for integer multisample 2D texture fetching.", has_constructor: false },
    BuiltinType { name: "isampler2DMSArray", detail: "isampler2DMSArray", description: "### `isampler2DMSArray`\n*GLSL 4.6*\n\nHandle for integer multisample 2D array texture fetching.", has_constructor: false },
    BuiltinType { name: "isamplerBuffer", detail: "isamplerBuffer", description: "### `isamplerBuffer`\n*GLSL 4.6*\n\nHandle for integer buffer texture fetching.", has_constructor: false },
    BuiltinType { name: "isampler2DRect", detail: "isampler2DRect", description: "### `isampler2DRect`\n*GLSL 4.6*\n\nHandle for integer rectangular 2D texture sampling.", has_constructor: false },

    // Unsigned Integer Samplers
    BuiltinType { name: "usampler1D", detail: "usampler1D", description: "### `usampler1D`\n*GLSL 4.6*\n\nHandle for unsigned integer 1D texture sampling.", has_constructor: false },
    BuiltinType { name: "usampler2D", detail: "usampler2D", description: "### `usampler2D`\n*GLSL 4.6*\n\nHandle for unsigned integer 2D texture sampling.", has_constructor: false },
    BuiltinType { name: "usampler3D", detail: "usampler3D", description: "### `usampler3D`\n*GLSL 4.6*\n\nHandle for unsigned integer 3D texture sampling.", has_constructor: false },
    BuiltinType { name: "usamplerCube", detail: "usamplerCube", description: "### `usamplerCube`\n*GLSL 4.6*\n\nHandle for unsigned integer cubemap texture sampling.", has_constructor: false },
    BuiltinType { name: "usampler2DArray", detail: "usampler2DArray", description: "### `usampler2DArray`\n*GLSL 4.6*\n\nHandle for unsigned integer 2D array texture sampling.", has_constructor: false },
    BuiltinType { name: "usampler2DMS", detail: "usampler2DMS", description: "### `usampler2DMS`\n*GLSL 4.6*\n\nHandle for unsigned integer multisample 2D texture fetching.", has_constructor: false },
    BuiltinType { name: "usampler2DMSArray", detail: "usampler2DMSArray", description: "### `usampler2DMSArray`\n*GLSL 4.6*\n\nHandle for unsigned integer multisample 2D array texture fetching.", has_constructor: false },
    BuiltinType { name: "usamplerBuffer", detail: "usamplerBuffer", description: "### `usamplerBuffer`\n*GLSL 4.6*\n\nHandle for unsigned integer buffer texture fetching.", has_constructor: false },
    BuiltinType { name: "usampler2DRect", detail: "usampler2DRect", description: "### `usampler2DRect`\n*GLSL 4.6*\n\nHandle for unsigned integer rectangular 2D texture sampling.", has_constructor: false },

    // Images
    BuiltinType { name: "image1D", detail: "image1D", description: "### `image1D`\n*GLSL 4.6*\n\nHandle for 1D image load, store, and atomic operations.", has_constructor: false },
    BuiltinType { name: "image2D", detail: "image2D", description: "### `image2D`\n*GLSL 4.6*\n\nHandle for 2D image load, store, and atomic operations.", has_constructor: false },
    BuiltinType { name: "image3D", detail: "image3D", description: "### `image3D`\n*GLSL 4.6*\n\nHandle for 3D image load, store, and atomic operations.", has_constructor: false },
    BuiltinType { name: "imageCube", detail: "imageCube", description: "### `imageCube`\n*GLSL 4.6*\n\nHandle for cubemap image load, store, and atomic operations.", has_constructor: false },
    BuiltinType { name: "image2DArray", detail: "image2DArray", description: "### `image2DArray`\n*GLSL 4.6*\n\nHandle for 2D array image load, store, and atomic operations.", has_constructor: false },
    BuiltinType { name: "imageBuffer", detail: "imageBuffer", description: "### `imageBuffer`\n*GLSL 4.6*\n\nHandle for buffer image load, store, and atomic operations.", has_constructor: false },
    BuiltinType { name: "image2DRect", detail: "image2DRect", description: "### `image2DRect`\n*GLSL 4.6*\n\nHandle for rectangular 2D image load, store, and atomic operations.", has_constructor: false },

    // Atomics & Vulkan Subpass
    BuiltinType { name: "atomic_uint", detail: "atomic_uint", description: "### `atomic_uint`\n*GLSL 4.6*\n\nUnsigned atomic counter type.", has_constructor: false },
    BuiltinType { name: "subpassInput", detail: "subpassInput", description: "### `subpassInput`\n*Vulkan GLSL*\n\nHandle for Vulkan subpass input attachment.", has_constructor: false },
    BuiltinType { name: "subpassInputMS", detail: "subpassInputMS", description: "### `subpassInputMS`\n*Vulkan GLSL*\n\nHandle for Vulkan multisampled subpass input attachment.", has_constructor: false },
];

pub fn get_all_types() -> &'static [BuiltinType] {
    BUILTIN_TYPES
}

// ------------------------------------------------------------------------
// GLSL Builtin Keywords & Qualifiers
// ------------------------------------------------------------------------

pub struct BuiltinKeyword {
    pub name: &'static str,
    pub detail: &'static str,
    pub description: &'static str,
}

static BUILTIN_KEYWORDS: &[BuiltinKeyword] = &[
    // Qualifiers & Storage
    BuiltinKeyword { name: "layout", detail: "layout(...)", description: "### `layout`\n*GLSL 4.6*\n\nSpecifies layout qualifiers for variables, interfaces, bindings, or buffer blocks (e.g. `layout(location = 0)` or `layout(binding = 0)`)." },
    BuiltinKeyword { name: "binding", detail: "layout(binding = ...)", description: "### `binding`\n*GLSL 4.6*\n\nLayout qualifier specifying the binding point index of a uniform block, buffer block, or texture sampler." },
    BuiltinKeyword { name: "location", detail: "layout(location = ...)", description: "### `location`\n*GLSL 4.6*\n\nLayout qualifier specifying the input or output location index for vertex or fragment stage interface variables." },
    BuiltinKeyword { name: "set", detail: "layout(set = ...)", description: "### `set`\n*Vulkan GLSL*\n\nLayout qualifier specifying the descriptor set index for a resource (e.g. `layout(set = 0, binding = 0)`)." },
    BuiltinKeyword { name: "push_constant", detail: "layout(push_constant)", description: "### `push_constant`\n*Vulkan GLSL*\n\nLayout qualifier declaring a uniform block backed by Vulkan push constants." },
    BuiltinKeyword { name: "offset", detail: "layout(offset = ...)", description: "### `offset`\n*GLSL 4.6*\n\nLayout qualifier specifying member byte offset within a uniform or storage buffer block." },
    BuiltinKeyword { name: "std140", detail: "layout(std140)", description: "### `std140`\n*GLSL 4.6*\n\nStandard packing layout rule for uniform blocks (OpenGL Standard 140)." },
    BuiltinKeyword { name: "std430", detail: "layout(std430)", description: "### `std430`\n*GLSL 4.6*\n\nStandard packing layout rule for shader storage buffer objects (SSBO), with tighter packing for arrays and matrices." },
    BuiltinKeyword { name: "uniform", detail: "uniform <type> <name>", description: "### `uniform`\n*GLSL 4.6*\n\nDeclares a read-only variable whose value is supplied by the application and constant across all shader invocations within a draw call." },
    BuiltinKeyword { name: "buffer", detail: "buffer <block_name> { ... }", description: "### `buffer`\n*GLSL 4.6*\n\nDeclares a Shader Storage Buffer Object (SSBO) with read/write capability." },
    BuiltinKeyword { name: "in", detail: "in <type> <name>", description: "### `in`\n*GLSL 4.6*\n\nDeclares an input variable passed from the previous pipeline stage or vertex attributes." },
    BuiltinKeyword { name: "out", detail: "out <type> <name>", description: "### `out`\n*GLSL 4.6*\n\nDeclares an output variable passed to the next pipeline stage or framebuffer." },
    BuiltinKeyword { name: "inout", detail: "inout <type> <param>", description: "### `inout`\n*GLSL 4.6*\n\nFunction parameter qualifier indicating that the parameter is passed by reference (read and written)." },
    BuiltinKeyword { name: "const", detail: "const <type> <name>", description: "### `const`\n*GLSL 4.6*\n\nDeclares a compile-time constant or read-only variable." },
    BuiltinKeyword { name: "flat", detail: "flat in/out", description: "### `flat`\n*GLSL 4.6*\n\nInterpolation qualifier: no interpolation across the primitive; value from the provoking vertex is used." },
    BuiltinKeyword { name: "smooth", detail: "smooth in/out", description: "### `smooth`\n*GLSL 4.6*\n\nInterpolation qualifier: perspective-correct interpolation across primitives (default)." },
    BuiltinKeyword { name: "noperspective", detail: "noperspective in/out", description: "### `noperspective`\n*GLSL 4.6*\n\nInterpolation qualifier: linear interpolation in screen space without perspective correction." },
    BuiltinKeyword { name: "centroid", detail: "centroid in/out", description: "### `centroid`\n*GLSL 4.6*\n\nInterpolation qualifier: evaluates the variable at a point within the primitive's covered area during multisampling." },
    BuiltinKeyword { name: "sample", detail: "sample in/out", description: "### `sample`\n*GLSL 4.6*\n\nInterpolation qualifier: causes per-sample interpolation and evaluation during multisampling." },
    BuiltinKeyword { name: "patch", detail: "patch in/out", description: "### `patch`\n*GLSL 4.6*\n\nTessellation stage interface qualifier for per-patch attributes." },
    BuiltinKeyword { name: "coherent", detail: "coherent", description: "### `coherent`\n*GLSL 4.6*\n\nMemory qualifier ensuring reads and writes to buffer or image variables are visible across shader invocations." },
    BuiltinKeyword { name: "volatile", detail: "volatile", description: "### `volatile`\n*GLSL 4.6*\n\nMemory qualifier indicating that variable values may change asynchronously in memory." },
    BuiltinKeyword { name: "restrict", detail: "restrict", description: "### `restrict`\n*GLSL 4.6*\n\nMemory qualifier hinting that the underlying memory is accessed solely through this variable." },
    BuiltinKeyword { name: "readonly", detail: "readonly", description: "### `readonly`\n*GLSL 4.6*\n\nMemory qualifier specifying that a buffer or image variable can only be read." },
    BuiltinKeyword { name: "writeonly", detail: "writeonly", description: "### `writeonly`\n*GLSL 4.6*\n\nMemory qualifier specifying that a buffer or image variable can only be written." },
    BuiltinKeyword { name: "precision", detail: "precision <qualifier> <type>", description: "### `precision`\n*GLSL 4.6*\n\nSets default precision for float or integer types (`highp`, `mediump`, `lowp`)." },
    BuiltinKeyword { name: "highp", detail: "highp", description: "### `highp`\n*GLSL 4.6*\n\nHigh-precision qualifier (at least 32-bit floating point)." },
    BuiltinKeyword { name: "mediump", detail: "mediump", description: "### `mediump`\n*GLSL 4.6*\n\nMedium-precision qualifier (typically 16-bit floating point)." },
    BuiltinKeyword { name: "lowp", detail: "lowp", description: "### `lowp`\n*GLSL 4.6*\n\nLow-precision qualifier (typically 8-bit to 10-bit fixed point)." },
    BuiltinKeyword { name: "invariant", detail: "invariant <name>", description: "### `invariant`\n*GLSL 4.6*\n\nGuarantees that an output variable's value is calculated identically across different shaders." },
    BuiltinKeyword { name: "precise", detail: "precise <name>", description: "### `precise`\n*GLSL 4.6*\n\nPrevents compiler optimizations that could change arithmetic evaluation order or precision." },
    BuiltinKeyword { name: "struct", detail: "struct <name> { ... }", description: "### `struct`\n*GLSL 4.6*\n\nDefines a user-defined composite data structure." },
    BuiltinKeyword { name: "subroutine", detail: "subroutine", description: "### `subroutine`\n*GLSL 4.6*\n\nDeclares dynamic shader subroutine function types and uniform selectors." },

    // Control Flow
    BuiltinKeyword { name: "return", detail: "return [value];", description: "### `return`\n*GLSL 4.6*\n\nReturns control and an optional result value from the current function." },
    BuiltinKeyword { name: "discard", detail: "discard;", description: "### `discard`\n*GLSL 4.6*\n\nTerminates execution of the current fragment shader invocation and discards the fragment so it is not written to the framebuffer." },
    BuiltinKeyword { name: "break", detail: "break;", description: "### `break`\n*GLSL 4.6*\n\nTerminates execution of the innermost loop (`for`, `while`, `do`) or `switch` statement." },
    BuiltinKeyword { name: "continue", detail: "continue;", description: "### `continue`\n*GLSL 4.6*\n\nSkips the remainder of the current loop iteration and proceeds to the next iteration." },
    BuiltinKeyword { name: "if", detail: "if (condition) { ... }", description: "### `if`\n*GLSL 4.6*\n\nExecutes a code block if the specified boolean condition evaluates to true." },
    BuiltinKeyword { name: "else", detail: "else { ... }", description: "### `else`\n*GLSL 4.6*\n\nExecutes a code block if the preceding `if` condition evaluated to false." },
    BuiltinKeyword { name: "for", detail: "for (init; cond; step) { ... }", description: "### `for`\n*GLSL 4.6*\n\nExecutes a code block repeatedly while the condition evaluates to true." },
    BuiltinKeyword { name: "while", detail: "while (condition) { ... }", description: "### `while`\n*GLSL 4.6*\n\nExecutes a code block repeatedly as long as the condition evaluates to true." },
    BuiltinKeyword { name: "do", detail: "do { ... } while (condition);", description: "### `do ... while`\n*GLSL 4.6*\n\nExecutes a code block at least once, then repeats as long as the condition is true." },
    BuiltinKeyword { name: "switch", detail: "switch (expr) { ... }", description: "### `switch`\n*GLSL 4.6*\n\nEvaluates an integer expression and transfers control to matching `case` or `default` label." },
    BuiltinKeyword { name: "case", detail: "case <constant>:", description: "### `case`\n*GLSL 4.6*\n\nDefines a branch target within a `switch` statement." },
    BuiltinKeyword { name: "default", detail: "default:", description: "### `default`\n*GLSL 4.6*\n\nDefines the fallback branch target within a `switch` statement when no `case` matches." },
];

pub fn get_all_keywords() -> &'static [BuiltinKeyword] {
    BUILTIN_KEYWORDS
}

// ------------------------------------------------------------------------
// GLSL Builtin Variables
// ------------------------------------------------------------------------

pub struct BuiltinVariable {
    pub name: &'static str,
    pub var_type: &'static str,
    pub stage: &'static str,
    pub description: &'static str,
}

static BUILTIN_VARIABLES: &[BuiltinVariable] = &[
    // Vertex Stage
    BuiltinVariable { name: "gl_Position", var_type: "vec4", stage: "vert", description: "### `gl_Position`\n*out vec4 gl_Position*\n\nHomogeneous clip-space coordinates of the current vertex output." },
    BuiltinVariable { name: "gl_PointSize", var_type: "float", stage: "vert", description: "### `gl_PointSize`\n*out float gl_PointSize*\n\nSpecifies the rasterized diameter in pixels of point primitives." },
    BuiltinVariable { name: "gl_ClipDistance", var_type: "float[]", stage: "vert", description: "### `gl_ClipDistance`\n*out float gl_ClipDistance[]*\n\nSpecifies clip distance values against user-defined clipping planes." },
    BuiltinVariable { name: "gl_CullDistance", var_type: "float[]", stage: "vert", description: "### `gl_CullDistance`\n*out float gl_CullDistance[]*\n\nSpecifies cull distance values against user-defined culling planes." },
    BuiltinVariable { name: "gl_VertexIndex", var_type: "int", stage: "vert", description: "### `gl_VertexIndex`\n*in int gl_VertexIndex*\n\nVulkan index of the vertex currently being processed." },
    BuiltinVariable { name: "gl_InstanceIndex", var_type: "int", stage: "vert", description: "### `gl_InstanceIndex`\n*in int gl_InstanceIndex*\n\nVulkan index of the current instance in an instanced draw call." },
    BuiltinVariable { name: "gl_VertexID", var_type: "int", stage: "vert", description: "### `gl_VertexID`\n*in int gl_VertexID*\n\nOpenGL index of the vertex currently being processed." },
    BuiltinVariable { name: "gl_InstanceID", var_type: "int", stage: "vert", description: "### `gl_InstanceID`\n*in int gl_InstanceID*\n\nOpenGL index of the current instance in an instanced draw call." },

    // Fragment Stage
    BuiltinVariable { name: "gl_FragCoord", var_type: "vec4", stage: "frag", description: "### `gl_FragCoord`\n*in vec4 gl_FragCoord*\n\nWindow-relative coordinates of the current fragment `(x, y, z, 1/w)`." },
    BuiltinVariable { name: "gl_FragColor", var_type: "vec4", stage: "frag", description: "### `gl_FragColor`\n*out vec4 gl_FragColor*\n\nLegacy fragment color output (OpenGL core shaders prefer user-defined `out vec4 fragColor`)." },
    BuiltinVariable { name: "gl_FragDepth", var_type: "float", stage: "frag", description: "### `gl_FragDepth`\n*out float gl_FragDepth*\n\nFragment depth value written to depth buffer; overrides interpolated depth." },
    BuiltinVariable { name: "gl_FrontFacing", var_type: "bool", stage: "frag", description: "### `gl_FrontFacing`\n*in bool gl_FrontFacing*\n\nTrue if the current fragment belongs to a front-facing primitive." },
    BuiltinVariable { name: "gl_PointCoord", var_type: "vec2", stage: "frag", description: "### `gl_PointCoord`\n*in vec2 gl_PointCoord*\n\nTwo-dimensional coordinates `[0, 1]` within a point primitive." },
    BuiltinVariable { name: "gl_SampleID", var_type: "int", stage: "frag", description: "### `gl_SampleID`\n*in int gl_SampleID*\n\nSample number of the current fragment during per-sample shading." },
    BuiltinVariable { name: "gl_SamplePosition", var_type: "vec2", stage: "frag", description: "### `gl_SamplePosition`\n*in vec2 gl_SamplePosition*\n\nSub-pixel offset `[0, 1]` of the sample currently being evaluated." },
    BuiltinVariable { name: "gl_SampleMaskIn", var_type: "int[]", stage: "frag", description: "### `gl_SampleMaskIn`\n*in int gl_SampleMaskIn[]*\n\nBitmask of samples covered by the primitive generating the fragment." },
    BuiltinVariable { name: "gl_SampleMask", var_type: "int[]", stage: "frag", description: "### `gl_SampleMask`\n*out int gl_SampleMask[]*\n\nOutput bitmask specifying which samples of the fragment should be written." },
    BuiltinVariable { name: "gl_PrimitiveID", var_type: "int", stage: "frag", description: "### `gl_PrimitiveID`\n*in int gl_PrimitiveID*\n\nID of the primitive generated by earlier pipeline stages." },
    BuiltinVariable { name: "gl_Layer", var_type: "int", stage: "frag", description: "### `gl_Layer`\n*out/in int gl_Layer*\n\nLayer index for layered rendering / cube map faces." },
    BuiltinVariable { name: "gl_ViewportIndex", var_type: "int", stage: "frag", description: "### `gl_ViewportIndex`\n*out/in int gl_ViewportIndex*\n\nViewport index to which the primitive was directed." },

    // Compute Stage
    BuiltinVariable { name: "gl_NumWorkGroups", var_type: "uvec3", stage: "comp", description: "### `gl_NumWorkGroups`\n*in uvec3 gl_NumWorkGroups*\n\nTotal number of work groups dispatched in each dimension." },
    BuiltinVariable { name: "gl_WorkGroupSize", var_type: "uvec3", stage: "comp", description: "### `gl_WorkGroupSize`\n*in uvec3 gl_WorkGroupSize*\n\nDimensions of a local work group defined by `layout(local_size_x = ...)`." },
    BuiltinVariable { name: "gl_WorkGroupID", var_type: "uvec3", stage: "comp", description: "### `gl_WorkGroupID`\n*in uvec3 gl_WorkGroupID*\n\nThree-dimensional index of the current work group being executed." },
    BuiltinVariable { name: "gl_LocalInvocationID", var_type: "uvec3", stage: "comp", description: "### `gl_LocalInvocationID`\n*in uvec3 gl_LocalInvocationID*\n\nThree-dimensional index of the invocation within the local work group." },
    BuiltinVariable { name: "gl_GlobalInvocationID", var_type: "uvec3", stage: "comp", description: "### `gl_GlobalInvocationID`\n*in uvec3 gl_GlobalInvocationID*\n\nUnique global 3D index: `gl_WorkGroupID * gl_WorkGroupSize + gl_LocalInvocationID`." },
    BuiltinVariable { name: "gl_LocalInvocationIndex", var_type: "uint", stage: "comp", description: "### `gl_LocalInvocationIndex`\n*in uint gl_LocalInvocationIndex*\n\n1D flattened index of the current invocation within the local work group." },
];

pub fn get_all_variables() -> &'static [BuiltinVariable] {
    BUILTIN_VARIABLES
}

// ------------------------------------------------------------------------
// GLSL Preprocessor Directives
// ------------------------------------------------------------------------

pub struct BuiltinDirective {
    pub name: &'static str,
    pub detail: &'static str,
    pub description: &'static str,
}

static BUILTIN_DIRECTIVES: &[BuiltinDirective] = &[
    BuiltinDirective { name: "#version", detail: "#version 460 core", description: "### `#version`\n*GLSL Preprocessor*\n\nSpecifies GLSL shading language version (e.g. `#version 460 core`)." },
    BuiltinDirective { name: "#include", detail: "#include \"header.glsl\"", description: "### `#include`\n*GLSL Preprocessor / GL_GOOGLE_include_directive*\n\nIncludes an external GLSL header file." },
    BuiltinDirective { name: "#define", detail: "#define NAME VALUE", description: "### `#define`\n*GLSL Preprocessor*\n\nDefines a preprocessor macro or constant." },
    BuiltinDirective { name: "#undef", detail: "#undef NAME", description: "### `#undef`\n*GLSL Preprocessor*\n\nUndefines a previously defined preprocessor macro." },
    BuiltinDirective { name: "#if", detail: "#if EXPRESSION", description: "### `#if`\n*GLSL Preprocessor*\n\nConditional preprocessor compilation." },
    BuiltinDirective { name: "#ifdef", detail: "#ifdef NAME", description: "### `#ifdef`\n*GLSL Preprocessor*\n\nCompiles block if macro NAME is defined." },
    BuiltinDirective { name: "#ifndef", detail: "#ifndef NAME", description: "### `#ifndef`\n*GLSL Preprocessor*\n\nCompiles block if macro NAME is not defined." },
    BuiltinDirective { name: "#elif", detail: "#elif EXPRESSION", description: "### `#elif`\n*GLSL Preprocessor*\n\nElse-if conditional preprocessor branch." },
    BuiltinDirective { name: "#else", detail: "#else", description: "### `#else`\n*GLSL Preprocessor*\n\nElse conditional preprocessor branch." },
    BuiltinDirective { name: "#endif", detail: "#endif", description: "### `#endif`\n*GLSL Preprocessor*\n\nCloses conditional preprocessor block." },
    BuiltinDirective { name: "#extension", detail: "#extension EXT : enable", description: "### `#extension`\n*GLSL Preprocessor*\n\nEnables, disables, or requires a GLSL extension." },
    BuiltinDirective { name: "#pragma", detail: "#pragma ...", description: "### `#pragma`\n*GLSL Preprocessor*\n\nCompiler directive or target configuration." },
    BuiltinDirective { name: "#line", detail: "#line NUMBER", description: "### `#line`\n*GLSL Preprocessor*\n\nSets line number for compiler diagnostic reporting." },
    BuiltinDirective { name: "#error", detail: "#error MESSAGE", description: "### `#error`\n*GLSL Preprocessor*\n\nEmits a compiler error with the specified message." },
];

pub fn get_all_directives() -> &'static [BuiltinDirective] {
    BUILTIN_DIRECTIVES
}

