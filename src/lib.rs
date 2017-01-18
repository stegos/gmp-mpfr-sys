// Copyright © 2017 University of Malta

// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License and a copy of the GNU General Public License along with
// this program. If not, see <http://www.gnu.org/licenses/>.

// Generate gmp.rs:
//     bindgen --link static=gmp path/to/gmp.h --output=gmp.rs
pub mod gmp;

// Generate mpfr.rs:
//     bindgen --link static=mpfr path/to/mpfr.h --output=mpfr.rs --match=mpfr.h
// Then, after #![] block, insert:
//     use ::gmp::*;
pub mod mpfr;

// Generate mpc.rs:
//     bindgen --link static=mpc path/to/mpfr.h --output=mpc.rs --match=mpc.h
// Then, after #![] block, insert:
//     use ::gmp::*;
//     use ::mpfr::*;
pub mod mpc;
