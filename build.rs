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
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Result as IoResult, Write};
use std::mem;
use std::path::{Path, PathBuf};
use std::process::Command;

const GMP_DIR: &'static str = "gmp-6.1.2-c";
const MPFR_DIR: &'static str = "mpfr-4.0.1-p6-c";
const MPC_DIR: &'static str = "mpc-1.1.0-c";

#[derive(Clone, Copy, PartialEq)]
enum Target {
    Mingw,
    Msvc,
    Other,
}

struct Environment {
    out_dir: PathBuf,
    lib_dir: PathBuf,
    include_dir: PathBuf,
    build_dir: PathBuf,
    cache_dir: Option<PathBuf>,
    jobs: OsString,
    target: Target,
    make_check: bool,
}

fn main() {
    let src_dir = PathBuf::from(cargo_env("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(cargo_env("OUT_DIR"));

    // The cache dir is for testing purposes, it is *not* meant for
    // general use.
    println!("cargo:rerun-if-env-changed=GMP_MPFR_SYS_CACHE");
    let cache_dir = env::var_os("GMP_MPFR_SYS_CACHE").map(|cache| {
        let version = cargo_env("CARGO_PKG_VERSION");
        PathBuf::from(cache).join(version)
    });

    let host = cargo_env("HOST");
    let target = cargo_env("TARGET");
    assert_eq!(host, target, "cross compilation is not supported");
    let target = target
        .into_string()
        .expect("cannot convert environment variable TARGET into a `String`");
    let target = if target.contains("-windows-msvc") {
        Target::Msvc
    } else if target.contains("-windows-gnu") {
        Target::Mingw
    } else {
        Target::Other
    };

    let make_check = there_is_env("CARGO_FEATURE_CTEST")
        || (!there_is_env("CARGO_FEATURE_CNOTEST")
            && cargo_env("PROFILE") == OsString::from("release"));

    let env = Environment {
        out_dir: out_dir.clone(),
        lib_dir: out_dir.join("lib"),
        include_dir: out_dir.join("include"),
        build_dir: out_dir.join("build"),
        cache_dir: cache_dir,
        jobs: cargo_env("NUM_JOBS"),
        target: target,
        make_check: make_check,
    };

    // make sure we have target directories
    create_dir_or_panic(&env.lib_dir);
    create_dir_or_panic(&env.include_dir);

    let gmp_ah = (env.lib_dir.join("libgmp.a"), env.include_dir.join("gmp.h"));
    let mpc_ah = if there_is_env("CARGO_FEATURE_MPC") {
        Some((env.lib_dir.join("libmpc.a"), env.include_dir.join("mpc.h")))
    } else {
        None
    };
    let mpfr_ah = if mpc_ah.is_some() || there_is_env("CARGO_FEATURE_MPFR") {
        Some((
            env.lib_dir.join("libmpfr.a"),
            env.include_dir.join("mpfr.h"),
        ))
    } else {
        None
    };

    let (compile_gmp, compile_mpfr, compile_mpc) =
        need_compile(&env, &gmp_ah, &mpfr_ah, &mpc_ah);
    if compile_gmp {
        check_for_msvc(&env);
        remove_dir_or_panic(&env.build_dir);
        create_dir_or_panic(&env.build_dir);
        check_for_bug_47048(&env);
        link_or_copy_dir(
            &src_dir.join(GMP_DIR),
            &env.build_dir,
            "gmp-src",
            target,
        );
        let (ref a, ref h) = gmp_ah;
        build_gmp(&env, a, h);
    }
    if compile_mpfr {
        link_or_copy_dir(
            &src_dir.join(MPFR_DIR),
            &env.build_dir,
            "mpfr-src",
            target,
        );
        let (ref a, ref h) = *mpfr_ah.as_ref().unwrap();
        build_mpfr(&env, a, h);
    }
    if compile_mpc {
        link_or_copy_dir(
            &src_dir.join(MPC_DIR),
            &env.build_dir,
            "mpc-src",
            target,
        );
        let (ref a, ref h) = *mpc_ah.as_ref().unwrap();
        build_mpc(&env, a, h);
    }
    if compile_gmp {
        remove_dir_or_panic(&env.build_dir);
        save_cache(&env, &gmp_ah, &mpfr_ah, &mpc_ah);
    }
    process_gmp_header(&gmp_ah.1, &out_dir.join("gmp_h.rs"));
    write_link_info(&env, mpfr_ah.is_some(), mpc_ah.is_some());
}

fn need_compile(
    env: &Environment,
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
        if should_save_cache(env, mpfr_ah.is_some(), mpc_ah.is_some()) {
            save_cache(env, gmp_ah, mpfr_ah, mpc_ah);
        }
        return (false, false, false);
    } else if load_cache(env, gmp_ah, mpfr_ah, mpc_ah) {
        // if loading cache works, we're done
        return (false, false, false);
    }
    let need_mpc = !mpc_fine;
    let need_mpfr = need_mpc || !mpfr_fine;
    let need_gmp = need_mpfr || !gmp_fine;
    (need_gmp, need_mpfr, need_mpc)
}

fn save_cache(
    env: &Environment,
    gmp_ah: &(PathBuf, PathBuf),
    mpfr_ah: &Option<(PathBuf, PathBuf)>,
    mpc_ah: &Option<(PathBuf, PathBuf)>,
) -> bool {
    let cache_dir = match env.cache_dir {
        Some(ref s) => s,
        None => return false,
    };
    let req_check = if env.make_check { "check" } else { "nocheck" };
    let req_libs = if mpc_ah.is_some() {
        "gmp_mpfr_mpc"
    } else if mpfr_ah.is_some() {
        "gmp_mpfr"
    } else {
        "gmp"
    };
    let dir = cache_dir.join(req_check).join(req_libs);
    let mut ok = create_dir(&dir).is_ok();
    let (ref a, ref h) = *gmp_ah;
    ok = ok && copy_file(a, &dir.join("libgmp.a")).is_ok();
    ok = ok && copy_file(h, &dir.join("gmp.h")).is_ok();
    if let Some((ref a, ref h)) = *mpfr_ah {
        ok = ok && copy_file(a, &dir.join("libmpfr.a")).is_ok();
        ok = ok && copy_file(h, &dir.join("mpfr.h")).is_ok();
    }
    if let Some((ref a, ref h)) = *mpc_ah {
        ok = ok && copy_file(a, &dir.join("libmpc.a")).is_ok();
        ok = ok && copy_file(h, &dir.join("mpc.h")).is_ok();
    }
    ok
}

fn load_cache(
    env: &Environment,
    gmp_ah: &(PathBuf, PathBuf),
    mpfr_ah: &Option<(PathBuf, PathBuf)>,
    mpc_ah: &Option<(PathBuf, PathBuf)>,
) -> bool {
    let cache_dir = match env.cache_dir {
        Some(ref s) => s,
        None => return false,
    };
    let checks = ["nocheck", "check"];
    let req_checks = if env.make_check {
        &checks[1..]
    } else {
        &checks
    };
    for req_check in req_checks {
        let check_dir = cache_dir.join(req_check);
        // first try "gmp" directory
        if mpfr_ah.is_none() {
            let dir = check_dir.join("gmp");
            let (ref a, ref h) = *gmp_ah;
            let mut ok = copy_file(&dir.join("libgmp.a"), a).is_ok();
            ok = ok && copy_file(&dir.join("gmp.h"), h).is_ok();
            if ok {
                return true;
            }
        }
        // next try "gmp_mpfr" directory
        if mpc_ah.is_none() {
            let dir = check_dir.join("gmp_mpfr");
            let (ref a, ref h) = *gmp_ah;
            let mut ok = copy_file(&dir.join("libgmp.a"), a).is_ok();
            ok = ok && copy_file(&dir.join("gmp.h"), h).is_ok();
            if let Some((ref a, ref h)) = *mpfr_ah {
                ok = ok && copy_file(&dir.join("libmpfr.a"), a).is_ok();
                ok = ok && copy_file(&dir.join("mpfr.h"), h).is_ok();
            }
            if ok {
                return true;
            }
        }
        // finally try "gmp_mpfr_mpc" directory
        let dir = check_dir.join("gmp_mpfr_mpc");
        let (ref a, ref h) = *gmp_ah;
        let mut ok = copy_file(&dir.join("libgmp.a"), a).is_ok();
        ok = ok && copy_file(&dir.join("gmp.h"), h).is_ok();
        if let Some((ref a, ref h)) = *mpfr_ah {
            ok = ok && copy_file(&dir.join("libmpfr.a"), a).is_ok();
            ok = ok && copy_file(&dir.join("mpfr.h"), h).is_ok();
        }
        if let Some((ref a, ref h)) = *mpc_ah {
            ok = ok && copy_file(&dir.join("libmpc.a"), a).is_ok();
            ok = ok && copy_file(&dir.join("mpc.h"), h).is_ok();
        }
        if ok {
            return true;
        }
    }
    false
}

fn should_save_cache(env: &Environment, mpfr: bool, mpc: bool) -> bool {
    let cache_dir = match env.cache_dir {
        Some(ref s) => s,
        None => return false,
    };
    let checks = ["nocheck", "check"];
    let req_checks = if env.make_check {
        &checks[1..]
    } else {
        &checks
    };
    for req_check in req_checks {
        let check_dir = cache_dir.join(req_check);
        // first try "gmp" directory
        if !mpfr {
            let dir = check_dir.join("gmp");
            let mut ok = dir.join("libgmp.a").is_file();
            ok = ok && dir.join("gmp.h").is_file();
            if ok {
                return false;
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
                return false;
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
            return false;
        }
    }
    true
}

fn build_gmp(env: &Environment, lib: &Path, header: &Path) {
    let build_dir = env.build_dir.join("gmp-build");
    create_dir_or_panic(&build_dir);
    println!("$ cd {:?}", build_dir);
    let conf = "../gmp-src/configure --enable-fat --disable-shared --with-pic";
    configure(&build_dir, &OsString::from(conf));
    make_and_check(env, &build_dir);
    let build_lib = build_dir.join(".libs").join("libgmp.a");
    copy_file_or_panic(&build_lib, &lib);
    let build_header = build_dir.join("gmp.h");
    copy_file_or_panic(&build_header, &header);
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

fn build_mpfr(env: &Environment, lib: &Path, header: &Path) {
    let build_dir = env.build_dir.join("mpfr-build");
    create_dir_or_panic(&build_dir);
    println!("$ cd {:?}", build_dir);
    symlink("../gmp-build", &build_dir);
    let conf = "../mpfr-src/configure --enable-thread-safe --disable-shared \
                --with-gmp-build=../gmp-build --with-pic";
    configure(&build_dir, &OsString::from(conf));
    make_and_check(env, &build_dir);
    let build_lib = build_dir.join("src").join(".libs").join("libmpfr.a");
    copy_file_or_panic(&build_lib, &lib);
    let src_header = env.build_dir.join("mpfr-src").join("src").join("mpfr.h");
    copy_file_or_panic(&src_header, &header);
}

fn build_mpc(env: &Environment, lib: &Path, header: &Path) {
    let build_dir = env.build_dir.join("mpc-build");
    create_dir_or_panic(&build_dir);
    println!("$ cd {:?}", build_dir);
    symlink("../mpfr-src", &build_dir);
    symlink("../mpfr-build", &build_dir);
    // steal link from mpfr-build to save some copying under MinGW,
    // where a symlink is a just a copy.
    mv("../mpfr-build/gmp-build", &build_dir);
    let conf = "../mpc-src/configure --disable-shared \
                --with-mpfr-include=../mpfr-src/src \
                --with-mpfr-lib=../mpfr-build/src/.libs \
                --with-gmp-include=../gmp-build \
                --with-gmp-lib=../gmp-build/.libs --with-pic";
    configure(&build_dir, &OsString::from(conf));
    make_and_check(env, &build_dir);
    let build_lib = build_dir.join("src").join(".libs").join("libmpc.a");
    copy_file_or_panic(&build_lib, &lib);
    let src_header = env.build_dir.join("mpc-src").join("src").join("mpc.h");
    copy_file_or_panic(&src_header, &header);
}

fn write_link_info(env: &Environment, feature_mpfr: bool, feature_mpc: bool) {
    let out_str = env.out_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            env.out_dir.display()
        )
    });
    let lib_str = env.lib_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            env.lib_dir.display()
        )
    });
    let include_str = env.include_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            env.include_dir.display()
        )
    });
    println!("cargo:out_dir={}", out_str);
    println!("cargo:lib_dir={}", lib_str);
    println!("cargo:include_dir={}", include_str);
    println!("cargo:rustc-link-search=native={}", lib_str);
    if feature_mpc {
        println!("cargo:rustc-link-lib=static=mpc");
    }
    if feature_mpfr {
        println!("cargo:rustc-link-lib=static=mpfr");
    }
    println!("cargo:rustc-link-lib=static=gmp");
    if env.target == Target::Mingw {
        add_mingw_libs(feature_mpfr, feature_mpc);
    }
}

