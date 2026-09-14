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
];

pub fn lookup_builtin_function(name: &str) -> Option<&'static BuiltinFunction> {
    BUILTIN_FUNCTIONS.iter().find(|f| f.name == name)
}

pub fn get_all_builtins() -> &'static [BuiltinFunction] {
    BUILTIN_FUNCTIONS
}

