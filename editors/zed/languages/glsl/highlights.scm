; GLSL highlights.scm
; Base: Zed official glsl extension (zed-industries/zed,
; extensions/glsl/languages/glsl/highlights.scm) - verified node types only.
; Additions: GLSL-specific types and built-ins matched via (#match?) on
; identifier nodes - no custom node types needed, no "Invalid node type" risk.

[
  "break"
  "case"
  "const"
  "continue"
  "default"
  "do"
  "else"
  "enum"
  "extern"
  "for"
  "if"
  "inline"
  "return"
  "sizeof"
  "static"
  "struct"
  "switch"
  "typedef"
  "union"
  "volatile"
  "while"
  "#define"
  "#elif"
  "#else"
  "#endif"
  "#if"
  "#ifdef"
  "#ifndef"
  "#include"
  (preproc_directive)
] @keyword

[
  "--"
  "-"
  "-="
  "->"
  "="
  "!="
  "*"
  "&"
  "&&"
  "+"
  "++"
  "+="
  "<"
  "=="
  ">"
  "||"
  "."
  ";"
] @operator

[
  (string_literal)
  (system_lib_string)
] @string

(null) @constant.builtin

[
  (number_literal)
  (char_literal)
] @number

(identifier) @variable

(field_identifier) @property

(statement_identifier) @label

[
  (type_identifier)
  (primitive_type)
  (sized_type_specifier)
] @type

(call_expression
  function: (identifier) @function)

(call_expression
  function: (field_expression
    field: (field_identifier) @function))

(function_declarator
  declarator: (identifier) @function)

(preproc_function_def
  name: (identifier) @function.special)

((identifier) @constant
  (#match? @constant "^[A-Z][A-Z\\d_]*$"))

(comment) @comment

; GLSL storage / precision qualifiers (defined in grammar.js rules)
[
  "in"
  "out"
  "inout"
  "uniform"
  "shared"
  "layout"
  "attribute"
  "varying"
  "buffer"
  "coherent"
  "readonly"
  "writeonly"
  "precision"
  "highp"
  "mediump"
  "lowp"
  "centroid"
  "sample"
  "patch"
  "smooth"
  "flat"
  "noperspective"
  "invariant"
  "precise"
] @type.qualifier

"subroutine" @keyword.function

; Ray tracing / extension storage classes (grammar.js extension_storage_class node)
(extension_storage_class) @storageclass

; gl_* built-in variables
((identifier) @variable.builtin
  (#match? @variable.builtin "^gl_"))

; --- GLSL built-in scalar/vector/matrix types (via #match? on identifier) ---

((identifier) @type.builtin
 (#match? @type.builtin "^(void|bool|int|uint|float|double)$"))

((identifier) @type.builtin
 (#match? @type.builtin "^(vec2|vec3|vec4|ivec2|ivec3|ivec4|uvec2|uvec3|uvec4|bvec2|bvec3|bvec4|dvec2|dvec3|dvec4)$"))

((identifier) @type.builtin
 (#match? @type.builtin "^(mat2|mat3|mat4|mat2x2|mat2x3|mat2x4|mat3x2|mat3x3|mat3x4|mat4x2|mat4x3|mat4x4|dmat2|dmat3|dmat4)$"))

((identifier) @type.builtin
 (#match? @type.builtin "^(sampler1D|sampler2D|sampler3D|samplerCube|sampler2DRect|sampler1DArray|sampler2DArray|samplerCubeArray|samplerBuffer|sampler2DMS|sampler2DMSArray)$"))

((identifier) @type.builtin
 (#match? @type.builtin "^(sampler1DShadow|sampler2DShadow|samplerCubeShadow|sampler2DRectShadow|sampler1DArrayShadow|sampler2DArrayShadow|samplerCubeArrayShadow)$"))

((identifier) @type.builtin
 (#match? @type.builtin "^(isampler1D|isampler2D|isampler3D|isamplerCube|isampler2DArray|usampler1D|usampler2D|usampler3D|usamplerCube|usampler2DArray)$"))

((identifier) @type.builtin
 (#match? @type.builtin "^(image1D|image2D|image3D|imageCube|image2DArray|imageBuffer|iimage1D|iimage2D|iimage3D|uimage1D|uimage2D|uimage3D)$"))

((identifier) @type.builtin
 (#match? @type.builtin "^(atomic_uint)$"))

; --- GLSL built-in functions (via #match? on call_expression function) ---

((call_expression function: (identifier) @function.builtin)
 (#match? @function.builtin "^(radians|degrees|sin|cos|tan|asin|acos|atan|sinh|cosh|tanh|asinh|acosh|atanh)$"))

((call_expression function: (identifier) @function.builtin)
 (#match? @function.builtin "^(pow|exp|log|exp2|log2|sqrt|inversesqrt)$"))

((call_expression function: (identifier) @function.builtin)
 (#match? @function.builtin "^(abs|sign|floor|trunc|round|roundEven|ceil|fract|mod|modf|min|max|clamp|mix|step|smoothstep|isnan|isinf|floatBitsToInt|floatBitsToUint|intBitsToFloat|uintBitsToFloat|fma|frexp|ldexp)$"))

((call_expression function: (identifier) @function.builtin)
 (#match? @function.builtin "^(packSnorm2x16|unpackSnorm2x16|packUnorm2x16|unpackUnorm2x16|packHalf2x16|unpackHalf2x16|packUnorm4x8|unpackUnorm4x8|packSnorm4x8|unpackSnorm4x8)$"))

((call_expression function: (identifier) @function.builtin)
 (#match? @function.builtin "^(length|distance|dot|cross|normalize|faceforward|reflect|refract|matrixCompMult|outerProduct|transpose|determinant|inverse)$"))

((call_expression function: (identifier) @function.builtin)
 (#match? @function.builtin "^(lessThan|lessThanEqual|greaterThan|greaterThanEqual|equal|notEqual|any|all|not)$"))

((call_expression function: (identifier) @function.builtin)
 (#match? @function.builtin "^(uaddCarry|usubBorrow|umulExtended|imulExtended|bitfieldExtract|bitfieldInsert|bitfieldReverse|bitCount|findLSB|findMSB)$"))

((call_expression function: (identifier) @function.builtin)
 (#match? @function.builtin "^(texture|textureProj|textureLod|textureOffset|texelFetch|texelFetchOffset|textureProjOffset|textureLodOffset|textureProjLod|textureProjLodOffset|textureGrad|textureGradOffset|textureProjGrad|textureProjGradOffset|textureGather|textureGatherOffset|textureQueryLod|textureQueryLevels|textureSize|textureSamples)$"))

((call_expression function: (identifier) @function.builtin)
 (#match? @function.builtin "^(imageLoad|imageStore|imageSize|imageSamples|imageAtomicAdd|imageAtomicMin|imageAtomicMax|imageAtomicAnd|imageAtomicOr|imageAtomicXor|imageAtomicExchange|imageAtomicCompSwap)$"))

((call_expression function: (identifier) @function.builtin)
 (#match? @function.builtin "^(dFdx|dFdy|dFdxCoarse|dFdyCoarse|dFdxFine|dFdyFine|fwidth|fwidthCoarse|fwidthFine|interpolateAtCentroid|interpolateAtSample|interpolateAtOffset)$"))

((call_expression function: (identifier) @function.builtin)
 (#match? @function.builtin "^(EmitVertex|EndPrimitive|EmitStreamVertex|EndStreamPrimitive|barrier|memoryBarrier|memoryBarrierAtomicCounter|memoryBarrierBuffer|memoryBarrierShared|memoryBarrierImage|groupMemoryBarrier)$"))

((call_expression function: (identifier) @function.builtin)
 (#match? @function.builtin "^(atomicAdd|atomicMin|atomicMax|atomicAnd|atomicOr|atomicXor|atomicExchange|atomicCompSwap|atomicCounter|atomicCounterIncrement|atomicCounterDecrement)$"))

((call_expression function: (identifier) @function.builtin)
 (#match? @function.builtin "^(noise1|noise2|noise3|noise4)$"))

; discard - GLSL control keyword, matched via identifier (not a grammar node type)
((identifier) @keyword.control
 (#eq? @keyword.control "discard"))
