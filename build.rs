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
use std::fs;
use std::ffi::OsString;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

// mkdir -p "${OUT_DIR}"/lib
// if [ ! -f "${OUT_DIR}"/lib/libgmp.a ]; then
//     rm -r "${OUT_DIR}"/build-gmp
//     mkdir "${OUT_DIR}"/build-gmp
//     (
//         cd "${OUT_DIR}"/build-gmp
//         "${CARGO_MANIFEST_DIR}"/gmp-6.1.2/configure --enable-shared=no \
//             --with-pic=yes
//         sed 's/\(^SUBDIRS = .*\) doc\( \|$\)/\1\2/' Makefile > Makefile.work
//         mv Makefile.work Makefile
//         make -j "${NUM_JOBS}"
//         make -j "${NUM_JOBS}" check
//     )
//     cp "${OUT_DIR}"/.libs/libgmp.a "${OUT_DIR}"/lib
// fi
// if [ ! -f "${OUT_DIR}"/lib/libmpfr.a ]; then
//     rm -r "${OUT_DIR}"/build-mpfr
//     mkdir "${OUT_DIR}"/build-mpfr
//     (
//         cd "${OUT_DIR}"/build-mpfr
//         "${CARGO_MANIFEST_DIR}"/mpfr-3.1.5/configure --enable-shared=no \
//             --with-pic=yes --with-gmp-build="${OUT_DIR}"/build-gmp
//         sed 's/\(^SUBDIRS = .*\) doc\( \|$\)/\1\2/' Makefile > Makefile.work
//         mv Makefile.work Makefile
//         make -j "${NUM_JOBS}"
//         make -j "${NUM_JOBS}" check
//     )
//     cp "${OUT_DIR}"/src/.libs/libmpfr.a "${OUT_DIR}"/lib
// fi

fn main() {
    let src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let jobs = env::var_os("NUM_JOBS").unwrap();
    let make_name = OsString::from("make");
    let lib_dir = out_dir.join("lib");
    let gmp_build_dir = out_dir.join("build-gmp");
    let gmp_src_dir = src_dir.join("gmp-6.1.2");
    let gmp_lib = lib_dir.join("libgmp.a");
    let mpfr_build_dir = out_dir.join("build-mpfr");
    let mpfr_src_dir = src_dir.join("mpfr-3.1.5");
    let mpfr_lib = lib_dir.join("libmpfr.a");

    fs::create_dir_all(&lib_dir).unwrap();
    if !gmp_lib.is_file() {
        let _ = fs::remove_dir_all(&gmp_build_dir);
        fs::create_dir(&gmp_build_dir).unwrap();
        configure(&gmp_src_dir, &gmp_build_dir, None);
        remove_doc_from_makefile(&gmp_build_dir);
        make_and_check(&make_name, &gmp_build_dir, &jobs);
        fs::copy(&gmp_build_dir.join(".libs").join("libgmp.a"), &gmp_lib).unwrap();
    }
    if !mpfr_lib.is_file() {
        let _ = fs::remove_dir_all(&mpfr_build_dir);
        fs::create_dir(&mpfr_build_dir).unwrap();
        let mut option = OsString::from("--with-gmp-build=\"");
        option.push(gmp_build_dir);
        option.push("\"");
        configure(&mpfr_src_dir, &mpfr_build_dir, Some(option));
        remove_doc_from_makefile(&mpfr_build_dir);
        make_and_check(&make_name, &mpfr_build_dir, &jobs);
        fs::copy(&mpfr_build_dir.join("src/.libs/libmpfr.a"), &mpfr_lib).unwrap();
    }

    println!("cargo:rustc-link-lib=static=gmp");
    println!("cargo:rustc-link-lib=static=mpfr");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
}

fn configure(src_dir: &PathBuf, build_dir: &PathBuf, options: Option<OsString>) {
    let mut line = OsString::from("\"");
    line.push(src_dir.join("configure"));
    line.push("\" --enable-shared=no --with-pic=yes");
    if let Some(options) = options {
        line.push(" ");
        line.push(options);
    }
    let mut conf = Command::new("sh");
    conf.current_dir(&build_dir).arg("-c").arg(line);
    execute(conf);
}

fn remove_doc_from_makefile(build_dir: &PathBuf) {
    let makefile = build_dir.join("Makefile");
    let work = build_dir.join("Makefile.work");
    let read_file = fs::File::open(&makefile).unwrap();
    let mut reader = io::BufReader::new(read_file);
    let write_file = fs::File::create(&work).unwrap();
    let mut writer = io::BufWriter::new(write_file);
    let mut buf = String::new();
    while reader.read_line(&mut buf).unwrap() > 0 {
        if buf.starts_with("SUBDIRS = ") {
            if let Some(doc) = buf.find(" doc ") {
                buf.drain(doc..doc + 4);
            }
            if let Some(doc) = buf.find(" doc\n") {
                buf.drain(doc..doc + 4);
            }
        }
        writer.write(buf.as_bytes()).unwrap();
        buf.clear();
    }
    drop(reader);
    // check for write errors
    writer.flush().unwrap();
    drop(writer);
    fs::rename(&work, &makefile).unwrap();
}

fn make_and_check(make_name: &OsString, build_dir: &PathBuf, jobs: &OsString) {
    let mut make = Command::new(make_name);
    make.current_dir(build_dir).arg("-j").arg(&jobs);
    execute(make);
    let mut make_check = Command::new(make_name);
    make_check.current_dir(&build_dir).arg("-j").arg(&jobs).arg("check");
    execute(make_check);
}

fn execute(mut command: Command) {
    println!("$ {:?}", command);
    let status = command.stdout(Stdio::inherit()).stderr(Stdio::inherit()).status().unwrap();
    assert!(status.success());
}
