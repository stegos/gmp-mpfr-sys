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
// 1. Add the following line to the top of mpfr-3.1.5/configure, to
//    work around lacking realpath in macOS:
//
//    # Define realpath if it is not found (for macOS)
//    realpath . >/dev/null 2>&1 || realpath() { case "$1" in
//      [\\/]* | ?:[\\/]* ) echo "$1" ;;
//      * ) echo "$PWD/$1" ;;
//    esac; }
//
// 2. Configure GMP with --enable-fat so that built file is portable.
//
// 3. Configure GMP and MPFR with: --disable-shared --with-pic
//
// 4. Use relative paths for configure otherwise mingw might be
//    confused with drives and such.

use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const GMP_DIR: &'static str = "gmp-6.1.2";
const MPFR_DIR: &'static str = "mpfr-3.1.5";
const MPC_DIR: &'static str = "mpc-1.0.3";

fn main() {
    let src_dir = PathBuf::from(cargo_env("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(cargo_env("OUT_DIR"));
    let jobs = cargo_env("NUM_JOBS");

    let lib_dir = out_dir.join("lib");
    create_dir(&lib_dir);
    let build_dir = out_dir.join("build");
    create_dir(&build_dir);
    symlink(&build_dir,
            &dir_relative(&build_dir, &src_dir.join(GMP_DIR)),
            Some(&OsString::from("gmp-src")));
    symlink(&build_dir,
            &dir_relative(&build_dir, &src_dir.join(MPFR_DIR)),
            Some(&OsString::from("mpfr-src")));
    symlink(&build_dir,
            &dir_relative(&build_dir, &src_dir.join(MPC_DIR)),
            Some(&OsString::from("mpc-src")));

    let gmp_lib = lib_dir.join("libgmp.a");
    if !gmp_lib.is_file() {
        let gmp_build_dir = build_dir.join("gmp-build");
        remove_dir(&gmp_build_dir);
        create_dir(&gmp_build_dir);
        let conf = "../gmp-src/configure --enable-fat --disable-shared \
                    --with-pic";
        println!("Running configure in {}", gmp_build_dir.display());
        configure(&gmp_build_dir, &OsString::from(conf));
        remove_from_makefile(&gmp_build_dir, &["doc", "demos"]);
        make_and_check(&gmp_build_dir, &jobs);
        let gmp_build_lib = gmp_build_dir.join(".libs").join("libgmp.a");
        copy_file(&gmp_build_lib, &gmp_lib);
    }

    let mpfr_lib = lib_dir.join("libmpfr.a");
    if !mpfr_lib.is_file() {
        let mpfr_build_dir = build_dir.join("mpfr-build");
        remove_dir(&mpfr_build_dir);
        create_dir(&mpfr_build_dir);
        symlink(&mpfr_build_dir, &OsString::from("../gmp-build"), None);
        // touch these files so that we don't try to rebuild them
        for f in &["aclocal.m4", "configure", "Makefile.am", "Makefile.in"] {
            touch(&mpfr_build_dir.join("../mpfr-src"), &OsString::from(f));
        }
        let conf = "../mpfr-src/configure --enable-thread-safe \
                    --disable-shared --with-gmp-build=../gmp-build --with-pic";
        println!("Running configure in {}", mpfr_build_dir.display());
        configure(&mpfr_build_dir, &OsString::from(conf));
        remove_from_makefile(&mpfr_build_dir, &["doc"]);
        make_and_check(&mpfr_build_dir, &jobs);
        let mpfr_build_lib =
            mpfr_build_dir.join("src").join(".libs").join("libmpfr.a");
        copy_file(&mpfr_build_lib, &mpfr_lib);
    }

    let mpc_lib = lib_dir.join("libmpc.a");
    if !mpc_lib.is_file() {
        let mpc_build_dir = build_dir.join("mpc-build");
        remove_dir(&mpc_build_dir);
        create_dir(&mpc_build_dir);
        symlink(&mpc_build_dir, &OsString::from("../mpfr-src"), None);
        symlink(&mpc_build_dir, &OsString::from("../mpfr-build"), None);
        symlink(&mpc_build_dir, &OsString::from("../gmp-build"), None);
        let conf = "../mpc-src/configure --disable-shared \
                    --with-mpfr-include=../mpfr-src/src \
                    --with-mpfr-lib=../mpfr-build/src/.libs \
                    --with-gmp-include=../gmp-build \
                    --with-gmp-lib=../gmp-build --with-pic";
        println!("Running configure in {}", mpc_build_dir.display());
        configure(&mpc_build_dir, &OsString::from(conf));
        remove_from_makefile(&mpc_build_dir, &["doc"]);
        make_and_check(&mpc_build_dir, &jobs);
        let mpc_build_lib =
            mpc_build_dir.join("src").join(".libs").join("libmpc.a");
        copy_file(&mpc_build_lib, &mpc_lib);
    }

    // remove_dir(&build_dir);

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

fn remove_from_makefile(build_dir: &Path, dirs: &[&str]) {
    let makefile = build_dir.join("Makefile");
    let work = build_dir.join("Makefile.work");
    let mut reader = open(&makefile);
    let mut writer = create(&work);
    let mut buf = String::new();
    while read_line(&mut reader, &mut buf, &makefile) > 0 {
        if buf.starts_with("SUBDIRS = ") {
            for dir in dirs {
                let mut space = String::from(" ") + dir + " ";
                if let Some(i) = buf.find(&space) {
                    buf.drain(i..i + dir.len() + 1);
                } else {
                    space.pop();
                    space += "\n";
                    if let Some(i) = buf.find(&space) {
                        buf.drain(i..i + dir.len() + 1);
                    }
                }
            }
            if let Some(doc) = buf.find(" doc ") {
                buf.drain(doc..doc + 4);
            } else if let Some(doc) = buf.find(" doc\n") {
                buf.drain(doc..doc + 4);
            }
        }
        write(&mut writer, &buf, &work);
        buf.clear();
    }
    drop(reader);
    // check for write errors
    flush(&mut writer, &work);
    drop(writer);
    rename(&work, &makefile);
}

fn make_and_check(build_dir: &Path, jobs: &OsStr) {
    let mut make = Command::new("make");
    make.current_dir(build_dir).arg("-j").arg(jobs);
    execute(make);
    let mut make_check = Command::new("make");
    make_check.current_dir(build_dir).arg("-j").arg(jobs).arg("check");
    execute(make_check);
}

fn copy_file(src: &Path, dst: &Path) {
    fs::copy(&src, &dst).unwrap_or_else(|_| {
        panic!("Unable to copy {} -> {}", src.display(), dst.display());
    });
}

fn touch(dir: &Path, file: &OsStr) {
    let mut c = Command::new("touch");
    c.current_dir(dir).arg(file);
    execute(c);
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

fn open(name: &Path) -> BufReader<File> {
    let file = File::open(name)
        .unwrap_or_else(|_| panic!("Cannot open file: {}", name.display()));
    BufReader::new(file)
}

fn create(name: &Path) -> BufWriter<File> {
    let file = File::create(name)
        .unwrap_or_else(|_| panic!("Cannot create file: {}", name.display()));
    BufWriter::new(file)
}

fn read_line(reader: &mut BufReader<File>,
             buf: &mut String,
             name: &Path)
             -> usize {
    reader.read_line(buf)
        .unwrap_or_else(|_| panic!("Cannot read from: {}", name.display()))
}

fn write(writer: &mut BufWriter<File>, buf: &str, name: &Path) {
    writer.write(buf.as_bytes())
        .unwrap_or_else(|_| panic!("Cannot write to: {}", name.display()));
}

fn flush(writer: &mut BufWriter<File>, name: &Path) {
    writer.flush()
        .unwrap_or_else(|_| panic!("Cannot write to: {}", name.display()));
}

fn rename(src: &Path, dst: &Path) {
    fs::rename(&src, &dst).unwrap_or_else(|_| {
        panic!("Unable to rename {} -> {}", src.display(), dst.display());
    });
}
