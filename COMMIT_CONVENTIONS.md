<img src= "https://github.com/thrustlang/.github/blob/main/assets/logos/thrustlang-logo-name.png" alt= "logo" style= "width: 80%; height: 80%;"></img>

# Compiler Dependency Builder

There is a simple guide of standard conventions to follow in order to delivery a good Github commit for the Compiler Dependency Builder (**compiler-dependency-builder**).

### Title

It needs to be detailed. It can be include a lot of technical slang. The base of a well designed Github commit title always will be and needs a specific syntax as:

#### Title - features

Following the syntax:

`feat(...)`

Valid locations:

- `project-visual` Any location that usually involucrates the visual representation or human guide for the builder available on Github (Example: README.md).
- `project` Any location that usually involucrates Cargo, Rust Compiler and Github repository changes or the conception of a new part of the build or the builder itself.

Example:

`feat(project)` Adding more information regarding LLVM.

#### Title - fixes

Following the syntax:

`fix(...)`

Valid locations:

- `project-visual` Any location that usually involucrates the visual representation or human guide for the builder available on Github (Example: README.md).
- `project` Any location that usually involucrates Cargo, Rust Compiler and Github repository changes or the conception of a new part of the build or the builder itself.

Any consecutive location written to the next one needs to be follow for a COMMA character `,`.

Example:

`fix(project)` Fixing several issues on `llvm.rs`.

#### Title - Combinatory

In order to create a well disigned combinatory title, you need to use the following syntax:

`(feat(...), fix(...))`

- It needs to be encapsulated for a pair characters PAREN `()`.
- Each next feature or fix needs to be followed for a COMMA character `,`. 

### Description

It needs to be concise, short, but detailed in the same time. It can be include a lot of technical slang.







