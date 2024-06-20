use tree_magic_mini as tree_magic;

#[macro_use]
extern crate bencher;
use bencher::Bencher;

/// Image benchmarks
fn image_gif(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("image/gif", include_bytes!("../tests/image/gif")));
}
fn image_png(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("image/png", include_bytes!("../tests/image/png")));
}

fn image_bmp(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("image/bmp", include_bytes!("../tests/image/bmp")));
}

fn image_tiff(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("image/tiff", include_bytes!("../tests/image/tiff")));
}

fn image_x_pcx(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("image/x-pcx", include_bytes!("../tests/image/x-pcx")));
}

fn image_x_portable_bitmap(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("image/x-portable-bitmap", include_bytes!("../tests/image/x-portable-bitmap")));
}

fn image_x_tga(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("image/x-tga", include_bytes!("../tests/image/x-tga")));
}

fn image_xbm(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("image/xbm", include_bytes!("../tests/image/xbm")));
}

/// Archive tests
fn application_zip(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("application/zip", include_bytes!("../tests/application/zip")));
}

fn application_x_7z_compressed(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("application/x-7z-compressed", include_bytes!("../tests/application/x-7z-compressed")));
}

fn application_x_tar(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("application/x-tar", include_bytes!("../tests/application/x-tar")));
}

/// Text tests
fn text_plain(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("text/plain", include_bytes!("../tests/text/plain")));
}

fn text_html(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("text/html", include_bytes!("../tests/text/html")));
}

/// Audio tests
fn audio_flac(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("audio/flac", include_bytes!("../tests/audio/flac")));
}

fn audio_mpeg(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("audio/mpeg", include_bytes!("../tests/audio/mpeg")));
}

fn audio_ogg(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("audio/ogg", include_bytes!("../tests/audio/ogg")));
}

fn audio_opus(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("audio/opus", include_bytes!("../tests/audio/opus")));
}

fn audio_wav(b: &mut Bencher) {
    b.iter(|| tree_magic::match_u8("audio/wav", include_bytes!("../tests/audio/wav")));
}

benchmark_group!(
    bench_match_u8,
    image_gif, image_png, image_bmp, image_tiff, image_x_pcx,
    image_x_portable_bitmap, image_x_tga, image_xbm,
    application_zip, application_x_7z_compressed, application_x_tar,
    text_plain, text_html,
    audio_flac, audio_mpeg, audio_ogg, audio_opus, audio_wav
);
benchmark_main!(bench_match_u8);
