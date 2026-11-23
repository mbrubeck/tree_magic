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

    // 1x1, 24-bit BMP whose info header is `header_size` bytes long
    fn bmp(header_size: u32) -> Vec<u8> {
        let off = 14 + header_size;
        let mut b = b"BM".to_vec();
        b.extend((off + 4).to_le_bytes());
        b.extend([0; 4]);
        b.extend(off.to_le_bytes());
        b.extend(header_size.to_le_bytes());
        b.extend([1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 24, 0]);
        b.resize(off as usize, 0);
        b.extend([0, 0, 0xff, 0]);
        b
    }

    // These rules have value bytes outside their mask ("BMxxxx\0\0" with
    // mask 0xffff00000000ffff), so the mask must apply to both sides.
    #[test]
    fn image_bmp_v4_header() {
        assert_eq!(tree_magic::from_u8(&bmp(108)), convmime!("image/bmp"));
    }
    #[test]
    fn image_bmp_v5_header() {
        assert_eq!(tree_magic::from_u8(&bmp(124)), convmime!("image/bmp"));
    }
    #[test]
    fn image_jp2() {
        let jp2 = b"\0\0\0\x0cjP  \r\n\x87\n\0\0\0\x14ftypjp2 \0\0\0\0jp2 ";
        assert_eq!(tree_magic::from_u8(jp2), convmime!("image/jp2"));
    }
    #[test]
    fn image_vnd_adobe_photoshop() {
        let psd = b"8BPS\0\x01\0\0\0\0\0\0\0\x03\0\0\0\x01\0\0\0\x01\0\x08\0\x03";
        assert_eq!(
            tree_magic::from_u8(psd),
            convmime!("image/vnd.adobe.photoshop")
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
