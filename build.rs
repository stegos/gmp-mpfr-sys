// Copyright © 2017 University of Malta

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const GMP_DIR: &'static str = "gmp-6.1.2";
const MPFR_DIR: &'static str = "mpfr-3.1.5";

fn main() {
    let src_dir = PathBuf::from(cargo_env("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(cargo_env("OUT_DIR"));
    let jobs = cargo_env("NUM_JOBS");

    let lib_dir = out_dir.join("lib");
    create_dir(&lib_dir);
    let gmp_lib = lib_dir.join("libgmp.a");
    if !gmp_lib.is_file() {
        let gmp_build_dir = out_dir.join("build-gmp");
        remove_dir(&gmp_build_dir);
        create_dir(&gmp_build_dir);
        let gmp_src_dir = src_dir.join(GMP_DIR);
        let mut conf = dir_relative(&gmp_build_dir, &gmp_src_dir);
        conf.push("/configure --enable-shared=no --with-pic=yes");
        configure(&conf, &gmp_build_dir);
        remove_from_makefile(&gmp_build_dir, &["doc", "demos"]);
        make_and_check(&gmp_build_dir, &jobs);
        let gmp_build_lib = gmp_build_dir.join(".libs").join("libgmp.a");
        copy_file(&gmp_build_lib, &gmp_lib);
    }
    let mpfr_lib = lib_dir.join("libmpfr.a");
    if !mpfr_lib.is_file() {
        let mpfr_build_dir = out_dir.join("build-mpfr");
        remove_dir(&mpfr_build_dir);
        create_dir(&mpfr_build_dir);
        // touch these files so that we don't try to rebuild them
        let mpfr_src_dir = src_dir.join(MPFR_DIR);
        for f in &["aclocal.m4", "configure", "Makefile.am", "Makefile.in"] {
            touch(&mpfr_src_dir.join(f));
        }
        let mut conf = dir_relative(&mpfr_build_dir, &mpfr_src_dir);
        conf.push("/configure --enable-shared=no --with-pic=yes \
                   --with-gmp-build=../build-gmp");
        configure(&conf, &mpfr_build_dir);
        remove_from_makefile(&mpfr_build_dir, &["doc"]);
        make_and_check(&mpfr_build_dir, &jobs);
        let mpfr_build_lib =
            mpfr_build_dir.join("src").join(".libs").join("libmpfr.a");
        copy_file(&mpfr_build_lib, &mpfr_lib);
    }

    let lib_search = lib_dir.to_str().unwrap_or_else(|| {
        panic!("Path contains unsupported characters, can only make {}",
               lib_dir.display())
    });
    println!("cargo:rustc-link-search=native={}", lib_search);
    println!("cargo:rustc-link-lib=static=gmp");
    println!("cargo:rustc-link-lib=static=mpfr");
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
    if !dir.is_dir() {
        panic!("Not a directory: {}", dir.display());
    }
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
    if !some_common {
        panic!("cannot access {} from {} using relative paths",
               rel_to.display(),
               dir.display());
    }
    let mut ret = OsString::from("");
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

fn configure(conf_line: &OsString, build_dir: &PathBuf) {
    let mut conf = Command::new("sh");
    conf.current_dir(&build_dir).arg("-c").arg(conf_line);
    execute(conf);
}

fn remove_from_makefile(build_dir: &PathBuf, dirs: &[&str]) {
    let makefile = build_dir.join("Makefile");
    let work = build_dir.join("Makefile.work");
    let read_file = open(&makefile);
    let mut reader = io::BufReader::new(read_file);
    let write_file = create(&work);
    let mut writer = io::BufWriter::new(write_file);
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

fn make_and_check(build_dir: &PathBuf, jobs: &OsString) {
    let mut make = Command::new("make");
    make.current_dir(build_dir).arg("-j").arg(&jobs);
    execute(make);
    let mut make_check = Command::new("make");
    make_check.current_dir(&build_dir).arg("-j").arg(&jobs).arg("check");
    execute(make_check);
}

fn copy_file(src: &Path, dst: &Path) {
    fs::copy(&src, &dst).unwrap_or_else(|_| {
        panic!("Unable to copy {} -> {}", src.display(), dst.display());
    });
}

fn touch(file: &Path) {
    let mut t = Command::new("touch");
    t.arg(file);
    execute(t);
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

fn open(name: &PathBuf) -> fs::File {
    fs::File::open(name)
        .unwrap_or_else(|_| panic!("Cannot open file: {}", name.display()))
}

fn create(name: &PathBuf) -> fs::File {
    fs::File::create(name)
        .unwrap_or_else(|_| panic!("Cannot create file: {}", name.display()))
}

fn read_line(reader: &mut io::BufReader<fs::File>,
             buf: &mut String,
             name: &PathBuf)
             -> usize {
    reader.read_line(buf)
        .unwrap_or_else(|_| panic!("Cannot read from: {}", name.display()))
}

fn write(writer: &mut io::BufWriter<fs::File>, buf: &str, name: &PathBuf) {
    writer.write(buf.as_bytes())
        .unwrap_or_else(|_| panic!("Cannot write to: {}", name.display()));
}

fn flush(writer: &mut io::BufWriter<fs::File>, name: &PathBuf) {
    writer.flush()
        .unwrap_or_else(|_| panic!("Cannot write to: {}", name.display()));
}

fn rename(src: &Path, dst: &Path) {
    fs::rename(&src, &dst).unwrap_or_else(|_| {
        panic!("Unable to rename {} -> {}", src.display(), dst.display());
    });
}
