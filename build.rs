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
// 4. Use relative paths for configure otherwise msys/mingw might be
//    confused with drives and such.

use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
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
    let gmp_header = lib_dir.join("gmp.h");
    let mpfr_lib = lib_dir.join("libmpfr.a");
    let mpfr_header = lib_dir.join("mpfr.h");
    let mpc_lib = lib_dir.join("libmpc.a");
    let mpc_header = lib_dir.join("mpc.h");
    if !gmp_lib.is_file() || !gmp_header.is_file() || !mpfr_lib.is_file() ||
       !mpfr_header.is_file() ||
       !mpc_lib.is_file() || !mpc_header.is_file() {
        create_dir(&lib_dir);
        let build_dir = out_dir.join("build");
        remove_dir(&build_dir);
        create_dir(&build_dir);
        symlink(&build_dir,
                &dir_relative(&build_dir, &src_dir.join(GMP_DIR)),
                Some(&OsString::from("gmp-src")));
        build_gmp(&build_dir, &jobs, check, &gmp_lib, &gmp_header);
        symlink(&build_dir,
                &dir_relative(&build_dir, &src_dir.join(MPFR_DIR)),
                Some(&OsString::from("mpfr-src")));
        build_mpfr(&build_dir, &jobs, check, &mpfr_lib, &mpfr_header);
        symlink(&build_dir,
                &dir_relative(&build_dir, &src_dir.join(MPC_DIR)),
                Some(&OsString::from("mpc-src")));
        build_mpc(&build_dir, &jobs, check, &mpc_lib, &mpc_header);
        remove_dir(&build_dir);
    }
    process_gmp_header(&gmp_header, &out_dir.join("gmp_h.rs"));
    write_cargo_info(&lib_dir);
}

fn build_gmp(top_build_dir: &Path,
             jobs: &OsStr,
             check: bool,
             lib: &Path,
             header: &Path) {
    let build_dir = top_build_dir.join("gmp-build");
    create_dir(&build_dir);
    println!("$ cd \"{}\"", build_dir.display());
    let conf = "../gmp-src/configure --enable-fat --disable-shared --with-pic";
    configure(&build_dir, &OsString::from(conf));
    make_and_check(&build_dir, &jobs, check);
    let build_lib = build_dir.join(".libs").join("libgmp.a");
    copy_file(&build_lib, &lib);
    let build_header = build_dir.join("gmp.h");
    copy_file(&build_header, &header);
}

fn process_gmp_header(header: &Path, out_file: &Path) {
    use std::fmt::Write;

    let mut limb_bits = None;
    let mut nail_bits = None;
    let mut long_long_limb = None;
    let mut cc = None;
    let mut cflags = None;
    let mut reader = open(&header);
    let mut buf = String::new();
    while read_line(&mut reader, &mut buf, &header) > 0 {
        if buf.contains("#undef _LONG_LONG_LIMB") {
            long_long_limb = Some(false);
        }
        if buf.contains("#define _LONG_LONG_LIMB 1") {
            long_long_limb = Some(true);
        }
        let s = "#define GMP_LIMB_BITS";
        if let Some(start) = buf.find(s) {
            limb_bits = buf[(start + s.len())..].trim().parse::<i32>().ok();
        }
        let s = "#define GMP_NAIL_BITS";
        if let Some(start) = buf.find(s) {
            nail_bits = buf[(start + s.len())..].trim().parse::<i32>().ok();
        }
        let s = "#define __GMP_CC";
        if let Some(start) = buf.find(s) {
            cc = Some(buf[(start + s.len())..]
                          .trim()
                          .trim_matches('"')
                          .to_string());
        }
        let s = "#define __GMP_CFLAGS";
        if let Some(start) = buf.find(s) {
            cflags = Some(buf[(start + s.len())..]
                              .trim()
                              .trim_matches('"')
                              .to_string());
        }
        buf.clear();
    }
    drop(reader);
    let limb_bits = limb_bits.unwrap_or_else(|| {
        panic!("Cannot determine GMP_LIMB_BITS from {}", header.display())
    });
    let nail_bits = nail_bits.unwrap_or_else(|| {
        panic!("Cannot determine GMP_NAIL_BITS from {}", header.display())
    });
    let long_long_limb = long_long_limb.unwrap_or_else(|| {
        panic!("Cannot determine _LONG_LONG_LIMB from {}", header.display())
    });
    let cc = cc.unwrap_or_else(|| {
                                   panic!("Cannot determine __GMP_CC from {}",
                                          header.display())
                               });
    let cflags = cflags.unwrap_or_else(|| {
        panic!("Cannot determine __GMP_CFLAGS from {}", header.display())
    });
    let mut content = String::new();
    match limb_bits {
        32 => {
            println!("cargo:rustc-cfg=gmp_limb_bits_32");
        }
        64 => {
            println!("cargo:rustc-cfg=gmp_limb_bits_64");
        }
        n => {
            let _ = write!(content, "const GMP_LIMB_BITS: c_int = {};\n", n);
        }
    }
    match nail_bits {
        0 => {
            println!("cargo:rustc-cfg=gmp_nail_bits_0");
        }
        n => {
            let _ = write!(content, "const GMP_NAIL_BITS: c_int = {};\n", n);
        }
    }
    if long_long_limb {
        println!("cargo:rustc-cfg=gmp_long_long_limb");
    }
    let _ = write!(content,
                   "const GMP_CC: *const c_char = b\"{}\\0\" as *const u8 as \
                    *const c_char;\n",
                   cc);
    let _ = write!(content,
                   "const GMP_CFLAGS: *const c_char = b\"{}\\0\" as *const \
                    u8 as *const c_char;\n",
                   cflags);
    let mut rs = create(out_file);
    write(&mut rs, &content, out_file);
    flush(&mut rs, out_file);
}

