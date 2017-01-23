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

// Notes:
//
// 1. Configure GMP with --enable-fat so that built file is portable.
//
// 2. Configure GMP, MPFR and MPC with: --disable-shared --with-pic
//
// 3. Add symlinks to work around relative path issues in MPFR and MPC.
//    In MPFR: ln -s ../gmp-build
//    In MPC: ln -s ../mpfr-src ../mpfr-build ../gmp-build .
//
// 4. Use relative paths for configure otherwise mingw might be
//    confused with drives and such.

use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const GMP_DIR: &'static str = "gmp-6.1.2-slim";
const MPFR_DIR: &'static str = "mpfr-3.1.5-slim";
const MPC_DIR: &'static str = "mpc-1.0.3-slim";

fn main() {
    let src_dir = PathBuf::from(cargo_env("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(cargo_env("OUT_DIR"));
    let jobs = cargo_env("NUM_JOBS");
    let profile = cargo_env("PROFILE");
    let check = profile == OsString::from("release");

    let lib_dir = out_dir.join("lib");
    let gmp_lib = lib_dir.join("libgmp.a");
    let mpfr_lib = lib_dir.join("libmpfr.a");
    let mpc_lib = lib_dir.join("libmpc.a");
    if !gmp_lib.is_file() || !mpfr_lib.is_file() || !mpc_lib.is_file() {
        create_dir(&lib_dir);
        let build_dir = out_dir.join("build");
        remove_dir(&build_dir);
        create_dir(&build_dir);
        build_gmp(&build_dir, &src_dir, &jobs, check, &gmp_lib);
        build_mpfr(&build_dir, &src_dir, &jobs, check, &mpfr_lib);
        build_mpc(&build_dir, &src_dir, &jobs, check, &mpc_lib);
        remove_dir(&build_dir);
    }
    write_cargo_info(&lib_dir);
}

fn build_gmp(top_build_dir: &Path,
             src_dir: &Path,
             jobs: &OsStr,
             check: bool,
             lib: &Path) {
    let build_dir = top_build_dir.join("gmp-build");
    create_dir(&build_dir);
    println!("$ cd \"{}\"", build_dir.display());
    let mut conf = dir_sane(&src_dir.join(GMP_DIR));
    conf.push("configure --enable-fat --disable-shared --with-pic");
    configure(&build_dir, &conf);
    make_and_check(&build_dir, &jobs, check);
    let build_lib = build_dir.join(".libs").join("libgmp.a");
    copy_file(&build_lib, &lib);
}

fn build_mpfr(top_build_dir: &Path,
              src_dir: &Path,
              jobs: &OsStr,
              check: bool,
              lib: &Path) {
    let build_dir = top_build_dir.join("mpfr-build");
    create_dir(&build_dir);
    println!("$ cd {}", build_dir.display());
    let mut conf = dir_sane(&src_dir.join(MPFR_DIR));
    conf.push("configure --enable-thread-safe --disable-shared --with-pic \
               --with-gmp-build=");
    conf.push(dir_sane(&top_build_dir.join("gmp-build")));
    configure(&build_dir, &conf);
    make_and_check(&build_dir, &jobs, check);
    let build_lib = build_dir.join("src").join(".libs").join("libmpfr.a");
    copy_file(&build_lib, &lib);
}

fn build_mpc(top_build_dir: &Path,
             src_dir: &Path,
             jobs: &OsStr,
             check: bool,
             lib: &Path) {
    let build_dir = top_build_dir.join("mpc-build");
    create_dir(&build_dir);
    println!("$ cd {}", build_dir.display());
    let mut conf = dir_sane(&src_dir.join(MPC_DIR));
    conf.push("configure --disable-shared --with-pic --with-mpfr-include=");
    conf.push(dir_sane(&src_dir.join(MPFR_DIR).join("src")));
    conf.push(" --with-mpfr-lib=");
    conf.push(dir_sane(&build_dir.join("mpfr-build")
        .join("src")
        .join(".libs")));
    conf.push(" --with-gmp-include=");
    conf.push(dir_sane(&build_dir.join("gmp-build")));
    conf.push(" --with-gmp-lib=");
    conf.push(dir_sane(&build_dir.join("gmp-build").join(".libs")));
    configure(&build_dir, &OsString::from(conf));
    make_and_check(&build_dir, &jobs, check);
    let build_lib = build_dir.join("src").join(".libs").join("libmpc.a");
    copy_file(&build_lib, &lib);
}

fn write_cargo_info(lib_dir: &Path) {
    let lib_search = lib_dir.to_str().unwrap_or_else(|| {
        panic!("Path contains unsupported characters, can only make {}",
               lib_dir.display())
    });
    println!("cargo:rustc-link-search=native={}", lib_search);
    println!("cargo:rustc-link-lib=static=gmp");
    println!("cargo:rustc-link-lib=static=mpfr");
    println!("cargo:rustc-link-lib=static=mpc");
}

fn cargo_env(name: &str) -> OsString {
    env::var_os(name).unwrap_or_else(|| {
        panic!("environment variable not found: {}, please use cargo", name)
    })
}

fn remove_dir(dir: &Path) {
    if !dir.exists() {
        return;
    }
    assert!(dir.is_dir(), "Not a directory: {}", dir.display());
    fs::remove_dir_all(dir).unwrap_or_else(|_| {
        panic!("Unable to remove directory: {}", dir.display())
    });
}

fn create_dir(dir: &Path) {
    fs::create_dir_all(dir).unwrap_or_else(|_| {
        panic!("Unable to create directory: {}", dir.display())
    });
}

fn dir_sane(dir: &Path) -> OsString {
    let s = dir.to_str().unwrap_or_else(|| {
        panic!("Path contains unsupported characters, can only make {}",
               dir.display())
    });
    if !cfg!(windows) {
        let mut r = OsString::from(s);
        if !s.ends_with('/') {
            r.push("/");
        }
        return r;
    }
    let mut result = String::new();
    let mut chars = s.chars();
    let first = chars.next();
    let second = chars.next();
    let third = chars.next();
    if second == Some(':') {
        result.push('/');
        result.push(first.unwrap());
        result.push('/');
        match third {
            None | Some('/') | Some('\\') => {}
            Some(c) => result.push(c),
        }
    } else {
        chars = s.chars();
    }
    for c in chars {
        match c {
            '\\' => result.push('/'),
            _ => result.push(c),
        }
    }
    if !result.ends_with('/') {
        result.push('/');
    }
    OsString::from(result)
}

fn configure(build_dir: &Path, conf_line: &OsStr) {
    let mut conf = Command::new("sh");
    conf.current_dir(&build_dir).arg("-c").arg(conf_line);
    execute(conf);
}

fn make_and_check(build_dir: &Path, jobs: &OsStr, check: bool) {
    let mut make = Command::new("make");
    make.current_dir(build_dir).arg("-j").arg(jobs);
    execute(make);
    if check {
        let mut make_check = Command::new("make");
        make_check.current_dir(build_dir).arg("-j").arg(jobs).arg("check");
        execute(make_check);
    }
}

fn copy_file(src: &Path, dst: &Path) {
    fs::copy(&src, &dst).unwrap_or_else(|_| {
        panic!("Unable to copy {} -> {}", src.display(), dst.display());
    });
}

fn execute(mut command: Command) {
    println!("$ {:?}", command);
    let status = command.status()
        .unwrap_or_else(|_| panic!("Unable to execute: {:?}", command));
    if !status.success() {
        if let Some(code) = status.code() {
            panic!("Program failed with code {}: {:?}", code, command);
        } else {
            panic!("Program failed: {:?}", command);
        }
    }
}
