dnl Configuration for Rust-based PHP extension via ext-php-rs.
dnl Allows phpize to recognize this extension during source compilation (PIE fallback).

PHP_ARG_ENABLE([kreuzberg],
  [whether to enable the kreuzberg extension],
  [AS_HELP_STRING([--enable-kreuzberg],
    [Enable kreuzberg extension support])],
  [yes])

if test "$PHP_KREUZBERG_ENABLED" = "yes"; then
  dnl Recognize the extension directory for phpize/make
  PHP_NEW_EXTENSION(kreuzberg, [], $ext_shared)

  dnl Invoke cargo build to compile the Rust FFI library
  AC_CONFIG_COMMANDS([cargo-build], [
    if test -f "crates/kreuzberg-php/Cargo.toml"; then
      cargo build --release --manifest-path crates/kreuzberg-php/Cargo.toml || exit 1
      cargo_output_dir="crates/kreuzberg-php/target/release"
      ext_soname="kreuzberg"

      dnl Detect output filename based on platform
      if test -f "${cargo_output_dir}/libkreuzberg_php.dylib"; then
        cargo_lib="${cargo_output_dir}/libkreuzberg_php.dylib"
      elif test -f "${cargo_output_dir}/libkreuzberg_php.so"; then
        cargo_lib="${cargo_output_dir}/libkreuzberg_php.so"
      else
        AC_MSG_ERROR([cargo build succeeded but .so/.dylib not found])
      fi

      dnl Copy the compiled library to modules/ directory for phpize to install
      cp "${cargo_lib}" "modules/${ext_soname}.so" || exit 1
    else
      AC_MSG_ERROR([crates/kreuzberg-php/Cargo.toml not found])
    fi
  ], [
    extension_name=kreuzberg
  ])
fi
