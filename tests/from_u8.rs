mod from_u8 {
    use tree_magic_mini as tree_magic;

    macro_rules! convmime {
        ($x:expr) => {
            $x.to_string()
        };
    }

    ///Image tests
    #[test]
    fn image_heic() {
        assert_eq!(
            tree_magic::from_u8(include_bytes!("image/heic")),
            convmime!("image/heif")
        );
    }
    #[test]
    fn image_gif() {
        assert_eq!(
            tree_magic::from_u8(include_bytes!("image/gif")),
            convmime!("image/gif")
        );
    }
    #[test]
    fn image_png() {
        assert_eq!(
            tree_magic::from_u8(include_bytes!("image/png")),
            convmime!("image/png")
        );
    }
    #[test]
    // GNU file reports image/x-ms-bmp
    fn image_bmp() {
        assert_eq!(
            tree_magic::from_u8(include_bytes!("image/bmp")),
            convmime!("image/bmp")
        );
    }
    #[test]
    fn image_tiff() {
        assert_eq!(
            tree_magic::from_u8(include_bytes!("image/tiff")),
            convmime!("image/tiff")
        );
    }
    #[test]
    fn image_x_portable_bitmap() {
        assert_eq!(
            tree_magic::from_u8(include_bytes!("image/x-portable-bitmap")),
            convmime!("image/x-portable-bitmap")
        );
    }
    #[test]
    fn image_x_pcx() {
        assert_eq!(
            tree_magic::from_u8(include_bytes!("image/x-pcx")),
            convmime!("image/vnd.zbrush.pcx")
        );
    }
    #[test]
    fn image_x_tga() {
        assert_eq!(
            tree_magic::from_u8(include_bytes!("image/x-tga")),
            convmime!("image/x-tga")
        );
    }

    /// Archive tests
    #[test]
    fn application_tar() {
        assert_eq!(
            tree_magic::from_u8(include_bytes!("application/x-tar")),
            convmime!("application/x-tar")
        );
    }
    #[test]
    fn application_x_7z() {
        assert_eq!(
            tree_magic::from_u8(include_bytes!("application/x-7z-compressed")),
            convmime!("application/x-7z-compressed")
        );
    }
    #[test]
    fn application_zip() {
        assert_eq!(
            tree_magic::from_u8(include_bytes!("application/zip")),
            convmime!("application/zip")
        );
    }

    /// Text tests
    #[test]
    fn text_html() {
        assert_eq!(
            tree_magic::from_u8(include_bytes!("text/html")),
            convmime!("text/html")
        );
    }

    #[test]
    fn text_html_doctype_lowercase() {
        let html = b"<!DOCTYPE html><html><body></body></html>";
        assert_eq!(tree_magic::from_u8(html), convmime!("text/html"));
    }

    #[test]
    fn text_html_doctype_uppercase() {
        let html = b"<!DOCTYPE HTML><html><body></body></html>";
        assert_eq!(tree_magic::from_u8(html), convmime!("text/html"));
    }

    #[test]
    fn text_html_doctype_only() {
        let html = b"<!DOCTYPE html>";
        assert_eq!(tree_magic::from_u8(html), convmime!("text/html"));
    }

    #[test]
    fn text_html_doctype_with_newline() {
        let html =
            b"<!DOCTYPE html>\n<html>\n<head><title>Test</title></head>\n<body></body>\n</html>";
        assert_eq!(tree_magic::from_u8(html), convmime!("text/html"));
    }

    #[test]
    fn text_plain() {
        assert_eq!(
            tree_magic::from_u8(include_bytes!("text/plain")),
            convmime!("text/plain")
        );
    }

    // XML with a UTF-8 byte order mark before the declaration. The rule for
    // this is only in the bundled database, not in the system one.
    #[cfg(feature = "with-gpl-data")]
    #[test]
    fn application_xml_utf8_bom() {
        let xml = b"\xEF\xBB\xBF<?xml version=\"1.0\"?><a/>";
        assert_eq!(tree_magic::from_u8(xml), convmime!("application/xml"));
    }

    #[cfg(feature = "with-gpl-data")]
    #[test]
    fn image_svg_utf8_bom() {
        let svg = b"\xEF\xBB\xBF<?xml version=\"1.0\"?><svg xmlns=\"http://www.w3.org/2000/svg\"/>";
        assert_eq!(tree_magic::from_u8(svg), convmime!("image/svg+xml"));
    }

    // Audio tests
    #[test]
    fn audio_flac() {
        assert_eq!(
            tree_magic::from_u8(include_bytes!("audio/flac")),
            convmime!("audio/flac")
        );
    }

    #[test]
    fn audio_mpeg() {
        assert_eq!(
            tree_magic::from_u8(include_bytes!("audio/mpeg")),
            convmime!("audio/mpeg")
        );
    }
}
