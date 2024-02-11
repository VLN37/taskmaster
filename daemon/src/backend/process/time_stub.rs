static mut TICK: u64 = 0;

#[derive(Debug)]
pub struct Instant {
    seconds: u64,
}

impl Instant {
    pub fn now() -> Instant {
        Instant {
            seconds: unsafe { TICK },
        }
    }

    pub fn elapsed(&self) -> Duration {
        Duration {
            seconds: unsafe { TICK - self.seconds },
        }
    }
}

impl Instant {
    pub fn advance(seconds: u64) {
        unsafe {
            TICK += seconds;
        }
    }

    pub fn advance_from_duration(duration: Duration) {
        unsafe {
            TICK += duration.seconds;
        }
    }
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq)]
pub struct Duration {
    seconds: u64,
}

impl Duration {
    pub(crate) fn from_secs(seconds: u64) -> Duration { Duration { seconds } }
}
