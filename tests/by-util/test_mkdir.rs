use crate::common::util::*;
#[cfg(not(windows))]
use std::os::unix::fs::PermissionsExt;
extern crate libc;
use self::libc::umask;

static TEST_DIR1: &str = "mkdir_test1";
static TEST_DIR2: &str = "mkdir_test2";
static TEST_DIR3: &str = "mkdir_test3";
static TEST_DIR4: &str = "mkdir_test4/mkdir_test4_1";
static TEST_DIR5: &str = "mkdir_test5/mkdir_test5_1";
static TEST_DIR6: &str = "mkdir_test6";
static TEST_FILE7: &str = "mkdir_test7";
static TEST_DIR8: &str = "mkdir_test8";

#[test]
fn test_mkdir_mkdir() {
        new_ucmd!().arg(TEST_DIR1).succeeds();
}

#[test]
fn test_mkdir_verbose() {
    let expected = "mkdir: created directory 'mkdir_test1'\n";
    new_ucmd!()
        .arg(TEST_DIR1)
        .arg("-v")
        .run()
        .stdout_is(expected);
}

#[test]
fn test_mkdir_dup_dir() {
    let scene = TestScenario::new(util_name!());
    scene.ucmd().arg(TEST_DIR2).succeeds();
    scene.ucmd().arg(TEST_DIR2).fails();
}

#[test]
fn test_mkdir_mode() {
    new_ucmd!().arg("-m").arg("755").arg(TEST_DIR3).succeeds();
}

#[test]
fn test_mkdir_parent() {
    let scene = TestScenario::new(util_name!());
    scene.ucmd().arg("-p").arg(TEST_DIR4).succeeds();
    scene.ucmd().arg("-p").arg(TEST_DIR4).succeeds();
    scene.ucmd().arg("--parent").arg(TEST_DIR4).succeeds();
    scene.ucmd().arg("--parents").arg(TEST_DIR4).succeeds();
}

#[test]
fn test_mkdir_no_parent() {
    new_ucmd!().arg(TEST_DIR5).fails();
}

#[test]
fn test_mkdir_dup_dir_parent() {
    let scene = TestScenario::new(util_name!());
    scene.ucmd().arg(TEST_DIR6).succeeds();
    scene.ucmd().arg("-p").arg(TEST_DIR6).succeeds();
}

#[test]
fn test_mkdir_dup_file() {
    let scene = TestScenario::new(util_name!());
    scene.fixtures.touch(TEST_FILE7);
    scene.ucmd().arg(TEST_FILE7).fails();

    // mkdir should fail for a file even if -p is specified.
    scene.ucmd().arg("-p").arg(TEST_FILE7).fails();
}

#[test]
#[cfg(not(windows))]
fn test_symbolic_mode() {
    let (at, mut ucmd) = at_and_ucmd!();

    ucmd.arg("-m").arg("a=rwx").arg(TEST_DIR1).succeeds();
    let perms = at.metadata(TEST_DIR1).permissions().mode();
    assert_eq!(perms, 0o40777);
}

#[test]
#[cfg(not(windows))]
fn test_symbolic_alteration() {
    let (at, mut ucmd) = at_and_ucmd!();

    ucmd.arg("-m").arg("-w").arg(TEST_DIR1).succeeds();
    let perms = at.metadata(TEST_DIR1).permissions().mode();
    assert_eq!(perms, 0o40555);
}

#[test]
#[cfg(not(windows))]
fn test_multi_symbolic() {
    let (at, mut ucmd) = at_and_ucmd!();

    ucmd.arg("-m")
        .arg("u=rwx,g=rx,o=")
        .arg(TEST_DIR1)
        .succeeds();
    let perms = at.metadata(TEST_DIR1).permissions().mode();
    assert_eq!(perms, 0o40750);
}

#[test]
#[cfg(not(windows))]
fn test_umask_compliance() {
    fn test_single_case(umask_set: u32) {
        let (at, mut ucmd) = at_and_ucmd!();

        let original_umask = unsafe { umask(umask_set) };

        ucmd.arg(TEST_DIR8).succeeds();
        let perms = at.metadata(TEST_DIR8).permissions().mode();

        assert_eq!(perms, (!umask_set & 0o0777) + 0o40000); // before compare, add the setguid, uid bits
        unsafe { umask(original_umask); } // set umask back to original
    }

    for i in 0o0..0o777 { // tests all permission combinations
        test_single_case(i);
    }
    
}
