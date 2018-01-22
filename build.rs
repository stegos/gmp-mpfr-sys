// Copyright © 2017–2018 University of Malta

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
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const GMP_DIR: &'static str = "gmp-6.1.2-c";
const MPFR_DIR: &'static str = "mpfr-4.0.0-c";
const MPC_DIR: &'static str = "mpc-1.1.0-c";

fn main() {
    let src_dir = PathBuf::from(cargo_env("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(cargo_env("OUT_DIR"));
    let jobs = cargo_env("NUM_JOBS");
    let profile = cargo_env("PROFILE");
    let check = there_is_env("CARGO_FEATURE_CTEST")
        || (!there_is_env("CARGO_FEATURE_CNOTEST")
            && profile == OsString::from("release"));

    // The cache dir is for testing purposes, it is *not* meant for
    // general use.
    println!("cargo:rerun-if-env-changed=GMP_MPFR_SYS_CACHE");
    let cache_dir = env::var_os("GMP_MPFR_SYS_CACHE").map(|cache| {
        let version = cargo_env("CARGO_PKG_VERSION");
        PathBuf::from(cache).join(version)
    });

    let lib_dir = out_dir.join("lib");
    let build_dir = out_dir.join("build");
    let gmp_ah = (lib_dir.join("libgmp.a"), lib_dir.join("gmp.h"));
    let mpc_ah = if there_is_env("CARGO_FEATURE_MPC") {
        Some((lib_dir.join("libmpc.a"), lib_dir.join("mpc.h")))
    } else {
        None
    };
    let mpfr_ah = if mpc_ah.is_some() || there_is_env("CARGO_FEATURE_MPFR") {
        Some((lib_dir.join("libmpfr.a"), lib_dir.join("mpfr.h")))
    } else {
        None
    };

    // make sure we have target directory
    create_dir(&lib_dir);
    let (compile_gmp, compile_mpfr, compile_mpc) =
        need_compile(&cache_dir, check, &gmp_ah, &mpfr_ah, &mpc_ah);

    if compile_gmp {
        remove_dir(&build_dir);
        create_dir(&build_dir);
        symlink(
            &build_dir,
            &dir_relative(&build_dir, &src_dir.join(GMP_DIR)),
            Some(&OsString::from("gmp-src")),
        );
        let (ref a, ref h) = gmp_ah;
        build_gmp(&build_dir, &jobs, check, a, h);
    }
    if compile_mpfr {
        symlink(
            &build_dir,
            &dir_relative(&build_dir, &src_dir.join(MPFR_DIR)),
            Some(&OsString::from("mpfr-src")),
        );
        let (ref a, ref h) = *mpfr_ah.as_ref().unwrap();
        build_mpfr(&build_dir, &jobs, check, a, h);
    }
    if compile_mpc {
        symlink(
            &build_dir,
            &dir_relative(&build_dir, &src_dir.join(MPC_DIR)),
            Some(&OsString::from("mpc-src")),
        );
        let (ref a, ref h) = *mpc_ah.as_ref().unwrap();
        build_mpc(&build_dir, &jobs, check, a, h);
    }
    if compile_gmp {
        remove_dir(&build_dir);
        if let Some(ref dir) = cache_dir {
            // ignore error, do not bail if saving cache fails
            save_cache(dir, check, &gmp_ah, &mpfr_ah, &mpc_ah).is_err();
        }
    }
    process_gmp_header(&gmp_ah.1, &out_dir.join("gmp_h.rs"));
    write_link_info(&lib_dir, mpfr_ah.is_some(), mpc_ah.is_some());
}

fn need_compile(
    cache_dir: &Option<PathBuf>,
    check: bool,
    gmp_ah: &(PathBuf, PathBuf),
    mpfr_ah: &Option<(PathBuf, PathBuf)>,
    mpc_ah: &Option<(PathBuf, PathBuf)>,
) -> (bool, bool, bool) {
    let gmp_fine = gmp_ah.0.is_file() && gmp_ah.1.is_file();
    let mpfr_fine = match *mpfr_ah {
        Some((ref a, ref h)) => a.is_file() && h.is_file(),
        None => true,
    };
    let mpc_fine = match *mpc_ah {
        Some((ref a, ref h)) => a.is_file() && h.is_file(),
        None => true,
    };
    if gmp_fine && mpfr_fine && mpc_fine {
        if let Some(ref dir) = *cache_dir {
            if !has_cache(dir, check, mpfr_ah.is_some(), mpc_ah.is_some()) {
                // ignore error, do not bail if saving cache fails
                save_cache(dir, check, gmp_ah, mpfr_ah, mpc_ah).is_err();
            }
        }
        return (false, false, false);
    } else if let Some(ref dir) = *cache_dir {
        // if loading cache works, we're done
        if load_cache(dir, check, gmp_ah, mpfr_ah, mpc_ah) {
            return (false, false, false);
        }
    }
    let need_mpc = !mpc_fine;
    let need_mpfr = need_mpc || !mpfr_fine;
    let need_gmp = need_mpfr || !gmp_fine;
    (need_gmp, need_mpfr, need_mpc)
}

fn save_cache(
    cache_dir: &PathBuf,
    check: bool,
    gmp_ah: &(PathBuf, PathBuf),
    mpfr_ah: &Option<(PathBuf, PathBuf)>,
    mpc_ah: &Option<(PathBuf, PathBuf)>,
) -> Result<(), io::Error> {
    let req_check = if check { "check" } else { "nocheck" };
    let req_libs = if mpc_ah.is_some() {
        "gmp_mpfr_mpc"
    } else if mpfr_ah.is_some() {
        "gmp_mpfr"
    } else {
        "gmp"
    };
    let dir = cache_dir.join(req_check).join(req_libs);
    fs::create_dir_all(&dir)?;
    let (ref a, ref h) = *gmp_ah;
    fs::copy(a, dir.join("libgmp.a"))?;
    fs::copy(h, dir.join("gmp.h"))?;
    if let Some((ref a, ref h)) = *mpfr_ah {
        fs::copy(a, dir.join("libmpfr.a"))?;
        fs::copy(h, dir.join("mpfr.h"))?;
    }
    if let Some((ref a, ref h)) = *mpc_ah {
        fs::copy(a, dir.join("libmpc.a"))?;
        fs::copy(h, dir.join("mpc.h"))?;
    }
    Ok(())
}

fn load_cache(
    cache_dir: &PathBuf,
    check: bool,
    gmp_ah: &(PathBuf, PathBuf),
    mpfr_ah: &Option<(PathBuf, PathBuf)>,
    mpc_ah: &Option<(PathBuf, PathBuf)>,
) -> bool {
    let checks = ["nocheck", "check"];
    let req_checks = if check { &checks[1..] } else { &checks };
    for req_check in req_checks {
        let check_dir = cache_dir.join(req_check);
        // first try "gmp" directory
        if mpfr_ah.is_none() {
            let dir = check_dir.join("gmp");
            let (ref a, ref h) = *gmp_ah;
            let mut ok = fs::copy(dir.join("libgmp.a"), a).is_ok();
            ok = ok && fs::copy(dir.join("gmp.h"), h).is_ok();
            if ok {
                return true;
            }
        }
        // next try "gmp_mpfr" directory
        if mpc_ah.is_none() {
            let dir = check_dir.join("gmp_mpfr");
            let (ref a, ref h) = *gmp_ah;
            let mut ok = fs::copy(dir.join("libgmp.a"), a).is_ok();
            ok = ok && fs::copy(dir.join("gmp.h"), h).is_ok();
            if let Some((ref a, ref h)) = *mpfr_ah {
                ok = ok && fs::copy(dir.join("libmpfr.a"), a).is_ok();
                ok = ok && fs::copy(dir.join("mpfr.h"), h).is_ok();
            }
            if ok {
                return true;
            }
        }
        // finally try "gmp_mpfr_mpc" directory
        let dir = check_dir.join("gmp_mpfr_mpc");
        let (ref a, ref h) = *gmp_ah;
        let mut ok = fs::copy(dir.join("libgmp.a"), a).is_ok();
        ok = ok && fs::copy(dir.join("gmp.h"), h).is_ok();
        if let Some((ref a, ref h)) = *mpfr_ah {
            ok = ok && fs::copy(dir.join("libmpfr.a"), a).is_ok();
            ok = ok && fs::copy(dir.join("mpfr.h"), h).is_ok();
        }
        if let Some((ref a, ref h)) = *mpc_ah {
            ok = ok && fs::copy(dir.join("libmpc.a"), a).is_ok();
            ok = ok && fs::copy(dir.join("mpc.h"), h).is_ok();
        }
        if ok {
            return true;
        }
    }
    false
}

fn has_cache(cache_dir: &PathBuf, check: bool, mpfr: bool, mpc: bool) -> bool {
    let checks = ["nocheck", "check"];
    let req_checks = if check { &checks[1..] } else { &checks };
    for req_check in req_checks {
        let check_dir = cache_dir.join(req_check);
        // first try "gmp" directory
        if !mpfr {
            let dir = check_dir.join("gmp");
            let mut ok = dir.join("libgmp.a").is_file();
            ok = ok && dir.join("gmp.h").is_file();
            if ok {
                return true;
            }
        }
        // next try "gmp_mpfr" directory
        if !mpc {
            let dir = check_dir.join("gmp_mpfr");
            let mut ok = dir.join("libgmp.a").is_file();
            ok = ok && dir.join("gmp.h").is_file();
            if mpfr {
                ok = ok && dir.join("libmpfr.a").is_file();
                ok = ok && dir.join("mpfr.h").is_file();
            }
            if ok {
                return true;
            }
        }
        // finally try "gmp_mpfr_mpc" directory
        let dir = check_dir.join("gmp_mpfr_mpc");
        let mut ok = dir.join("libgmp.a").is_file();
        ok = ok && dir.join("gmp.h").is_file();
        if mpfr {
            ok = ok && dir.join("libmpfr.a").is_file();
            ok = ok && dir.join("mpfr.h").is_file();
        }
        if mpc {
            ok = ok && dir.join("libmpc.a").is_file();
            ok = ok && dir.join("mpc.h").is_file();
        }
        if ok {
            return true;
        }
    }
    false
}

fn build_gmp(
    top_build_dir: &Path,
    jobs: &OsStr,
    check: bool,
    lib: &Path,
    header: &Path,
) {
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
            cc = Some(
                buf[(start + s.len())..]
                    .trim()
                    .trim_matches('"')
                    .to_string(),
            );
        }
        let s = "#define __GMP_CFLAGS";
        if let Some(start) = buf.find(s) {
            cflags = Some(
                buf[(start + s.len())..]
                    .trim()
                    .trim_matches('"')
                    .to_string(),
            );
        }
        buf.clear();
    }
    drop(reader);

    let limb_bits =
        limb_bits.expect("Cannot determine GMP_LIMB_BITS from gmp.h");
    println!("cargo:limb_bits={}", limb_bits);

    let nail_bits =
        nail_bits.expect("Cannot determine GMP_NAIL_BITS from gmp.h");
    if nail_bits > 0 {
        println!("cargo:rustc-cfg=nails");
    }

    let long_long_limb =
        long_long_limb.expect("Cannot determine _LONG_LONG_LIMB from gmp.h");
    let long_long_limb = if long_long_limb {
        println!("cargo:rustc-cfg=long_long_limb");
        "::std::os::raw::c_ulonglong"
    } else {
        "::std::os::raw::c_ulong"
    };
    let cc = cc.expect("Cannot determine __GMP_CC from gmp.h");
    let cflags = cflags.expect("Cannot determine __GMP_CFLAGS from gmp.h");
    let content = format!(
        concat!(
            "const GMP_LIMB_BITS: c_int = {};\n",
            "const GMP_NAIL_BITS: c_int = {};\n",
            "type GMP_LIMB_T = {};\n",
            "const GMP_CC: *const c_char =\n",
            "    b\"{}\\0\" as *const _ as _;\n",
            "const GMP_CFLAGS: *const c_char =\n",
            "    b\"{}\\0\" as *const _ as _;\n"
        ),
        limb_bits,
        nail_bits,
        long_long_limb,
        cc,
        cflags
    );

    let mut rs = create(out_file);
    write(&mut rs, &content, out_file);
    flush(&mut rs, out_file);
}

