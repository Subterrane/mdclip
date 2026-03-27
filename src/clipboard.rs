use objc2_app_kit::{NSPasteboard, NSPasteboardTypeRTF};
use objc2_foundation::NSData;

pub fn set_rtf(data: &[u8]) -> Result<(), String> {
    let pasteboard = NSPasteboard::generalPasteboard();
    pasteboard.clearContents();

    let nsdata = NSData::with_bytes(data);
    let ok = pasteboard.setData_forType(Some(&nsdata), unsafe { NSPasteboardTypeRTF });

    if ok {
        Ok(())
    } else {
        Err("failed to write RTF to pasteboard".into())
    }
}
