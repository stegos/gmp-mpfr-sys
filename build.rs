// Copyright (C) 2017  University of Malta

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
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let jobs = env::var("NUM_JOBS").unwrap();
    let make = "make";
    let lib_dir = out_dir.join("lib");
    let gmp_build_dir = out_dir.join("build-gmp");
    let gmp_src_dir = src_dir.join("gmp-6.1.2");
    let gmp_lib = lib_dir.join("libgmp.a");
    let mpfr_build_dir = out_dir.join("build-mpfr");
    let mpfr_src_dir = src_dir.join("mpfr-3.1.5");
    let mpfr_lib = lib_dir.join("libmpfr.a");

    if !lib_dir.exists() || !gmp_lib.exists() || !mpfr_lib.exists() {
        let _ = fs::remove_dir_all(&lib_dir);
        fs::create_dir(&lib_dir).unwrap();

        let _ = fs::remove_dir_all(&gmp_build_dir);
        fs::create_dir(&gmp_build_dir).unwrap();
        let gmp_conf_path = gmp_src_dir.join("configure");
        let gmp_conf = gmp_conf_path.to_str().unwrap();
        let mut gmp_conf_cmd = Command::new("sh");
        gmp_conf_cmd.current_dir(&gmp_build_dir)
            .arg("-c")
            .arg(format!("{} --enable-shared=no --with-pic=yes", gmp_conf));
        execute(gmp_conf_cmd);
        remove_doc_from_makefile(&gmp_build_dir);
        let mut gmp_make_cmd = Command::new(make);
        gmp_make_cmd.current_dir(&gmp_build_dir).arg("-j").arg(&jobs);
        execute(gmp_make_cmd);
        let mut gmp_check_cmd = Command::new(make);
        gmp_check_cmd.current_dir(&gmp_build_dir).arg("-j").arg(&jobs).arg("check");
        execute(gmp_check_cmd);
        fs::copy(&gmp_build_dir.join(".libs/libgmp.a"), &gmp_lib).unwrap();

        let _ = fs::remove_dir_all(&mpfr_build_dir);
        fs::create_dir(&mpfr_build_dir).unwrap();
        let mpfr_conf_path = mpfr_src_dir.join("configure");
        let mpfr_conf = mpfr_conf_path.to_str().unwrap();
        let mut mpfr_conf_cmd = Command::new("sh");
        mpfr_conf_cmd.current_dir(&mpfr_build_dir)
            .arg("-c")
            .arg(format!("{} --enable-shared=no --with-pic=yes --with-gmp-build={}",
                         mpfr_conf,
                         gmp_build_dir.to_str().unwrap()));
        execute(mpfr_conf_cmd);
        remove_doc_from_makefile(&mpfr_build_dir);
        let mut mpfr_make_cmd = Command::new(make);
        mpfr_make_cmd.current_dir(&mpfr_build_dir).arg("-j").arg(&jobs);
        execute(mpfr_make_cmd);
        let mut mpfr_check_cmd = Command::new(make);
        mpfr_check_cmd.current_dir(&mpfr_build_dir).arg("-j").arg(&jobs).arg("check");
        execute(mpfr_check_cmd);
        fs::copy(&mpfr_build_dir.join("src/.libs/libmpfr.a"), &mpfr_lib).unwrap();
    }

    println!("cargo:rustc-flags=-L {} -l gmp -l mpfr",
             lib_dir.to_str().unwrap());
}

fn execute(mut command: Command) {
    println!("$ {:?}", command);
    let status = command.stdout(Stdio::inherit()).stderr(Stdio::inherit()).status().unwrap();
    assert!(status.success());
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
    drop(writer);
    fs::remove_file(&makefile).unwrap();
    fs::rename(&work, &makefile).unwrap();
}
