use std::{
    io::Write,
    time::{Duration, Instant},
};

#[inline]
/// Take number of seconds to delay/pause execution.
/// Returns the real duration that was delayed.
/// This function uses a `busy-loop` while checking elapsed time.
///
/// # Panics
/// Panics if passed duration is negativ.
pub fn delay_busy(dur: f64) -> Duration {
    assert!(dur >= 0.0, "{dur}");
    if dur == 0.0 {
        return Duration::from_secs(0);
    }
    let duration = Duration::from_secs_f64(dur);
    let start = Instant::now();
    loop {
        let elapsed = start.elapsed();
        if elapsed >= duration {
            return elapsed;
        }
    }
}

#[inline]
/// Take number of seconds to delay/pause execution.
/// Returns the real duration that was delayed.
/// This function uses `std::thread::sleep`.
///
/// # Panics
/// Panics if passed duration is negativ.
pub fn delay_sleep(dur: f64) -> Duration {
    assert!(dur >= 0.0, "{dur}");
    if dur == 0.0 {
        return Duration::from_secs(0);
    }
    let now = Instant::now();
    std::thread::sleep(Duration::from_secs_f64(dur));
    now.elapsed()
}

#[test]
fn test_delay_busy_precision_10_microseconds() {
    let dur = Duration::from_millis(10);
    let range = dur.as_micros() - 10..dur.as_micros() + 10;
    for _ in 0..100 {
        let now = Instant::now();
        delay_busy(0.005);
        let elapsed = now.elapsed();

        assert!(range.contains(&elapsed.as_micros()));
    }
}

#[test]
fn test_delay_busy_precision() {
    const ITER: usize = 10000;
    let mut samples = Vec::with_capacity(ITER);

    for _ in 0..ITER {
        let now = Instant::now();
        delay_busy(0.005);
        let elapsed = now.elapsed().as_micros();
        samples.push(elapsed);
    }

    let mut out = std::io::BufWriter::new(
        std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open("delay_busy.csv")
            .unwrap(),
    );

    out.write_all(b"samples\n").unwrap();
    for sample in samples {
        out.write_fmt(format_args!("{sample}\n")).unwrap();
    }
}

#[test]
fn test_delay_sleep_precision() {
    const ITER: usize = 10000;
    let mut samples = Vec::with_capacity(ITER);

    for _ in 0..ITER {
        let now = Instant::now();
        delay_sleep(0.005);
        let elapsed = now.elapsed().as_micros();
        samples.push(elapsed);
    }

    let mut out = std::io::BufWriter::new(
        std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open("delay_sleep.csv")
            .unwrap(),
    );

    out.write_all(b"samples\n").unwrap();
    for sample in samples {
        out.write_fmt(format_args!("{sample}\n")).unwrap();
    }
}
