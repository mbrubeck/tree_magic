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

    // This TGA also matches image/x-win-bitmap, with the same priority. The
    // answer must not depend on the per-process hash seed, so compare the
    // results of several child processes.
    #[test]
    fn same_type_in_every_process() {
        let tga = [
            0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 24, 0, 0x10, 0x20, 0x30,
        ];
        if std::env::var_os("TREE_MAGIC_CHILD").is_some() {
            println!("mime={}", tree_magic::from_u8(&tga));
            return;
        }
        let mut seen = std::collections::BTreeSet::new();
        for _ in 0..16 {
            let out = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "from_u8::same_type_in_every_process"])
                .arg("--nocapture")
                .env("TREE_MAGIC_CHILD", "1")
                .output()
                .unwrap();
            let stdout = String::from_utf8(out.stdout).unwrap();
            for line in stdout.lines() {
                if let Some(mime) = line.strip_prefix("mime=") {
                    seen.insert(mime.to_string());
                }
            }
        }
        assert_eq!(seen.len(), 1, "{seen:?}");
    }
}
