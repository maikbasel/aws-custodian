use crate::profile::core::error::ProfileError;
use error_stack::{AttachmentKind, FrameKind, Report};

pub fn extract_printable_attachments(report: &Report<ProfileError>) -> Vec<String> {
    report
        .frames()
        .filter_map(|frame| {
            if let FrameKind::Attachment(AttachmentKind::Printable(attachment)) = frame.kind() {
                Some(attachment.to_string())
            } else {
                None
            }
        })
        .collect()
}
