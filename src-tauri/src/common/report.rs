use error_stack::{AttachmentKind, FrameKind, Report};

use crate::profile::core::error::ProfileError;

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

#[cfg(test)]
mod tests {
    use crate::profile::core::error::ProfileError;

    use super::*;

    #[test]
    fn should_return_empty_printable_attachments_given_no_printable_attachments_are_attached() {
        let report = Report::new(ProfileError::InvalidProfileNameError);

        assert_eq!(extract_printable_attachments(&report), Vec::<String>::new());
    }

    #[test]
    fn should_return_empty_printable_attachments_given_only_non_printable_attachments_are_attached() {
        let report = Report::new(ProfileError::InvalidProfileNameError)
            .attach("non printable");

        assert_eq!(extract_printable_attachments(&report), Vec::<String>::new());
    }

    #[test]
    fn should_extract_printable_attachments() {
        let report = Report::new(ProfileError::InvalidProfileNameError)
            .attach_printable("printable attachment");

        assert_eq!(extract_printable_attachments(&report), vec!["printable attachment".to_string()]);
    }
}