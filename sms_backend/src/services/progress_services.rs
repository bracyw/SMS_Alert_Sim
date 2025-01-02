use std::sync::Arc;

use crate::{models::progress_models::SendProgressData, structures::progress_monitor::ProgressMonitor};

pub fn get_progress_data(pm: Arc<ProgressMonitor>) -> SendProgressData {
    SendProgressData {
        msgs_sent: pm.get_msgs_sent(),
        msgs_failed: pm.get_msgs_failed(),
        avg_wait_time: pm.get_avg_wait_time()
    }
}