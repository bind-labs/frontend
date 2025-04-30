{
  description = "Bind Frontend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
          config.android_sdk.accept_license = true;
          config.allowUnfree = true;
        };

        # Create a custom Android SDK with specific components
        customAndroidSdk = pkgs.androidenv.composeAndroidPackages {
          toolsVersion = null;
          platformToolsVersion = "34.0.5";
          buildToolsVersions = [ "34.0.0" ];
          # TODO: do we need both 33 and 34?
          platformVersions = [ "33" "34" ];

          includeNDK = true;

          includeEmulator = true;
          includeSystemImages = true;
        };

        # Create emulator using emulateApp function
        emulator = pkgs.androidenv.emulateApp {
          name = "emulator";
          platformVersion = "34";
          abiVersion = "x86_64";
          systemImageType = "google_apis";
        };

        # Helper script to start emulator with additional options
        startEmulatorScript = pkgs.writeScriptBin "start-emulator"
          "${emulator}/bin/run-test-emulator";
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Building dioxus cli
            openssl
            libiconv
            pkg-config

            # Using dioxus cli
            wasm-bindgen-cli_0_2_100
            (pkgs.fenix.fromToolchainFile {
              file = ./rust-toolchain.toml;
              sha256 = "sha256-X/4ZBHO3iW0fOenQ3foEvscgAPJYl2abspaBThDOukI=";
            })

            # Android
            customAndroidSdk.androidsdk
            jdk21
            gradle
            startEmulatorScript
          ];

          shellHook = ''
            export ANDROID_HOME="${customAndroidSdk.androidsdk}/libexec/android-sdk"
            export ANDROID_SDK_ROOT="$ANDROID_HOME"
            export ANDROID_NDK_HOME="$ANDROID_HOME/ndk-bundle"
            export PATH="$ANDROID_HOME/tools:$ANDROID_HOME/tools/bin:$ANDROID_HOME/platform-tools:$PATH"

            # Android emulator flags
            # Not working on Nvidia 570 drivers
            # export NIX_ANDROID_EMULATOR_FLAGS="-gpu host -no-snapshot-save"
            export NIX_ANDROID_EMULATOR_FLAGS="-netdelay none -netspeed full"

            # For Java
            export JAVA_HOME="${pkgs.jdk21}"
          '';
        };
      });
}