fn build_mpfr(top_build_dir: &Path,
              jobs: &OsStr,
              check: bool,
              lib: &Path,
              header: &Path) {
    let build_dir = top_build_dir.join("mpfr-build");
    create_dir(&build_dir);
    println!("$ cd {}", build_dir.display());
    symlink(&build_dir, &OsString::from("../gmp-build"), None);
    let conf = "../mpfr-src/configure --enable-thread-safe --disable-shared \
                --with-gmp-build=../gmp-build --with-pic";
    configure(&build_dir, &OsString::from(conf));
    make_and_check(&build_dir, &jobs, check);
    let build_lib = build_dir.join("src").join(".libs").join("libmpfr.a");
    copy_file(&build_lib, &lib);
    let src_header = top_build_dir
        .join("mpfr-src")
        .join("src")
        .join("mpfr.h");
    copy_file(&src_header, &header);
}

fn build_mpc(top_build_dir: &Path,
             jobs: &OsStr,
             check: bool,
             lib: &Path,
             header: &Path) {
    let build_dir = top_build_dir.join("mpc-build");
    create_dir(&build_dir);
    println!("$ cd {}", build_dir.display());
    symlink(&build_dir, &OsString::from("../mpfr-src"), None);
    symlink(&build_dir, &OsString::from("../mpfr-build"), None);
    symlink(&build_dir, &OsString::from("../gmp-build"), None);
    let conf = "../mpc-src/configure --disable-shared \
                --with-mpfr-include=../mpfr-src/src \
                --with-mpfr-lib=../mpfr-build/src/.libs \
                --with-gmp-include=../gmp-build \
                --with-gmp-lib=../gmp-build/.libs --with-pic";
    configure(&build_dir, &OsString::from(conf));
    make_and_check(&build_dir, &jobs, check);
    let build_lib = build_dir.join("src").join(".libs").join("libmpc.a");
    copy_file(&build_lib, &lib);
    let src_header = top_build_dir.join("mpc-src").join("src").join("mpc.h");
    copy_file(&src_header, &header);
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

fn dir_relative(dir: &Path, rel_to: &Path) -> OsString {
    let (mut diri, mut reli) = (dir.components(), rel_to.components());
    let (mut dirc, mut relc) = (diri.next(), reli.next());
    let mut some_common = false;
    while let (Some(d), Some(r)) = (dirc, relc) {
        if d != r {
            break;
        }
        some_common = true;
        dirc = diri.next();
        relc = reli.next();
    }
    assert!(some_common,
            "cannot access {} from {} using relative paths",
            rel_to.display(),
            dir.display());
    let mut ret = OsString::new();
    while dirc.is_some() {
        if !ret.is_empty() {
            ret.push("/");
        }
        ret.push("..");
        dirc = diri.next();
    }
    while let Some(r) = relc {
        if !ret.is_empty() {
            ret.push("/");
        }
        ret.push(r);
        relc = reli.next();
    }
    if ret.is_empty() {
        ret.push(".");
    }
    ret
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
        make_check
            .current_dir(build_dir)
            .arg("-j")
            .arg(jobs)
            .arg("check");
        execute(make_check);
    }
}

fn copy_file(src: &Path, dst: &Path) {
    fs::copy(&src, &dst).unwrap_or_else(|_| {
                                            panic!("Unable to copy {} -> {}",
                                                   src.display(),
                                                   dst.display());
                                        });
}

fn symlink(dir: &Path, link: &OsStr, name: Option<&OsStr>) {
    let mut c = Command::new("ln");
    c.current_dir(dir).arg("-s").arg(link);
    if let Some(name) = name {
        c.arg(name);
    }
    execute(c);
}

fn execute(mut command: Command) {
    println!("$ {:?}", command);
    let status =
        command
            .status()
            .unwrap_or_else(|_| panic!("Unable to execute: {:?}", command));
    if !status.success() {
        if let Some(code) = status.code() {
            panic!("Program failed with code {}: {:?}", code, command);
        } else {
            panic!("Program failed: {:?}", command);
        }
    }
}

fn open(name: &Path) -> BufReader<File> {
    let file =
        File::open(name).unwrap_or_else(|_| {
                                            panic!("Cannot open file: {}",
                                                   name.display())
                                        });
    BufReader::new(file)
}

fn create(name: &Path) -> BufWriter<File> {
    let file =
        File::create(name).unwrap_or_else(|_| {
                                              panic!("Cannot create file: {}",
                                                     name.display())
                                          });
    BufWriter::new(file)
}

fn read_line(reader: &mut BufReader<File>,
             buf: &mut String,
             name: &Path)
             -> usize {
    reader
        .read_line(buf)
        .unwrap_or_else(|_| panic!("Cannot read from: {}", name.display()))
}

fn write(writer: &mut BufWriter<File>, buf: &str, name: &Path) {
    writer
        .write(buf.as_bytes())
        .unwrap_or_else(|_| panic!("Cannot write to: {}", name.display()));
}

fn flush(writer: &mut BufWriter<File>, name: &Path) {
    writer
        .flush()
        .unwrap_or_else(|_| panic!("Cannot write to: {}", name.display()));
}
