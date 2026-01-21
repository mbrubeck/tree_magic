{
  rustPlatform,
  lib,
  gcc,
  stdenv,
  rust-cbindgen,
}:
let
  cargoToml = fromTOML (builtins.readFile ../Cargo.toml);
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "tree_magic_mini";
  inherit (cargoToml.package) version;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  src = lib.cleanSourceWith {
    src = ../.;
    filter =
      p: type:
      let
        relPath = lib.removePrefix (toString ../. + "/") (toString p);
      in
      lib.any (p: lib.hasPrefix p relPath) [
        "src"
        "tests"
        "magic_db"
        "benches"
        "cbindgen.toml"
        "Cargo.toml"
        "Cargo.lock"
        "tree_magic_mini.pc.in"
      ];
  };

  nativeBuildInputs = [
    rustPlatform.bindgenHook
    rust-cbindgen
  ];

  buildInputs = [
    gcc
  ];

  postInstall = ''
    mkdir -p $out/include  
    cbindgen --config cbindgen.toml --crate tree_magic_mini --output $out/include/tree_magic_mini.h --lang C

    mkdir -p $out/lib  
    cp target/${stdenv.hostPlatform.rust.rustcTarget}/release/libtree_magic_mini${stdenv.hostPlatform.extensions.sharedLibrary} $out/lib

    mkdir -p $out/lib/pkgconfig

    sed \
      -e "s|@PREFIX@|$out|g" \
      -e "s|@INCLUDE@|$out/include|g" \
      -e "s|@LIBDIR@|$out/lib|g" \
      -e "s|@VERSION@|${finalAttrs.version}|g" \
      < ./tree_magic_mini.pc.in > $out/lib/pkgconfig/tree_magic_mini.pc
  '';

  doCheck = false;

  meta = {
    description = "Determines the MIME type of a file by traversing a filetype tree.";
    homepage = "https://github.com/mbrubeck/tree_magic";
    license = lib.licenses.gpl3;
    maintainers = builtins.attrValues { inherit (lib.maintainers) r0chd; };
    platforms = lib.platforms.all;
  };
})
