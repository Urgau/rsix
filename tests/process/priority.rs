use rsix::process::nice;
#[cfg(not(target_os = "redox"))]
use rsix::process::{getpriority_process, setpriority_process, Pid};

#[cfg(not(target_os = "freebsd"))] // FreeBSD's nice(3) doesn't return the old value.
#[test]
fn test_priorities() {
    let old = nice(0).unwrap();

    #[cfg(not(target_os = "redox"))]
    {
        let get_prio = getpriority_process(Pid::NONE).unwrap();
        assert_eq!(get_prio, old);
    }

    let new = nice(1).unwrap();
    assert_eq!(old + 1, new);

    let get = nice(0).unwrap();
    assert_eq!(new, get);

    #[cfg(not(target_os = "redox"))]
    {
        let get_prio = getpriority_process(Pid::NONE).unwrap();
        assert_eq!(get_prio, new);

        setpriority_process(Pid::NONE, get + 1).unwrap();
        let now = getpriority_process(Pid::NONE).unwrap();
        assert_eq!(get + 1, now);
        setpriority_process(Pid::NONE, get + 10000).unwrap();
        let now = getpriority_process(Pid::NONE).unwrap();
        // Linux's max is 19; Darwin's max is 20.
        assert!(now >= 19 && now <= 20);
        // Darwin appears to return `EPERM` on an out of range `nice`.
        if let Ok(again) = nice(1) {
            assert_eq!(now, again);
        }
    }
}

/// FreeBSD's `nice` doesn't return the new nice value, so use a specialized
/// test.
#[cfg(target_os = "freebsd")]
#[test]
fn test_priorities() {
    let start = getpriority_process(Pid::NONE).unwrap();

    let _ = nice(0).unwrap();

    let now = getpriority_process(Pid::NONE).unwrap();
    assert_eq!(start, now);

    let _ = nice(1).unwrap();

    let now = getpriority_process(Pid::NONE).unwrap();
    assert_eq!(start + 1, now);

    setpriority_process(Pid::NONE, start + 2).unwrap();

    let now = getpriority_process(Pid::NONE).unwrap();
    assert_eq!(start + 2, now);

    setpriority_process(Pid::NONE, 10000).unwrap();

    let now = getpriority_process(Pid::NONE).unwrap();
    assert_eq!(now, 20);

    let _ = nice(1).unwrap();

    let now = getpriority_process(Pid::NONE).unwrap();
    assert_eq!(now, 20);
}
