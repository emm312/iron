{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  pname = "rocket";
  version = "0.1";

  src = ./.;

  cargoSha256 = "sha256-nWJ/QeNS1xMee/E58dbDyvnyC0sVUW+FLpp3z08OLN8=";

  meta = with lib; {
    description = "Language";
    homepage = "https://github.com/emm312/rocket";
    platforms = rustPlatform.rust.rustc.meta.platforms;
    license = with licenses; [mit];
  };
}