fn build_mpfr(
    top_build_dir: &Path,
    jobs: &OsStr,
    check: bool,
    lib: &Path,
    header: &Path,
) {
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
    let src_header = top_build_dir.join("mpfr-src").join("src").join("mpfr.h");
    copy_file(&src_header, &header);
}

fn build_mpc(
    top_build_dir: &Path,
    jobs: &OsStr,
    check: bool,
    lib: &Path,
    header: &Path,
) {
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

fn write_link_info(lib_dir: &Path, feature_mpfr: bool, feature_mpc: bool) {
    let lib_search = lib_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            lib_dir.display()
        )
    });
    println!("cargo:lib_dir={}", lib_search);
    println!("cargo:rustc-link-search=native={}", lib_search);
    if feature_mpc {
        println!("cargo:rustc-link-lib=static=mpc");
    }
    if feature_mpfr {
        println!("cargo:rustc-link-lib=static=mpfr");
    }
    println!("cargo:rustc-link-lib=static=gmp");
    check_mingw(feature_mpfr, feature_mpc);
}

fn cargo_env(name: &str) -> OsString {
    env::var_os(name).unwrap_or_else(|| {
        panic!("environment variable not found: {}, please use cargo", name)
    })
}

fn there_is_env(name: &str) -> bool {
    env::var_os(name).is_some()
}