fn cargo_env(name: &str) -> OsString {
    env::var_os(name).unwrap_or_else(|| {
        panic!("environment variable not found: {}, please use cargo", name)
    })
}

fn there_is_env(name: &str) -> bool {
    env::var_os(name).is_some()
}

fn check_for_msvc(env: &Environment) {
    if env.target == Target::Msvc {
        panic!("Windows MSVC target is not supported (linking would fail)");
    }
}

fn check_for_bug_47048(env: &Environment) {
    if env.target != Target::Mingw {
        return;
    }
    let try_dir = env.build_dir.join("try_47048");
    let rustc = cargo_env("RUSTC");
    create_dir_or_panic(&try_dir);
    println!("$ cd {:?}", try_dir);
    create_file_or_panic(&try_dir.join("say_hi.c"), BUG_47048_SAY_HI_C);
    create_file_or_panic(&try_dir.join("c_main.c"), BUG_47048_C_MAIN_C);
    create_file_or_panic(&try_dir.join("r_main.rs"), BUG_47048_R_MAIN_RS);
    let mut cmd;

    cmd = Command::new("gcc");
    cmd.current_dir(&try_dir).args(&["-c", "say_hi.c"]);
    execute(cmd);

    cmd = Command::new("ar");
    cmd.current_dir(&try_dir)
        .args(&["cr", "libsay_hi.a", "say_hi.o"]);
    execute(cmd);

    cmd = Command::new("gcc");
    cmd.current_dir(&try_dir).args(&[
        "c_main.c",
        "-L.",
        "-lsay_hi",
        "-o",
        "c_main.exe",
    ]);
    execute(cmd);

    // try simple rustc command that should work, so that failure
    // really is the bug being checked for
    cmd = Command::new(&rustc);
    cmd.arg("--version");
    execute(cmd);

    cmd = Command::new(&rustc);
    cmd.current_dir(&try_dir).args(&[
        "r_main.rs",
        "-L.",
        "-lsay_hi",
        "-o",
        "r_main.exe",
    ]);
    println!("$ {:?}", cmd);
    let status = cmd
        .status()
        .unwrap_or_else(|_| panic!("Unable to execute: {:?}", cmd));
    if !status.success() {
        let message = match mem::size_of::<usize>() {
            4 => BUG_47048_MESSAGE_32,
            8 => BUG_47048_MESSAGE_64,
            _ => unreachable!(),
        };
        panic!("{}", message);
    }

    remove_dir_or_panic(&try_dir);
}

