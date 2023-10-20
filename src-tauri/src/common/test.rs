#[cfg(test)]
pub mod report_utils {
    use error_stack::{AttachmentKind, FrameKind, Report};

    pub fn messages<E>(report: Report<E>) -> Vec<String> {
        report.frames()
            .map(|frame| {
                if let FrameKind::Attachment(AttachmentKind::Printable(attachment)) = frame.kind() {
                    Some(attachment.to_string())
                } else {
                    None
                }
            })
            .filter_map(|result| result)
            .collect()
    }
}