// Copyright © 2017–2019 University of Malta

// Copying and distribution of this file, with or without
// modification, are permitted in any medium without royalty provided
// the copyright notice and this notice are preserved. This file is
// offered as-is, without any warranty.

// Notes:
//
//  1. Configure GMP with --enable-fat so that built file is portable.
//
//  2. Configure GMP, MPFR and MPC with: --disable-shared --with-pic
//
//  3. Add symlinks to work around relative path issues in MPFR and MPC.
//     In MPFR: ln -s ../gmp-build
//     In MPC: ln -s ../mpfr-src ../mpfr-build ../gmp-build .
//
//  4. Use relative paths for configure otherwise msys/mingw might be
//     confused with drives and such.

extern crate dirs;

use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Result as IoResult, Write};
#[cfg(unix)]
use std::os::unix::fs as unix_fs;
#[cfg(windows)]
use std::os::windows::fs as windows_fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const GMP_DIR: &'static str = "gmp-6.1.2-c";
const MPFR_DIR: &'static str = "mpfr-4.0.2-c";
const MPC_DIR: &'static str = "mpc-1.1.0-c";

#[derive(Clone, Copy, PartialEq)]
enum Target {
    Mingw,
    Msvc,
    Other,
}

struct Environment {
    rustc: OsString,
    out_dir: PathBuf,
    lib_dir: PathBuf,
    include_dir: PathBuf,
    build_dir: PathBuf,
    cache_dir: Option<PathBuf>,
    jobs: OsString,
    target: Target,
    make_check: bool,
    version_prefix: String,
    version_patch: Option<u64>,
}

#[derive(Clone, Copy, PartialEq)]
enum Workaround47048 {
    Yes,
    No,
}

