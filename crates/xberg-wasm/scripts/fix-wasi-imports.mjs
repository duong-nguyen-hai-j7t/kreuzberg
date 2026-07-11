#!/usr/bin/env node
/**
 * Post-build patch for the wasm-pack `nodejs` target output.
 *
 * Problem: the `ocr-wasm` feature links `xberg-tesseract` (Tesseract +
 * Leptonica cross-compiled with the WASI SDK for wasm32-wasi) into the
 * wasm32-unknown-unknown module produced for this crate (see
 * `.cargo/config.toml`'s `--allow-multiple-definition` for the target). The
 * resulting `xberg_wasm_bg.wasm` therefore imports the `env` (Leptonica's
 * `mkstemp`/`system`) and `wasi_snapshot_preview1` (standard WASI preview1
 * syscalls) modules, and wasm-bindgen's `--target nodejs` glue emits:
 *
 *   const import1 = require("env");
 *   const import3 = require("wasi_snapshot_preview1");
 *   ...
 *
 * Neither "env" nor "wasi_snapshot_preview1" is a real Node built-in, so
 * `require('@xberg-io/xberg-wasm')` throws `Cannot find module 'env'`
 * immediately, before the WASM module is even instantiated. Separately, the
 * generated `__wbg_get_imports()` return object has one duplicate "env" /
 * "wasi_snapshot_preview1" key per imported *symbol* (JS object literals keep
 * only the last value per key), which is harmless once every occurrence
 * points at the same stub object.
 *
 * Fix: strip the `require(...)` statements, replace every reference to the
 * per-symbol import variables with two shared stub objects, and deduplicate
 * the resulting object-literal keys. All OCR/table-detection work happens on
 * image bytes already resident in WASM linear memory, so the stubs never
 * need a real filesystem or shell: they report "no filesystem here"
 * (WASI errno values) for path-based syscalls and answer `fd_read`/`fd_write`/
 * `clock_time_get`/`environ_*` with harmless real values so Tesseract/
 * Leptonica's initialization and buffered stdio calls succeed.
 *
 * Runs against `pkg/nodejs/xberg_wasm.js` (the only wasm-pack target that
 * uses CommonJS `require()` for its WASI/env imports; `web`/`bundler`/`deno`
 * targets use ESM `import` and are handled by wasm-bindgen without invoking
 * Node's module resolver, so they are unaffected by this specific failure
 * mode). Idempotent: running twice is a no-op.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const jsFile = path.join(__dirname, "..", "pkg", "nodejs", "xberg_wasm.js");

if (!fs.existsSync(jsFile)) {
  console.log(`[fix-wasi-imports] ${jsFile} not found, skipping.`);
  process.exit(0);
}

let content = fs.readFileSync(jsFile, "utf-8");
const originalContent = content;

if (content.includes("__wasi_stubs__")) {
  console.log("[fix-wasi-imports] already patched, skipping.");
  process.exit(0);
}

const hasCjsImports = content.includes('require("env")') || content.includes('require("wasi_snapshot_preview1")');
if (!hasCjsImports) {
  console.log("[fix-wasi-imports] no env/wasi_snapshot_preview1 require() calls found, skipping.");
  process.exit(0);
}

console.log('[fix-wasi-imports] patching require("env") / require("wasi_snapshot_preview1") in xberg_wasm.js...');

const cjsPattern = /^const (import\d+) = require\("(env|wasi_snapshot_preview1)"\);?$/gm;
const envImports = [];
const wasiImports = [];
for (const match of content.matchAll(cjsPattern)) {
  const [, varName, moduleName] = match;
  (moduleName === "env" ? envImports : wasiImports).push(varName);
}

console.log(`[fix-wasi-imports] found ${envImports.length} env import(s), ${wasiImports.length} wasi import(s).`);

content = content.replace(/^const import\d+ = require\("(env|wasi_snapshot_preview1)"\);?\n/gm, "");

const stubCode = `// __wasi_stubs__ - inline replacements for the unresolvable "env" /
// "wasi_snapshot_preview1" require() targets. See fix-wasi-imports.mjs for
// the full rationale; this block is injected by that script.
let __wasi_mem_ref = { memory: null };
function __wasi_view() {
    if (!__wasi_mem_ref.memory) return null;
    return new DataView(__wasi_mem_ref.memory.buffer);
}

// Leptonica's system()/mkstemp() shell-exec and temp-file helpers are never
// reached on the in-memory OCR path. The Proxy catch-all covers any other
// unresolved libc/env symbol the WASI-SDK link left dangling.
const __env_stubs__ = new Proxy({
    system: () => -1,
    mkstemp: () => -1,
}, {
    get(target, prop) {
        if (prop in target) return target[prop];
        return () => {};
    }
});

// WASI preview1 stubs. Functions with output pointers write real values into
// WASM memory; everything filesystem-shaped reports absence (EBADF/ENOSYS)
// since this embedding never preopens a directory.
const __wasi_stubs__ = {
    fd_close: () => 0,
    fd_read: (fd, iovs_ptr, iovs_len, nread_ptr) => {
        const v = __wasi_view();
        if (v && nread_ptr) v.setUint32(nread_ptr, 0, true);
        return 0;
    },
    fd_write: (fd, iovs_ptr, iovs_len, nwritten_ptr) => {
        const v = __wasi_view();
        if (v) {
            let total = 0;
            for (let i = 0; i < iovs_len; i++) {
                total += v.getUint32(iovs_ptr + i * 8 + 4, true);
            }
            if (nwritten_ptr) v.setUint32(nwritten_ptr, total, true);
        }
        return 0;
    },
    fd_seek: (fd, offset_lo, offset_hi, whence, newoffset_ptr) => {
        const v = __wasi_view();
        if (v && newoffset_ptr) {
            v.setUint32(newoffset_ptr, 0, true);
            v.setUint32(newoffset_ptr + 4, 0, true);
        }
        return 0;
    },
    fd_fdstat_get: (fd, fdstat_ptr) => {
        const v = __wasi_view();
        if (v && fdstat_ptr) {
            v.setUint8(fdstat_ptr, fd <= 2 ? 2 : 4);
            v.setUint16(fdstat_ptr + 2, 0, true);
            v.setBigUint64(fdstat_ptr + 8, 0xffffffffffffffffn, true);
            v.setBigUint64(fdstat_ptr + 16, 0xffffffffffffffffn, true);
        }
        return 0;
    },
    fd_fdstat_set_flags: (fd, flags) => 0,
    fd_prestat_get: (fd, prestat_ptr) => 8, // EBADF - no preopened dirs
    fd_prestat_dir_name: (fd, path_ptr, path_len) => 8, // EBADF
    environ_get: (environ_ptr, environ_buf_ptr) => 0,
    environ_sizes_get: (count_ptr, buf_size_ptr) => {
        const v = __wasi_view();
        if (v) {
            if (count_ptr) v.setUint32(count_ptr, 0, true);
            if (buf_size_ptr) v.setUint32(buf_size_ptr, 0, true);
        }
        return 0;
    },
    clock_time_get: (clock_id, precision, time_ptr) => {
        const v = __wasi_view();
        if (v && time_ptr) {
            v.setBigUint64(time_ptr, BigInt(Math.floor(Date.now() * 1e6)), true);
        }
        return 0;
    },
    path_create_directory: (fd, path_ptr, path_len) => 63, // ENOSYS
    path_filestat_get: (fd, flags, path_ptr, path_len, filestat_ptr) => 63,
    path_open: (dirfd, dirflags, path_ptr, path_len, oflags, fs_rights_base_lo, fs_rights_base_hi, fs_rights_inheriting_lo, fs_rights_inheriting_hi, fdflags, fd_ptr) => 63,
    path_remove_directory: (fd, path_ptr, path_len) => 63,
    path_unlink_file: (fd, path_ptr, path_len) => 63,
    proc_exit: (code) => { throw new Error(\`WASM proc_exit called with code \${code}\`); },
    sched_yield: () => 0,
};

`;

const getImportsIdx = content.indexOf("function __wbg_get_imports()");
if (getImportsIdx === -1) {
  console.error("[fix-wasi-imports] ERROR: could not find __wbg_get_imports() in xberg_wasm.js");
  process.exit(1);
}
content = content.slice(0, getImportsIdx) + stubCode + content.slice(getImportsIdx);

for (const varName of envImports) content = content.replaceAll(varName, "__env_stubs__");
for (const varName of wasiImports) content = content.replaceAll(varName, "__wasi_stubs__");

// Deduplicate the "env" / "wasi_snapshot_preview1" keys in the import object
// literal returned by __wbg_get_imports() -- every occurrence now points at
// the same stub object, so only one key per module needs to survive.
const returnBlockStart = content.indexOf('"./xberg_wasm_bg.js": import0,');
if (returnBlockStart !== -1) {
  const returnBlockEnd = content.indexOf("};", returnBlockStart);
  if (returnBlockEnd !== -1) {
    const returnBlock = content.slice(returnBlockStart, returnBlockEnd);
    let seenEnv = false;
    let seenWasi = false;
    const dedupedLines = returnBlock.split("\n").filter((line) => {
      const trimmed = line.trim();
      if (trimmed.startsWith('"env"')) {
        if (seenEnv) return false;
        seenEnv = true;
      }
      if (trimmed.startsWith('"wasi_snapshot_preview1"')) {
        if (seenWasi) return false;
        seenWasi = true;
      }
      return true;
    });
    content = content.slice(0, returnBlockStart) + dedupedLines.join("\n") + content.slice(returnBlockEnd);
  }
} else {
  console.log("[fix-wasi-imports] WARNING: could not find the import-object return block to dedupe.");
}

// Give the stubs access to WASM linear memory once the instance exists.
// wasm-bindgen's nodejs target instantiates synchronously via
// `let wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());`.
const instantiatePattern = /^(let wasmInstance = new WebAssembly\.Instance\(.*\);)$/m;
if (instantiatePattern.test(content)) {
  content = content.replace(
    instantiatePattern,
    "$1\n// Populate WASI memory reference for stubs that write output values\n__wasi_mem_ref.memory = wasmInstance.exports.memory;",
  );
} else {
  console.log(
    "[fix-wasi-imports] WARNING: could not find the WebAssembly.Instance instantiation to inject the memory reference.",
  );
}

if (content === originalContent) {
  console.log("[fix-wasi-imports] no changes needed.");
} else {
  fs.writeFileSync(jsFile, content);
  console.log(
    `[fix-wasi-imports] replaced ${envImports.length + wasiImports.length} external imports with inline stubs. Done.`,
  );
}