fn add_mingw_libs(feature_mpfr: bool, _feature_mpc: bool) {
    // extra libraries needed only for mpfr because of thread-local storage
    if !feature_mpfr {
        return;
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

fn remove_dir(dir: &Path) -> IoResult<()> {
    if !dir.exists() {
        return Ok(());
    }
    assert!(dir.is_dir(), "Not a directory: {:?}", dir);
    println!("$ rm -r {:?}", dir);
    fs::remove_dir_all(dir)
}

fn remove_dir_or_panic(dir: &Path) {
    remove_dir(dir)
        .unwrap_or_else(|_| panic!("Unable to remove directory: {:?}", dir));
}

fn create_dir(dir: &Path) -> IoResult<()> {
    println!("$ mkdir -p {:?}", dir);
    fs::create_dir_all(dir)
}

fn create_dir_or_panic(dir: &Path) {
    create_dir(dir)
        .unwrap_or_else(|_| panic!("Unable to create directory: {:?}", dir));
}

fn create_file_or_panic(filename: &Path, contents: &str) {
    println!("$ printf '%s' {:?}... > {:?}", &contents[0..10], filename);
    let mut file = File::create(filename)
        .unwrap_or_else(|_| panic!("Unable to create file: {:?}", filename));
    file.write_all(contents.as_bytes())
        .unwrap_or_else(|_| panic!("Unable to write to file: {:?}", filename));
}

fn copy_file(src: &Path, dst: &Path) -> IoResult<u64> {
    println!("$ cp {:?} {:?}", src, dst);
    fs::copy(src, dst)
}

fn copy_file_or_panic(src: &Path, dst: &Path) {
    copy_file(src, dst).unwrap_or_else(|_| {
        panic!("Unable to copy {:?} -> {:?}", src, dst);
    });
}

fn configure(build_dir: &Path, conf_line: &OsStr) {
    let mut conf = Command::new("sh");
    conf.current_dir(&build_dir).arg("-c").arg(conf_line);
    execute(conf);
}

fn make_and_check(env: &Environment, build_dir: &Path) {
    let mut make = Command::new("make");
    make.current_dir(build_dir).arg("-j").arg(&env.jobs);
    execute(make);
    if env.make_check {
        let mut make_check = Command::new("make");
        make_check
            .current_dir(build_dir)
            .arg("-j")
            .arg(&env.jobs)
            .arg("check");
        execute(make_check);
    }
}

fn link_or_copy_dir(
    src: &Path,
    dst_dir: &Path,
    dst_name: &str,
    target: Target,
) {
    println!("$ cd {:?}", dst_dir);
    let mut c;
    if target == Target::Mingw {
        c = Command::new("cp");
        c.arg("-R");
    } else {
        c = Command::new("ln");
        c.arg("-s");
    }
    c.arg(src).arg(dst_name).current_dir(dst_dir);
    execute(c);
}

fn symlink(src: &str, dst_dir: &Path) {
    let mut c = Command::new("ln");
    c.arg("-s").arg(src).current_dir(dst_dir);
    execute(c);
}

fn mv(src: &str, dst_dir: &Path) {
    let mut c = Command::new("mv");
    c.arg(src).arg(".").current_dir(dst_dir);
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
        .unwrap_or_else(|_| panic!("Cannot open file: {:?}", name));
    BufReader::new(file)
}

fn create(name: &Path) -> BufWriter<File> {
    let file = File::create(name)
        .unwrap_or_else(|_| panic!("Cannot create file: {:?}", name));
    BufWriter::new(file)
}

fn read_line(
    reader: &mut BufReader<File>,
    buf: &mut String,
    name: &Path,
) -> usize {
    reader
        .read_line(buf)
        .unwrap_or_else(|_| panic!("Cannot read from: {:?}", name))
}

fn write(writer: &mut BufWriter<File>, buf: &str, name: &Path) {
    writer
        .write(buf.as_bytes())
        .unwrap_or_else(|_| panic!("Cannot write to: {:?}", name));
}

fn flush(writer: &mut BufWriter<File>, name: &Path) {
    writer
        .flush()
        .unwrap_or_else(|_| panic!("Cannot write to: {:?}", name));
}

const BUG_47048_SAY_HI_C: &'static str = r#"/* say_hi.c */
#include <stdio.h>
void say_hi(void) {
    fprintf(stdout, "hi!\n");
}
"#;

const BUG_47048_C_MAIN_C: &'static str = r#"/* c_main.c */
void say_hi(void);
int main(void) {
    say_hi();
    return 0;
}
"#;

const BUG_47048_R_MAIN_RS: &'static str = r#"// r_main.rs
extern "C" {
    fn say_hi();
}
fn main() {
    unsafe {
        say_hi();
    }
}
"#;

const BUG_47048_MESSAGE_32: &'static str = r#"
Detected rustc bug 47048.

As a workaround, you can downgrade the MinGW headers and crt packages
using the following steps:

* Download the following two packages:
  1. http://repo.msys2.org/mingw/i686/mingw-w64-i686-crt-git-5.0.0.5002.34a7c1c0-1-any.pkg.tar.xz
  2. http://repo.msys2.org/mingw/i686/mingw-w64-i686-headers-git-5.0.0.5002.34a7c1c0-1-any.pkg.tar.xz

* Downgrade using the following bash command:
  pacman -U mingw-w64-i686-{crt,headers}-git-5.0.0.5002.34a7c1c0-1-any.pkg.tar.xz

More details at: https://github.com/rust-lang/rust/issues/47048

"#;

const BUG_47048_MESSAGE_64: &'static str = r#"
Detected rustc bug 47048.

As a workaround, you can downgrade the MinGW headers and crt packages
using the following steps:

* Download the following two packages:
  1. http://repo.msys2.org/mingw/x86_64/mingw-w64-x86_64-crt-git-5.0.0.5002.34a7c1c0-1-any.pkg.tar.xz
  2. http://repo.msys2.org/mingw/x86_64/mingw-w64-x86_64-headers-git-5.0.0.5002.34a7c1c0-1-any.pkg.tar.xz

* Downgrade using the following bash command:
  pacman -U mingw-w64-x86_64-{crt,headers}-git-5.0.0.5002.34a7c1c0-1-any.pkg.tar.xz

More details at: https://github.com/rust-lang/rust/issues/47048

"#;
