# intel-mkl-src

|crate         | crate.io                                                                                               | docs.rs                                                                               | master                                                                                                                                    | description                                                           |
|:-------------|:-------------------------------------------------------------------------------------------------------|:--------------------------------------------------------------------------------------|:------------------------------------------------------------------------------------------------------------------------------------------|:----------------------------------------------------------------------|
|intel-mkl-src | [![crate](https://img.shields.io/crates/v/intel-mkl-src.svg)](https://crates.io/crates/intel-mkl-src)  | [![docs.rs](https://docs.rs/intel-mkl-src/badge.svg)](https://docs.rs/intel-mkl-src)  | [![crate](https://img.shields.io/badge/master-intel--mkl--src-blue)](https://rust-math.github.io/intel-mkl-src/intel_mkl_src/index.html)  | Source crate for Intel-MKL                                            |
|intel-mkl-sys | [![Crate](https://img.shields.io/crates/v/intel-mkl-sys.svg)](https://crates.io/crates/intel-mkl-sys)  | [![docs.rs](https://docs.rs/intel-mkl-sys/badge.svg)](https://docs.rs/intel-mkl-sys)  | [![Crate](https://img.shields.io/badge/master-intel--mkl--sys-blue)](https://rust-math.github.io/intel-mkl-src/intel_mkl_sys/index.html)  | FFI for Intel-MKL [vector math][VM], and [statistical functions][VSL] |
|intel-mkl-tool| [![Crate](https://img.shields.io/crates/v/intel-mkl-tool.svg)](https://crates.io/crates/intel-mkl-tool)| [![docs.rs](https://docs.rs/intel-mkl-tool/badge.svg)](https://docs.rs/intel-mkl-tool)| [![Crate](https://img.shields.io/badge/master-intel--mkl--tool-blue)](https://rust-math.github.io/intel-mkl-src/intel_mkl_tool/index.html)| Seek Intel-MKL libraries from filesystem                              |

[VM]:  https://software.intel.com/en-us/mkl-developer-reference-c-vector-mathematical-functions
[VSL]: https://software.intel.com/en-us/mkl-developer-reference-c-statistical-functions

## Usage

`intel-mkl-src` crate is a `*-src` crate. This links MKL libraries to executable build by cargo, but does not provide Rust bindings.
Please use `blas-sys`, `lapack-sys`, or `fftw-sys` to use BLAS, LAPACK, FFTW interface of MKL, e.g.

```toml
[dependencies]
fftw-sys = { version = "0.4", features = ["intel-mkl"] }
```

Binding to MKL specific features are provided by `intel-mkl-sys` crate. This contains 

- [Vector Mathematical Functions](https://www.intel.com/content/www/us/en/develop/documentation/onemkl-developer-reference-c/top/vector-mathematical-functions.html)
- [Statistical Functions](https://www.intel.com/content/www/us/en/develop/documentation/onemkl-developer-reference-c/top/statistical-functions.html)

## How to find system MKL libraries

`intel-mkl-tool` crate seeks system MKL library installed by various installer as following manner:

- Seek using `pkg-config` command
- Seek `${MKLROOT}` directory
- Seek default installation path
  - `/opt/intel/mkl` for Linux
  - `C:/Program Files (x86)/IntelSWTools/` and `C:/Program Files (x86)/Intel/oneAPI` for Windows

If `intel-mkl-tool` does not find MKL library, `intel-mkl-src` try to download MKL binaries from [GitHub Container Registry (ghcr.io)](https://github.com/orgs/rust-math/packages?repo_name=rust-mkl-container).

## Supported features

There are 8 (=2x2x2) `mkl-*-*-*` features to specify how to link MKL libraries.
If any feature is set, default to `mkl-static-ilp64-iomp`.

### Link type (`static` or `dynamic`)
`dynamic` means MKL is linked dynamically, i.e. the executable does not contains MKL libraries
and will seek them from filesystem while execution.
This is better choice when the MKL libraries are managed by the system package manager e.g. `apt`.

`static` means MKL is linked statically, i.e. the MKL binaries are embedded in the executable file.
This is better choice when creating portable executable.

### Data model (`lp64` or `ilp64`)

This specify the data model:

- `ilp64` means `int` (i), `long` (l), and pointers (p) are 64-bit.
- `lp64` means `long` (l) and pointers (p) are 64-bit, `int` is 32-bit.

### Thread management (`iomp` or `seq`)

- `iomp` means MKL uses Intel OpenMP runtime
- `seq` means sequential (single thread) execution

Using GNU OpenMP runtime (`libgomp`) is not supported yet. Please see https://github.com/rust-math/intel-mkl-src/issues/97

## Installing Intel MKL on Windows

Install only the MKL component without the full oneAPI toolkit using one of:

**winget (recommended):**
```
winget install Intel.oneMKL
```

**NuGet (for CI or reproducible builds):**
```
nuget install intelmkl.devel.win-x64 -Version 2025.3.0
nuget install intelmkl.static.win-x64 -Version 2025.3.0
```
Then set `MKLROOT=<path to the nuget package directory>`.

**Intel online installer:**
Download from https://www.intel.com/content/www/us/en/developer/tools/oneapi/onemkl.html, select the standalone MKL component only.

If MKL is installed but not detected automatically, set one of:
```
MKLROOT=C:\Program Files (x86)\Intel\oneAPI\mkl\latest
ONEAPI_ROOT=C:\Program Files (x86)\Intel\oneAPI
```

## Deploying on Windows

This crate only tells the Rust linker where to find MKL at compile time. It does **not** copy DLLs to your output directory. You must ship the required DLLs alongside your `.exe` manually.

### Required DLLs

For configurations using `iomp` (Intel OpenMP), you must ship:
- `libiomp5md.dll` — Intel OpenMP runtime

For configurations using `dynamic` link type, you must also ship the MKL runtime DLLs (e.g. `mkl_rt.2.dll`).

These are found in your MKL installation under:
```
C:\Program Files (x86)\Intel\oneAPI\mkl\latest\redist\intel64\
C:\Program Files (x86)\Intel\oneAPI\compiler\latest\windows\redist\intel64_win\compiler\
```

### Simplest deployment: avoid runtime DLL dependencies

Use `mkl-static-ilp64-seq` (sequential, no OpenMP). This statically links all MKL code and requires no MKL or OpenMP DLLs at runtime.

Note: even with `static` + `iomp`, `libiomp5md.dll` cannot be statically linked on Windows with MSVC and will still be required at runtime.

## Python Bindings on Windows

When this crate is used as a dependency of a Rust-based Python extension (e.g. via [PyO3](https://pyo3.rs) and [maturin](https://maturin.rs)), the compiled output is a `.pyd` file rather than an `.exe`. DLL resolution rules still apply, but the search context changes: Windows looks for DLL dependencies relative to the `.pyd` file's directory, not the Python executable.

### Development

`maturin develop` installs the `.pyd` into your active virtual environment's `site-packages`. `libiomp5md.dll` will not be present there, so importing the module will fail unless the MKL redist directory is on your `PATH`:

```
C:\Program Files (x86)\Intel\oneAPI\compiler\latest\windows\redist\intel64_win\compiler
```

Adding this to your system or environment `PATH` once after installing MKL is the practical solution for development.

### Deployment (wheels)

Use [`delvewheel`](https://github.com/adang1345/delvewheel) to produce self-contained wheels. It scans the `.pyd` for DLL dependencies, copies them into the wheel under a `.libs\` subdirectory, and patches the loader so users do not need MKL installed.

```powershell
pip install delvewheel
maturin build --release
delvewheel repair target\wheels\*.whl --wheel-dir dist\
```

Upload the repaired wheel from `dist\` to PyPI or your artifact store. The wheel is fully self-contained.

### DLL resolution summary

| Scenario | Where libiomp5md.dll must be |
|---|---|
| `maturin develop` | On `PATH` |
| Running `.pyd` directly | On `PATH` or in the `.pyd`'s directory |
| Wheel distribution | Bundled automatically by `delvewheel repair` |

## License
MKL is distributed under the Intel Simplified Software License for Intel(R) Math Kernel Library, See [License.txt](License.txt).
Some wrapper codes are licensed by MIT License (see the header of each file).
