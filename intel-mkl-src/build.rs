// MIT License
//
// Copyright (c) 2017 Toshiki Teramura
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use anyhow::{bail, Result};
use intel_mkl_tool::*;
use std::str::FromStr;

macro_rules! def_mkl_config {
    ($cfg:literal) => {
        #[cfg(feature = $cfg)]
        const MKL_CONFIG: &str = $cfg;
    };
}

def_mkl_config!("mkl-static-lp64-iomp");
def_mkl_config!("mkl-static-lp64-seq");
def_mkl_config!("mkl-static-ilp64-iomp");
def_mkl_config!("mkl-static-ilp64-seq");
def_mkl_config!("mkl-dynamic-lp64-iomp");
def_mkl_config!("mkl-dynamic-lp64-seq");
def_mkl_config!("mkl-dynamic-ilp64-iomp");
def_mkl_config!("mkl-dynamic-ilp64-seq");

// Default value
#[cfg(all(
    not(feature = "mkl-static-lp64-iomp"),
    not(feature = "mkl-static-lp64-seq"),
    not(feature = "mkl-static-ilp64-iomp"),
    not(feature = "mkl-static-ilp64-seq"),
    not(feature = "mkl-dynamic-lp64-iomp"),
    not(feature = "mkl-dynamic-lp64-seq"),
    not(feature = "mkl-dynamic-ilp64-iomp"),
    not(feature = "mkl-dynamic-ilp64-seq"),
))]
const MKL_CONFIG: &str = "mkl-static-ilp64-iomp";

fn main() -> Result<()> {
    let cfg = Config::from_str(MKL_CONFIG).unwrap();
    match Library::new(cfg) {
        Ok(lib) => lib.print_cargo_metadata()?,
        Err(_) => {
            bail!(
                "Intel MKL ({cfg}) not found.\n\n\
                Please install Intel MKL and set the MKLROOT or ONEAPI_ROOT environment variable.\n\n\
                Installation options:\n\
                \n\
                  Windows (NuGet, recommended):\n\
                    nuget install intelmkl.devel.win-x64 -Version 2025.3.0\n\
                    nuget install intelmkl.static.win-x64 -Version 2025.3.0\n\
                    Then set: MKLROOT=<path to nuget package directory>\n\
                \n\
                  Linux (APT):\n\
                    # Add Intel repository: https://www.intel.com/content/www/us/en/developer/tools/oneapi/onemkl.html\n\
                    sudo apt install intel-oneapi-mkl-devel\n\
                \n\
                  Conda (all platforms):\n\
                    conda install -c intel mkl-devel\n\
                    Then set: MKLROOT=<conda env prefix>\n\
                \n\
                  oneAPI standalone installer:\n\
                    https://www.intel.com/content/www/us/en/developer/tools/oneapi/onemkl.html\n\
                \n\
                After installing, if MKL is not found automatically, set one of:\n\
                  MKLROOT=<path to MKL root>     (e.g. .../oneapi/mkl/2025.3)\n\
                  ONEAPI_ROOT=<path to oneAPI>   (e.g. C:/Program Files/Intel/oneAPI)\n"
            );
        }
    }
    Ok(())
}