fn main() {
    let rustc = cargo_env("RUSTC");

    let src_dir = PathBuf::from(cargo_env("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(cargo_env("OUT_DIR"));

    let host = cargo_env("HOST");
    let target = cargo_env("TARGET");
    assert_eq!(host, target, "cross compilation is not supported");

    let (version_prefix, version_patch) = get_version();

    println!("cargo:rerun-if-env-changed=GMP_MPFR_SYS_CACHE");
    let cache_dir = match env::var_os("GMP_MPFR_SYS_CACHE") {
        Some(ref c) if c.is_empty() => None,
        Some(c) => Some(PathBuf::from(c)),
        None => dirs::cache_dir().map(|c| c.join("gmp-mpfr-sys")),
    };
    let cache_dir = cache_dir.map(|cache| cache.join(&version_prefix).join(host));

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
        rustc: rustc,
        out_dir: out_dir.clone(),
        lib_dir: out_dir.join("lib"),
        include_dir: out_dir.join("include"),
        build_dir: out_dir.join("build"),
        cache_dir: cache_dir,
        jobs: cargo_env("NUM_JOBS"),
        target: target,
        make_check: make_check,
        version_prefix: version_prefix,
        version_patch: version_patch,
    };
    env.check_feature("maybe_uninit", TRY_MAYBE_UNINIT, Some("maybe_uninit"));

    // make sure we have target directories
    create_dir_or_panic(&env.lib_dir);
    create_dir_or_panic(&env.include_dir);

    let workaround_47048 = check_for_bug_47048(&env);

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

    let (compile_gmp, compile_mpfr, compile_mpc) = need_compile(&env, &gmp_ah, &mpfr_ah, &mpc_ah);
    if compile_gmp {
        check_for_msvc(&env);
        remove_dir_or_panic(&env.build_dir);
        create_dir_or_panic(&env.build_dir);
        link_dir(&src_dir.join(GMP_DIR), &env.build_dir.join("gmp-src"));
        let (ref a, ref h) = gmp_ah;
        build_gmp(&env, a, h);
    }
    if compile_mpfr {
        link_dir(&src_dir.join(MPFR_DIR), &env.build_dir.join("mpfr-src"));
        let (ref a, ref h) = *mpfr_ah.as_ref().unwrap();
        build_mpfr(&env, a, h);
    }
    if compile_mpc {
        link_dir(&src_dir.join(MPC_DIR), &env.build_dir.join("mpc-src"));
        let (ref a, ref h) = *mpc_ah.as_ref().unwrap();
        build_mpc(&env, a, h);
    }
    if compile_gmp {
        if !there_is_env("CARGO_FEATURE_CNODELETE") {
            remove_dir_or_panic(&env.build_dir);
        }
        if save_cache(&env, &gmp_ah, &mpfr_ah, &mpc_ah) {
            clear_cache_redundancies(&env, mpfr_ah.is_some(), mpc_ah.is_some());
        }
    }
    process_gmp_header(&gmp_ah.1, &out_dir.join("gmp_h.rs"));
    write_link_info(&env, workaround_47048, mpfr_ah.is_some(), mpc_ah.is_some());
}

fn get_version() -> (String, Option<u64>) {
    let version = cargo_env("CARGO_PKG_VERSION")
        .into_string()
        .unwrap_or_else(|e| panic!("version not in utf-8: {:?}", e));
    let last_dot = version
        .rfind('.')
        .unwrap_or_else(|| panic!("version has no dots: {}", version));
    if last_dot == 0 {
        panic!("version starts with dot: {}", version);
    }
    match version[last_dot + 1..].parse::<u64>() {
        Ok(patch) => {
            let mut v = version;
            v.truncate(last_dot);
            (v, Some(patch))
        }
        Err(_) => (version, None),
    }
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
            if save_cache(env, gmp_ah, mpfr_ah, mpc_ah) {
                clear_cache_redundancies(&env, mpfr_ah.is_some(), mpc_ah.is_some());
            }
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
    let req_check = if env.make_check { "ctest" } else { "cnotest" };
    let version_dir = match env.version_patch {
        None => cache_dir.join(&env.version_prefix),
        Some(patch) => cache_dir.join(format!("{}.{}", env.version_prefix, patch)),
    };
    let check_dir = version_dir.join(req_check);
    let mut ok = create_dir(&check_dir).is_ok();
    let (ref a, ref h) = *gmp_ah;
    ok = ok && copy_file(a, &check_dir.join("libgmp.a")).is_ok();
    ok = ok && copy_file(h, &check_dir.join("gmp.h")).is_ok();
    if let Some((ref a, ref h)) = *mpfr_ah {
        ok = ok && copy_file(a, &check_dir.join("libmpfr.a")).is_ok();
        ok = ok && copy_file(h, &check_dir.join("mpfr.h")).is_ok();
    }
    if let Some((ref a, ref h)) = *mpc_ah {
        ok = ok && copy_file(a, &check_dir.join("libmpc.a")).is_ok();
        ok = ok && copy_file(h, &check_dir.join("mpc.h")).is_ok();
    }
    ok
}

fn clear_cache_redundancies(env: &Environment, mpfr: bool, mpc: bool) {
    let cache_dir = match env.cache_dir {
        Some(ref s) => s,
        None => return,
    };
    let cache_dirs = cache_directories(env, &cache_dir)
        .into_iter()
        .rev()
        .filter(|x| match env.version_patch {
            None => x.1.is_none(),
            Some(patch) => x.1.map(|p| p <= patch).unwrap_or(false),
        });
    let checks = ["cnotest", "ctest"];
    let req_checks = if env.make_check {
        &checks
    } else {
        &checks[..1]
    };
    for (version_dir, version_patch) in cache_dirs {
        for req_check in req_checks {
            let check_dir = version_dir.join(req_check);

            // do not clear newly saved cache
            let make_check = *req_check == "ctest";
            if version_patch == env.version_patch && make_check == env.make_check {
                continue;
            }

            // do not clear cache with more libraries  than newly saved cache
            if (!mpc && check_dir.join("libmpc.a").is_file())
                || (!mpfr && check_dir.join("libmpfr.a").is_file())
            {
                continue;
            }

            let _ = remove_dir(&check_dir);
        }
        if !version_dir.join(checks[0]).is_dir() && !version_dir.join(checks[1]).is_dir() {
            let _ = remove_dir(&version_dir);
        }
    }
}

fn cache_directories(env: &Environment, base: &Path) -> Vec<(PathBuf, Option<u64>)> {
    let dir = match fs::read_dir(base) {
        Ok(dir) => dir,
        Err(_) => return Vec::new(),
    };
    let mut vec = Vec::new();
    for entry in dir {
        let path = match entry {
            Ok(e) => e.path(),
            Err(_) => continue,
        };
        if !path.is_dir() {
            continue;
        }
        let patch = {
            let file_name = match path.file_name() {
                Some(name) => name,
                None => continue,
            };
            let path_str = match file_name.to_str() {
                Some(p) => p,
                None => continue,
            };
            if path_str == &env.version_prefix {
                None
            } else if !path_str.starts_with(&env.version_prefix) {
                continue;
            } else if !path_str[env.version_prefix.len()..].starts_with('.') {
                continue;
            } else {
                match path_str[env.version_prefix.len() + 1..].parse::<u64>() {
                    Ok(patch) => Some(patch),
                    Err(_) => continue,
                }
            }
        };
        vec.push((path, patch));
    }
    vec.sort_by_key(|k| k.1);
    vec
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
    let cache_dirs = cache_directories(env, &cache_dir)
        .into_iter()
        .rev()
        .filter(|x| match env.version_patch {
            None => x.1.is_none(),
            Some(patch) => x.1.map(|p| p >= patch).unwrap_or(false),
        });
    let checks = ["cnotest", "ctest"];
    let req_checks = if env.make_check {
        &checks[1..]
    } else {
        &checks
    };
    for (version_dir, _) in cache_dirs {
        for req_check in req_checks {
            let check_dir = version_dir.join(req_check);
            let mut ok = true;
            if let Some((ref a, ref h)) = *mpc_ah {
                ok = ok && copy_file(&check_dir.join("libmpc.a"), a).is_ok();
                ok = ok && copy_file(&check_dir.join("mpc.h"), h).is_ok();
            }
            if let Some((ref a, ref h)) = *mpfr_ah {
                ok = ok && copy_file(&check_dir.join("libmpfr.a"), a).is_ok();
                ok = ok && copy_file(&check_dir.join("mpfr.h"), h).is_ok();
            }
            let (ref a, ref h) = *gmp_ah;
            ok = ok && copy_file(&check_dir.join("libgmp.a"), a).is_ok();
            ok = ok && copy_file(&check_dir.join("gmp.h"), h).is_ok();
            if ok {
                return true;
            }
        }
    }
    false
}

fn should_save_cache(env: &Environment, mpfr: bool, mpc: bool) -> bool {
    let cache_dir = match env.cache_dir {
        Some(ref s) => s,
        None => return false,
    };
    let cache_dirs = cache_directories(env, &cache_dir)
        .into_iter()
        .rev()
        .filter(|x| match env.version_patch {
            None => x.1.is_none(),
            Some(patch) => x.1.map(|p| p >= patch).unwrap_or(false),
        });
    let checks = ["cnotest", "ctest"];
    let req_checks = if env.make_check {
        &checks[1..]
    } else {
        &checks
    };
    for (version_dir, _) in cache_dirs {
        for req_check in req_checks {
            let check_dir = version_dir.join(req_check);
            let mut ok = true;
            if mpc {
                ok = ok && check_dir.join("libmpc.a").is_file();
                ok = ok && check_dir.join("mpc.h").is_file();
            }
            if mpfr {
                ok = ok && check_dir.join("libmpfr.a").is_file();
                ok = ok && check_dir.join("mpfr.h").is_file();
            }
            ok = ok && check_dir.join("libgmp.a").is_file();
            ok = ok && check_dir.join("gmp.h").is_file();
            if ok {
                return false;
            }
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

    let limb_bits = limb_bits.expect("Cannot determine GMP_LIMB_BITS from gmp.h");
    println!("cargo:limb_bits={}", limb_bits);

    let nail_bits = nail_bits.expect("Cannot determine GMP_NAIL_BITS from gmp.h");
    if nail_bits > 0 {
        println!("cargo:rustc-cfg=nails");
    }

    let long_long_limb = long_long_limb.expect("Cannot determine _LONG_LONG_LIMB from gmp.h");
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
        limb_bits, nail_bits, long_long_limb, cc, cflags
    );

    let mut rs = create(out_file);
    write(&mut rs, &content, out_file);
    flush(&mut rs, out_file);
}

fn build_mpfr(env: &Environment, lib: &Path, header: &Path) {
    let build_dir = env.build_dir.join("mpfr-build");
    create_dir_or_panic(&build_dir);
    println!("$ cd {:?}", build_dir);
    link_dir(
        &env.build_dir.join("gmp-build"),
        &build_dir.join("gmp-build"),
    );
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
    // steal link from mpfr-build to save some copying under MinGW,
    // where a symlink is a just a copy (unless in developer mode).
    mv("../mpfr-build/gmp-build", &build_dir);
    link_dir(&env.build_dir.join("mpfr-src"), &build_dir.join("mpfr-src"));
    link_dir(
        &env.build_dir.join("mpfr-build"),
        &build_dir.join("mpfr-build"),
    );
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

fn write_link_info(
    env: &Environment,
    workaround_47048: Workaround47048,
    feature_mpfr: bool,
    feature_mpc: bool,
) {
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
        if workaround_47048 == Workaround47048::Yes {
            println!("cargo:rustc-link-lib=static=workaround_47048");
        }
        add_mingw_libs(feature_mpfr, feature_mpc);
    }
}

impl Environment {
    fn check_feature(&self, name: &str, contents: &str, nightly_features: Option<&str>) {
        let try_dir = self.out_dir.join(format!("try_{}", name));
        let filename = format!("try_{}.rs", name);
        create_dir_or_panic(&try_dir);
        println!("$ cd {:?}", try_dir);

        enum Iteration {
            Stable,
            Unstable,
        }
        for i in &[Iteration::Stable, Iteration::Unstable] {
            let s;
            let file_contents = match *i {
                Iteration::Stable => contents,
                Iteration::Unstable => match nightly_features {
                    Some(features) => {
                        s = format!("#![feature({})]\n{}", features, contents);
                        &s
                    }
                    None => continue,
                },
            };
            create_file_or_panic(&try_dir.join(&filename), file_contents);
            let mut cmd = Command::new(&self.rustc);
            cmd.current_dir(&try_dir)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .args(&[&*filename, "--emit=dep-info,metadata"]);
            println!("$ {:?} >& /dev/null", cmd);
            let status = cmd
                .status()
                .unwrap_or_else(|_| panic!("Unable to execute: {:?}", cmd));
            if status.success() {
                println!("cargo:rustc-cfg={}", name);
                if let Iteration::Unstable = *i {
                    println!("cargo:rustc-cfg=nightly_{}", name);
                }
                break;
            }
        }

        remove_dir_or_panic(&try_dir);
    }
}

fn cargo_env(name: &str) -> OsString {
    env::var_os(name)
        .unwrap_or_else(|| panic!("environment variable not found: {}, please use cargo", name))
}

fn there_is_env(name: &str) -> bool {
    env::var_os(name).is_some()
}

fn check_for_msvc(env: &Environment) {
    if env.target == Target::Msvc {
        panic!("Windows MSVC target is not supported (linking would fail)");
    }
}

fn check_for_bug_47048(env: &Environment) -> Workaround47048 {
    if env.target != Target::Mingw {
        return Workaround47048::No;
    }
    let try_dir = env.build_dir.join("try_47048");
    let rustc = cargo_env("RUSTC");
    remove_dir_or_panic(&try_dir);
    create_dir_or_panic(&try_dir);
    println!("$ cd {:?}", try_dir);
    println!("$ #Check for bug 47048");
    create_file_or_panic(&try_dir.join("say_hi.c"), BUG_47048_SAY_HI_C);
    create_file_or_panic(&try_dir.join("c_main.c"), BUG_47048_C_MAIN_C);
    create_file_or_panic(&try_dir.join("r_main.rs"), BUG_47048_R_MAIN_RS);
    create_file_or_panic(&try_dir.join("workaround.c"), BUG_47048_WORKAROUND_C);
    let mut cmd;

    cmd = Command::new("gcc");
    cmd.current_dir(&try_dir).args(&["-fPIC", "-c", "say_hi.c"]);
    execute(cmd);

    cmd = Command::new("ar");
    cmd.current_dir(&try_dir)
        .args(&["cr", "libsay_hi.a", "say_hi.o"]);
    execute(cmd);

    cmd = Command::new("gcc");
    cmd.current_dir(&try_dir)
        .args(&["c_main.c", "-L.", "-lsay_hi", "-o", "c_main.exe"]);
    execute(cmd);

    // try simple rustc command that should work, so that failure
    // really is the bug being checked for
    cmd = Command::new(&rustc);
    cmd.arg("--version");
    execute(cmd);

    cmd = Command::new(&rustc);
    cmd.current_dir(&try_dir)
        .args(&["r_main.rs", "-L.", "-lsay_hi", "-o", "r_main.exe"])
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    println!(
        "$ {:?} >& /dev/null && echo Bug 47048 not found || echo Working around bug 47048",
        cmd
    );
    let status = cmd
        .status()
        .unwrap_or_else(|_| panic!("Unable to execute: {:?}", cmd));
    let need_workaround = if status.success() {
        println!("Bug 47048 not found");
        Workaround47048::No
    } else {
        println!("Working around bug 47048");

        cmd = Command::new("gcc");
        cmd.current_dir(&try_dir)
            .args(&["-fPIC", "-O2", "-c", "workaround.c"]);
        execute(cmd);

        cmd = Command::new("ar");
        cmd.current_dir(&try_dir)
            .args(&["cr", "libworkaround_47048.a", "workaround.o"]);
        execute(cmd);

        cmd = Command::new(&rustc);
        cmd.current_dir(&try_dir).args(&[
            "r_main.rs",
            "-L.",
            "-lsay_hi",
            "-lworkaround_47048",
            "-o",
            "r_main.exe",
        ]);
        execute(cmd);

        let src = try_dir.join("libworkaround_47048.a");
        let dst = env.lib_dir.join("libworkaround_47048.a");
        copy_file_or_panic(&src, &dst);

        Workaround47048::Yes
    };
    remove_dir_or_panic(&try_dir);
    need_workaround
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
    let version = String::from_utf8(output.stdout).expect("unrecognized rustc version");
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
    remove_dir(dir).unwrap_or_else(|_| panic!("Unable to remove directory: {:?}", dir));
}

fn create_dir(dir: &Path) -> IoResult<()> {
    println!("$ mkdir -p {:?}", dir);
    fs::create_dir_all(dir)
}

fn create_dir_or_panic(dir: &Path) {
    create_dir(dir).unwrap_or_else(|_| panic!("Unable to create directory: {:?}", dir));
}

fn create_file_or_panic(filename: &Path, contents: &str) {
    println!("$ printf '%s' {:?}... > {:?}", &contents[0..10], filename);
    let mut file =
        File::create(filename).unwrap_or_else(|_| panic!("Unable to create file: {:?}", filename));
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

#[cfg(unix)]
fn link_dir(src: &Path, dst: &Path) {
    println!("$ ln -s {:?} {:?}", src, dst);
    unix_fs::symlink(src, dst).unwrap_or_else(|_| {
        panic!("Unable to symlink {:?} -> {:?}", src, dst);
    });
}

#[cfg(windows)]
fn link_dir(src: &Path, dst: &Path) {
    println!("$ ln -s {:?} {:?}", src, dst);
    if windows_fs::symlink_dir(src, dst).is_ok() {
        return;
    }
    println!("symlink_dir: failed to create symbolic link, copying instead");
    let mut c = Command::new("cp");
    c.arg("-R").arg(src).arg(dst);
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
    let file = File::open(name).unwrap_or_else(|_| panic!("Cannot open file: {:?}", name));
    BufReader::new(file)
}

fn create(name: &Path) -> BufWriter<File> {
    let file = File::create(name).unwrap_or_else(|_| panic!("Cannot create file: {:?}", name));
    BufWriter::new(file)
}

fn read_line(reader: &mut BufReader<File>, buf: &mut String, name: &Path) -> usize {
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

const TRY_MAYBE_UNINIT: &'static str = r#"// try_maybe_uninit.rs
use std::mem::MaybeUninit;
fn main() {
    let mut x = MaybeUninit::<u8>::zeroed();
    let _ = x.as_ptr();
    let _ = x.as_mut_ptr();
    let _ = unsafe { x.assume_init() };
    let _ = MaybeUninit::<u8>::uninit();
}
"#;

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

const BUG_47048_WORKAROUND_C: &'static str = r#"/* workaround.c */
#define _CRTBLD
#include <stdio.h>

FILE *__cdecl __acrt_iob_func(unsigned index)
{
    return &(__iob_func()[index]);
}

typedef FILE *__cdecl (*_f__acrt_iob_func)(unsigned index);
_f__acrt_iob_func __MINGW_IMP_SYMBOL(__acrt_iob_func) = __acrt_iob_func;
"#;