fn check_mingw(feature_mpfr: bool, _feature_mpc: bool) {
    // extra libraries needed only for mpfr because of thread-local storage
    if !feature_mpfr {
        return;
    }

    for check in &["HOST", "TARGET"] {
        if !cargo_env(check)
            .into_string()
            .map(|s| s.ends_with("-windows-gnu"))
            .unwrap_or(false)
        {
            return;
        }
    }

    // link to gcc_eh
    println!("cargo:rustc-link-lib=static=gcc_eh");

    // also link to pthread, but only if rustc version >= 1.18
    if rustc_later_eq(1, 18) {
        println!("cargo:rustc-link-lib=static=pthread");
    }
}

fn rustc_later_eq(major: i32, minor: i32) -> bool {
    let rustc = cargo_env("RUSTC");
    let output = Command::new(rustc)
        .arg("--version")
        .output()
        .expect("unable to run rustc --version");
    let version =
        String::from_utf8(output.stdout).expect("unrecognized rustc version");
    if !version.starts_with("rustc ") {
        panic!("unrecognized rustc version: {}", version);
    }
    let remain = &version[6..];
    let dot = remain.find('.').expect("unrecognized rustc version");
    let ver_major = remain[0..dot]
        .parse::<i32>()
        .expect("unrecognized rustc version");
    if ver_major < major {
        return false;
    } else if ver_major > major {
        return true;
    }
    let remain = &remain[dot + 1..];
    let dot = remain.find('.').expect("unrecognized rustc version");
    let ver_minor = remain[0..dot]
        .parse::<i32>()
        .expect("unrecognized rustc version");
    ver_minor >= minor
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
    assert!(
        some_common,
        "cannot access {} from {} using relative paths",
        rel_to.display(),
        dir.display()
    );
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
        panic!("Unable to copy {} -> {}", src.display(), dst.display());
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
    let status = command
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
    let file = File::open(name)
        .unwrap_or_else(|_| panic!("Cannot open file: {}", name.display()));
    BufReader::new(file)
}

fn create(name: &Path) -> BufWriter<File> {
    let file = File::create(name)
        .unwrap_or_else(|_| panic!("Cannot create file: {}", name.display()));
    BufWriter::new(file)
}

fn read_line(
    reader: &mut BufReader<File>,
    buf: &mut String,
    name: &Path,
) -> usize {
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
