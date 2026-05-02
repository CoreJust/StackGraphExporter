use std::sync::mpsc::Sender;

pub enum ProgressMsg {
    SetTotal {
        branch: usize,
        total: usize,
    },
    Tick {
        branch: usize,
        done: usize,
        label: &'static str,
    },
    Finished,
}

pub struct BranchProgress {
    tx: Sender<ProgressMsg>,
    branch: usize,
    last_sent: usize,
}

impl BranchProgress {
    pub fn new(tx: Sender<ProgressMsg>, branch: usize) -> Self {
        Self {
            tx,
            branch,
            last_sent: 0,
        }
    }

    #[inline]
    pub fn set_total(&self, total: usize) {
        let _ = self.tx.send(ProgressMsg::SetTotal {
            branch: self.branch,
            total,
        });
    }

    #[inline]
    pub fn tick(&mut self, label: &'static str, done: usize) {
        // throttle
        if done - self.last_sent < 256 {
            return;
        }
        self.last_sent = done;

        let _ = self.tx.send(ProgressMsg::Tick {
            branch: self.branch,
            done,
            label,
        });
    }

    #[inline]
    pub fn finish(&mut self, label: &'static str, done: usize) {
        let _ = self.tx.send(ProgressMsg::Tick {
            branch: self.branch,
            done,
            label,
        });
        let _ = self.tx.send(ProgressMsg::Finished);
    }
}
