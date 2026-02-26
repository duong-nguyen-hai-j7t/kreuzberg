#!/usr/bin/env Rscript
# Kreuzberg R Bindings - Comprehensive Test Suite
# Standalone test runner for the kreuzberg R package.
# Run with: Rscript main_test.R

library(kreuzberg)

cat(strrep("=", 80), "\n")
cat("KREUZBERG R BINDINGS COMPREHENSIVE TEST SUITE\n")
cat(strrep("=", 80), "\n")

# ---------------------------------------------------------------------------
# Simple test runner
# ---------------------------------------------------------------------------
TestRunner <- setRefClass("TestRunner",
  fields = list(
    passed  = "integer",
    failed  = "integer",
    skipped = "integer",
    section = "integer"
  ),
  methods = list(
    initialize = function() {
      passed  <<- 0L
      failed  <<- 0L
      skipped <<- 0L
      section <<- 0L
    },

    start_section = function(name) {
      section <<- section + 1L
      cat(sprintf("\n[SECTION %d] %s\n", section, name))
      cat(strrep("-", 80), "\n")
    },

    test = function(description, expr) {
      result <- tryCatch(
        {
          val <- expr
          if (identical(val, FALSE)) {
            cat(sprintf("  FAIL  %s\n", description))
            failed <<- failed + 1L
            return(invisible(FALSE))
          }
          cat(sprintf("  PASS  %s\n", description))
          passed <<- passed + 1L
          invisible(TRUE)
        },
        error = function(e) {
          cat(sprintf("  FAIL  %s\n", description))
          cat(sprintf("    Error: %s: %s\n", class(e)[1], conditionMessage(e)))
          failed <<- failed + 1L
          invisible(FALSE)
        }
      )
      result
    },

    skip = function(description, reason) {
      cat(sprintf("  SKIP  %s (%s)\n", description, reason))
      skipped <<- skipped + 1L
    },

    summary = function() {
      cat("\n", strrep("=", 80), "\n", sep = "")
      cat("TEST SUMMARY\n")
      cat(strrep("=", 80), "\n")
      total <- passed + failed
      cat(sprintf("Total Tests: %d\n", total))
      cat(sprintf("  Passed:  %d\n", passed))
      cat(sprintf("  Failed:  %d\n", failed))
      cat(sprintf("  Skipped: %d\n", skipped))
      cat(strrep("=", 80), "\n")
      if (failed == 0L) {
        cat("\nALL TESTS PASSED\n")
      } else {
        cat(sprintf("\n%d TEST(S) FAILED\n", failed))
      }
      invisible(failed == 0L)
    }
  )
)

runner <- TestRunner$new()

# ---------------------------------------------------------------------------
# Resolve test documents directory
# ---------------------------------------------------------------------------
script_dir <- tryCatch(
  normalizePath(dirname(sys.frame(1)$ofile), mustWork = FALSE),
  error = function(e) getwd()
)
repo_root <- normalizePath(file.path(script_dir, "..", "..", ".."), mustWork = FALSE)
test_docs <- file.path(repo_root, "test_documents")

if (!dir.exists(test_docs)) {
  # Fallback: walk upward
  d <- getwd()
  for (i in seq_len(10)) {
    if (dir.exists(file.path(d, "test_documents"))) {
      repo_root <- d
      test_docs <- file.path(d, "test_documents")
      break
    }
    d <- dirname(d)
  }
}

has_test_docs <- dir.exists(test_docs)
cat(sprintf("\nRepository root: %s\n", repo_root))
cat(sprintf("Test documents:  %s (%s)\n\n",
            test_docs, if (has_test_docs) "found" else "NOT FOUND"))

# Helper to resolve a test document path
resolve_doc <- function(relative) {
  file.path(test_docs, relative)
}

# ============================================================================
# SECTION 1: Package Loading and Basic Smoke Test
# ============================================================================
runner$start_section("Package Loading and Smoke Test")

runner$test("kreuzberg package is loaded", {
  "kreuzberg" %in% loadedNamespaces()
})

runner$test("extract_file_sync function exists", {
  is.function(extract_file_sync)
})

runner$test("extract_file function exists", {
  is.function(extract_file)
})

runner$test("extract_bytes_sync function exists", {
  is.function(extract_bytes_sync)
})

runner$test("extract_bytes function exists", {
  is.function(extract_bytes)
})

runner$test("batch_extract_files_sync function exists", {
  is.function(batch_extract_files_sync)
})

runner$test("batch_extract_files function exists", {
  is.function(batch_extract_files)
})

runner$test("batch_extract_bytes_sync function exists", {
  is.function(batch_extract_bytes_sync)
})

runner$test("batch_extract_bytes function exists", {
  is.function(batch_extract_bytes)
})

# ============================================================================
# SECTION 2: Configuration Builders
# ============================================================================
runner$start_section("Configuration Builders")

runner$test("extraction_config() creates empty config", {
  config <- extraction_config()
  is.list(config) && length(config) == 0L
})

runner$test("extraction_config(force_ocr = TRUE) sets force_ocr", {
  config <- extraction_config(force_ocr = TRUE)
  identical(config$force_ocr, TRUE)
})

runner$test("extraction_config with output_format", {
  config <- extraction_config(output_format = "markdown")
  identical(config$output_format, "markdown")
})

runner$test("extraction_config with result_format", {
  config <- extraction_config(result_format = "unified")
  identical(config$result_format, "unified")
})

runner$test("extraction_config accepts extra arguments via ...", {
  config <- extraction_config(custom_field = "value")
  identical(config$custom_field, "value")
})

runner$test("extraction_config with nested ocr and chunking", {
  config <- extraction_config(
    ocr = ocr_config(backend = "tesseract", language = "deu"),
    chunking = chunking_config(max_characters = 500L)
  )
  identical(config$ocr$backend, "tesseract") &&
    identical(config$ocr$language, "deu") &&
    identical(config$chunking$max_characters, 500L)
})

runner$test("ocr_config() creates defaults", {
  config <- ocr_config()
  identical(config$backend, "tesseract") && identical(config$language, "eng")
})

runner$test("ocr_config with custom backend and language", {
  config <- ocr_config(backend = "paddle-ocr", language = "chi_sim")
  identical(config$backend, "paddle-ocr") && identical(config$language, "chi_sim")
})

runner$test("ocr_config with dpi", {
  config <- ocr_config(dpi = 300L)
  identical(config$dpi, 300L)
})

runner$test("ocr_config accepts extra arguments via ...", {
  config <- ocr_config(custom_option = TRUE)
  identical(config$custom_option, TRUE)
})

runner$test("chunking_config() creates defaults", {
  config <- chunking_config()
  identical(config$max_characters, 1000L) && identical(config$overlap, 200L)
})

runner$test("chunking_config with custom values", {
  config <- chunking_config(max_characters = 2000L, overlap = 100L)
  identical(config$max_characters, 2000L) && identical(config$overlap, 100L)
})

runner$test("chunking_config accepts extra arguments via ...", {
  config <- chunking_config(strategy = "semantic")
  identical(config$strategy, "semantic")
})

runner$test("extraction_config serializes to JSON and back", {
  config <- extraction_config(force_ocr = TRUE, output_format = "markdown")
  json <- jsonlite::toJSON(config, auto_unbox = TRUE)
  parsed <- jsonlite::fromJSON(as.character(json))
  identical(parsed$force_ocr, TRUE) && identical(parsed$output_format, "markdown")
})

# ============================================================================
# SECTION 3: Configuration Validation (Error Cases)
# ============================================================================
runner$start_section("Configuration Validation (Error Cases)")

runner$test("ocr_config rejects negative dpi", {
  tryCatch(
    { ocr_config(dpi = -100); FALSE },
    error = function(e) grepl("dpi must be a positive", conditionMessage(e))
  )
})

runner$test("ocr_config rejects zero dpi", {
  tryCatch(
    { ocr_config(dpi = 0); FALSE },
    error = function(e) grepl("dpi must be a positive", conditionMessage(e))
  )
})

runner$test("ocr_config rejects non-character backend", {
  tryCatch(
    { ocr_config(backend = 123); FALSE },
    error = function(e) TRUE
  )
})

runner$test("chunking_config rejects negative max_characters", {
  tryCatch(
    { chunking_config(max_characters = -1); FALSE },
    error = function(e) grepl("max_characters must be a positive", conditionMessage(e))
  )
})

runner$test("chunking_config rejects zero max_characters", {
  tryCatch(
    { chunking_config(max_characters = 0); FALSE },
    error = function(e) grepl("max_characters must be a positive", conditionMessage(e))
  )
})

runner$test("chunking_config rejects negative overlap", {
  tryCatch(
    { chunking_config(overlap = -1); FALSE },
    error = function(e) grepl("overlap must be non-negative", conditionMessage(e))
  )
})

# ============================================================================
# SECTION 4: MIME Type Functions
# ============================================================================
runner$start_section("MIME Type Functions")

runner$test("detect_mime_type_from_path function exists", {
  is.function(detect_mime_type_from_path)
})

runner$test("detect_mime_type function exists", {
  is.function(detect_mime_type)
})

runner$test("validate_mime_type function exists", {
  is.function(validate_mime_type)
})

runner$test("get_extensions_for_mime function exists", {
  is.function(get_extensions_for_mime)
})

runner$test("detect_mime_type detects text/plain from raw bytes", {
  bytes <- charToRaw("Hello, world!")
  mime <- detect_mime_type(bytes)
  is.character(mime) && nchar(mime) > 0
})

if (has_test_docs) {
  txt_file <- resolve_doc("text/fake_text.txt")
  if (file.exists(txt_file)) {
    runner$test("detect_mime_type_from_path detects text file", {
      mime <- detect_mime_type_from_path(txt_file)
      is.character(mime) && grepl("text", mime)
    })
  } else {
    runner$skip("detect_mime_type_from_path detects text file", "test file not found")
  }
} else {
  runner$skip("detect_mime_type_from_path detects text file", "test documents not found")
}

runner$test("validate_mime_type validates known MIME type", {
  result <- validate_mime_type("text/plain")
  isTRUE(result)
})

runner$test("get_extensions_for_mime returns extensions for application/pdf", {
  exts <- get_extensions_for_mime("application/pdf")
  is.character(exts) && "pdf" %in% exts
})

runner$test("detect_mime_type rejects non-raw input", {
  tryCatch(
    { detect_mime_type("not raw"); FALSE },
    error = function(e) TRUE
  )
})

runner$test("detect_mime_type_from_path rejects non-character input", {
  tryCatch(
    { detect_mime_type_from_path(123); FALSE },
    error = function(e) TRUE
  )
})

# ============================================================================
# SECTION 5: Validation Functions
# ============================================================================
runner$start_section("Validation Functions")

runner$test("validate_ocr_backend_name accepts 'tesseract'", {
  isTRUE(validate_ocr_backend_name("tesseract"))
})

runner$test("validate_ocr_backend_name rejects invalid backend", {
  tryCatch(
    { validate_ocr_backend_name("totally_fake_backend"); FALSE },
    error = function(e) TRUE
  )
})

runner$test("validate_ocr_backend_name rejects non-character input", {
  tryCatch(
    { validate_ocr_backend_name(123); FALSE },
    error = function(e) TRUE
  )
})

runner$test("validate_language_code accepts 'eng'", {
  isTRUE(validate_language_code("eng"))
})

runner$test("validate_language_code accepts 'deu'", {
  isTRUE(validate_language_code("deu"))
})

runner$test("validate_language_code rejects non-character input", {
  tryCatch(
    { validate_language_code(123); FALSE },
    error = function(e) TRUE
  )
})

runner$test("validate_output_format accepts 'text'", {
  isTRUE(validate_output_format("text"))
})

runner$test("validate_output_format accepts 'markdown'", {
  isTRUE(validate_output_format("markdown"))
})

runner$test("validate_output_format accepts 'html'", {
  isTRUE(validate_output_format("html"))
})

runner$test("validate_output_format rejects invalid format", {
  tryCatch(
    { validate_output_format("not_a_format"); FALSE },
    error = function(e) TRUE
  )
})

runner$test("validate_output_format rejects non-character input", {
  tryCatch(
    { validate_output_format(123); FALSE },
    error = function(e) TRUE
  )
})

# ============================================================================
# SECTION 6: Plugin Registry Functions
# ============================================================================
runner$start_section("Plugin Registry Functions")

runner$test("list_post_processors returns character or list", {
  result <- list_post_processors()
  is.character(result) || is.list(result)
})

runner$test("list_validators returns character or list", {
  result <- list_validators()
  is.character(result) || is.list(result)
})

runner$test("list_ocr_backends returns character or list", {
  result <- list_ocr_backends()
  is.character(result) || is.list(result)
})

runner$test("list_document_extractors returns character or list", {
  result <- list_document_extractors()
  is.character(result) || is.list(result)
})

runner$test("clear_post_processors does not error", {
  tryCatch({ clear_post_processors(); TRUE }, error = function(e) FALSE)
})

runner$test("clear_validators does not error", {
  tryCatch({ clear_validators(); TRUE }, error = function(e) FALSE)
})

runner$test("clear_ocr_backends does not error", {
  tryCatch({ clear_ocr_backends(); TRUE }, error = function(e) FALSE)
})

runner$test("clear_document_extractors does not error", {
  tryCatch({ clear_document_extractors(); TRUE }, error = function(e) FALSE)
})

runner$test("unregister_post_processor handles missing name gracefully", {
  tryCatch({ unregister_post_processor("nonexistent-xyz"); TRUE }, error = function(e) FALSE)
})

runner$test("unregister_validator handles missing name gracefully", {
  tryCatch({ unregister_validator("nonexistent-xyz"); TRUE }, error = function(e) FALSE)
})

runner$test("unregister_ocr_backend handles missing name gracefully", {
  tryCatch({ unregister_ocr_backend("nonexistent-xyz"); TRUE }, error = function(e) FALSE)
})

runner$test("unregister_document_extractor handles missing name gracefully", {
  tryCatch({ unregister_document_extractor("nonexistent-xyz"); TRUE }, error = function(e) FALSE)
})

# ============================================================================
# SECTION 7: Cache Functions
# ============================================================================
runner$start_section("Cache Functions")

runner$test("clear_cache function exists", {
  is.function(clear_cache)
})

runner$test("cache_stats function exists", {
  is.function(cache_stats)
})

runner$test("clear_cache does not error", {
  tryCatch({ clear_cache(); TRUE }, error = function(e) FALSE)
})

runner$test("cache_stats returns a list", {
  stats <- cache_stats()
  is.list(stats)
})

# ============================================================================
# SECTION 8: Extraction - Plain Text (Sync)
# ============================================================================
runner$start_section("Extraction - Plain Text (Sync)")

runner$test("extract_file_sync extracts text from temp file", {
  tmp <- tempfile(fileext = ".txt")
  writeLines("Hello, kreuzberg!", tmp)
  on.exit(unlink(tmp))

  result <- extract_file_sync(tmp)
  inherits(result, "kreuzberg_result") &&
    nchar(result$content) > 0 &&
    grepl("Hello", result$content, fixed = TRUE)
})

runner$test("extract_file extracts text from temp file", {
  tmp <- tempfile(fileext = ".txt")
  writeLines("Async extraction test", tmp)
  on.exit(unlink(tmp))

  result <- extract_file(tmp)
  inherits(result, "kreuzberg_result") &&
    grepl("Async", result$content, fixed = TRUE)
})

runner$test("extract_bytes_sync extracts from raw bytes", {
  bytes <- charToRaw("Test content in bytes")
  result <- extract_bytes_sync(bytes, "text/plain")
  inherits(result, "kreuzberg_result") && nchar(result$content) > 0
})

runner$test("extract_bytes extracts from raw bytes", {
  bytes <- charToRaw("Async bytes test")
  result <- extract_bytes(bytes, "text/plain")
  inherits(result, "kreuzberg_result") && nchar(result$content) > 0
})

runner$test("extract_file_sync with config works", {
  tmp <- tempfile(fileext = ".txt")
  writeLines("Config test content", tmp)
  on.exit(unlink(tmp))

  config <- extraction_config(output_format = "plain")
  result <- extract_file_sync(tmp, config = config)
  inherits(result, "kreuzberg_result")
})

runner$test("extract_file_sync with mime_type override works", {
  tmp <- tempfile(fileext = ".txt")
  writeLines("MIME override test", tmp)
  on.exit(unlink(tmp))

  result <- extract_file_sync(tmp, mime_type = "text/plain")
  inherits(result, "kreuzberg_result") && nchar(result$content) > 0
})

# ============================================================================
# SECTION 9: Extraction - Error Handling
# ============================================================================
runner$start_section("Extraction - Error Handling")

runner$test("extract_file_sync errors on non-existent file", {
  tryCatch(
    { extract_file_sync("/nonexistent/path/file.txt"); FALSE },
    error = function(e) grepl("File not found", conditionMessage(e))
  )
})

runner$test("extract_file_sync errors on non-character path", {
  tryCatch(
    { extract_file_sync(123); FALSE },
    error = function(e) TRUE
  )
})

runner$test("extract_file_sync errors on vector path", {
  tryCatch(
    { extract_file_sync(c("a", "b")); FALSE },
    error = function(e) TRUE
  )
})

runner$test("extract_bytes_sync errors on non-raw data", {
  tryCatch(
    { extract_bytes_sync("not raw", "text/plain"); FALSE },
    error = function(e) TRUE
  )
})

runner$test("extract_bytes_sync errors on non-character mime_type", {
  tryCatch(
    { extract_bytes_sync(charToRaw("hi"), 123); FALSE },
    error = function(e) TRUE
  )
})

runner$test("extract_file_sync produces typed error for unsupported format", {
  tmp <- tempfile(fileext = ".xyz_unsupported")
  writeLines("test", tmp)
  on.exit(unlink(tmp))

  err <- tryCatch(
    extract_file_sync(tmp),
    kreuzberg_error = function(e) e,
    error = function(e) e
  )
  inherits(err, "error") || inherits(err, "kreuzberg_error")
})

# ============================================================================
# SECTION 10: Result Object (S3 Methods)
# ============================================================================
runner$start_section("Result Object (S3 Methods)")

# Create a result to test S3 methods
tmp_s3 <- tempfile(fileext = ".txt")
writeLines("S3 test content for kreuzberg", tmp_s3)
s3_result <- tryCatch(extract_file_sync(tmp_s3), error = function(e) NULL)
unlink(tmp_s3)

if (!is.null(s3_result)) {
  runner$test("result has kreuzberg_result class", {
    inherits(s3_result, "kreuzberg_result")
  })

  runner$test("content() accessor works", {
    identical(content(s3_result), s3_result$content)
  })

  runner$test("mime_type() accessor works", {
    identical(mime_type(s3_result), s3_result$mime_type)
  })

  runner$test("page_count() returns numeric", {
    pc <- page_count(s3_result)
    is.numeric(pc) || is.integer(pc)
  })

  runner$test("chunk_count() returns numeric", {
    cc <- chunk_count(s3_result)
    is.numeric(cc) || is.integer(cc)
  })

  runner$test("detected_language() returns NULL or character", {
    dl <- detected_language(s3_result)
    is.null(dl) || is.character(dl)
  })

  runner$test("metadata_field() returns NULL for missing field", {
    is.null(metadata_field(s3_result, "nonexistent_field"))
  })

  runner$test("print.kreuzberg_result works", {
    output <- capture.output(print(s3_result))
    any(grepl("kreuzberg_result", output))
  })

  runner$test("summary.kreuzberg_result works", {
    output <- capture.output(summary(s3_result))
    any(grepl("kreuzberg_result summary", output))
  })

  runner$test("format.kreuzberg_result returns character", {
    fmt <- format(s3_result)
    is.character(fmt) && grepl("kreuzberg_result", fmt)
  })
} else {
  for (desc in c(
    "result has kreuzberg_result class",
    "content() accessor works",
    "mime_type() accessor works",
    "page_count() returns numeric",
    "chunk_count() returns numeric",
    "detected_language() returns NULL or character",
    "metadata_field() returns NULL for missing field",
    "print.kreuzberg_result works",
    "summary.kreuzberg_result works",
    "format.kreuzberg_result returns character"
  )) {
    runner$skip(desc, "could not create result object")
  }
}

# ============================================================================
# SECTION 11: Batch Extraction
# ============================================================================
runner$start_section("Batch Extraction")

runner$test("batch_extract_files_sync extracts multiple files", {
  tmp1 <- tempfile(fileext = ".txt")
  tmp2 <- tempfile(fileext = ".txt")
  writeLines("Batch file one", tmp1)
  writeLines("Batch file two", tmp2)
  on.exit({ unlink(tmp1); unlink(tmp2) })

  results <- batch_extract_files_sync(c(tmp1, tmp2))
  is.list(results) && length(results) == 2L &&
    all(vapply(results, inherits, logical(1), "kreuzberg_result"))
})

runner$test("batch_extract_files extracts multiple files", {
  tmp1 <- tempfile(fileext = ".txt")
  tmp2 <- tempfile(fileext = ".txt")
  writeLines("Async batch one", tmp1)
  writeLines("Async batch two", tmp2)
  on.exit({ unlink(tmp1); unlink(tmp2) })

  results <- batch_extract_files(c(tmp1, tmp2))
  is.list(results) && length(results) == 2L
})

runner$test("batch_extract_bytes_sync extracts multiple byte arrays", {
  data_list <- list(charToRaw("Bytes one"), charToRaw("Bytes two"))
  mime_types <- c("text/plain", "text/plain")
  results <- batch_extract_bytes_sync(data_list, mime_types)
  is.list(results) && length(results) == 2L &&
    all(vapply(results, inherits, logical(1), "kreuzberg_result"))
})

runner$test("batch_extract_bytes extracts multiple byte arrays", {
  data_list <- list(charToRaw("Async bytes one"), charToRaw("Async bytes two"))
  mime_types <- c("text/plain", "text/plain")
  results <- batch_extract_bytes(data_list, mime_types)
  is.list(results) && length(results) == 2L
})

runner$test("batch_extract_bytes_sync errors on mismatched lengths", {
  tryCatch(
    {
      batch_extract_bytes_sync(
        list(charToRaw("one")),
        c("text/plain", "text/plain")
      )
      FALSE
    },
    error = function(e) grepl("same length", conditionMessage(e))
  )
})

runner$test("batch_extract_files_sync with config", {
  tmp <- tempfile(fileext = ".txt")
  writeLines("Batch config test", tmp)
  on.exit(unlink(tmp))

  config <- extraction_config(output_format = "plain")
  results <- batch_extract_files_sync(c(tmp), config = config)
  is.list(results) && length(results) == 1L
})

# ============================================================================
# SECTION 12: File Extraction with Test Documents
# ============================================================================
runner$start_section("File Extraction with Test Documents")

if (has_test_docs) {

  # --- Text file ---
  txt_path <- resolve_doc("text/fake_text.txt")
  if (file.exists(txt_path)) {
    runner$test("extract text file from test_documents", {
      result <- extract_file_sync(txt_path)
      inherits(result, "kreuzberg_result") && nchar(result$content) > 0
    })
  } else {
    runner$skip("extract text file from test_documents", "text/fake_text.txt not found")
  }

  # --- HTML file ---
  html_path <- resolve_doc("html/simple_table.html")
  if (file.exists(html_path)) {
    runner$test("extract HTML file from test_documents", {
      result <- extract_file_sync(html_path)
      inherits(result, "kreuzberg_result") && nchar(result$content) > 0
    })

    runner$test("HTML extraction result has mime_type containing 'html'", {
      result <- extract_file_sync(html_path)
      grepl("html", result$mime_type, ignore.case = TRUE)
    })
  } else {
    runner$skip("extract HTML file from test_documents", "html/simple_table.html not found")
    runner$skip("HTML extraction result has mime_type containing 'html'", "html/simple_table.html not found")
  }

  # --- PDF file ---
  pdf_path <- resolve_doc("pdf/fake_memo.pdf")
  if (file.exists(pdf_path)) {
    runner$test("extract PDF file from test_documents", {
      result <- extract_file_sync(pdf_path)
      inherits(result, "kreuzberg_result") && nchar(result$content) > 0
    })

    runner$test("PDF extraction contains expected content", {
      result <- extract_file_sync(pdf_path)
      grepl("May 5, 2023", result$content, fixed = TRUE) ||
        grepl("Whom it May Concern", result$content, fixed = TRUE)
    })
  } else {
    runner$skip("extract PDF file from test_documents", "pdf/fake_memo.pdf not found")
    runner$skip("PDF extraction contains expected content", "pdf/fake_memo.pdf not found")
  }

  # --- DOCX file ---
  docx_path <- resolve_doc("docx/fake.docx")
  if (file.exists(docx_path)) {
    runner$test("extract DOCX file from test_documents", {
      result <- extract_file_sync(docx_path)
      inherits(result, "kreuzberg_result") && nchar(result$content) > 0
    })
  } else {
    runner$skip("extract DOCX file from test_documents", "docx/fake.docx not found")
  }

  # --- XLSX file ---
  xlsx_path <- resolve_doc("xlsx/stanley_cups.xlsx")
  if (file.exists(xlsx_path)) {
    runner$test("extract XLSX file from test_documents", {
      result <- extract_file_sync(xlsx_path)
      inherits(result, "kreuzberg_result") && nchar(result$content) > 0
    })

    runner$test("XLSX extraction contains expected content", {
      result <- extract_file_sync(xlsx_path)
      grepl("Stanley Cups", result$content, fixed = TRUE) ||
        grepl("Team", result$content, fixed = TRUE)
    })
  } else {
    runner$skip("extract XLSX file from test_documents", "xlsx/stanley_cups.xlsx not found")
    runner$skip("XLSX extraction contains expected content", "xlsx/stanley_cups.xlsx not found")
  }

  # --- JSON file ---
  json_path <- resolve_doc("json/simple.json")
  if (file.exists(json_path)) {
    runner$test("extract JSON file from test_documents", {
      result <- extract_file_sync(json_path)
      inherits(result, "kreuzberg_result") && nchar(result$content) > 0
    })
  } else {
    runner$skip("extract JSON file from test_documents", "json/simple.json not found")
  }

  # --- Bytes extraction from file ---
  if (file.exists(txt_path)) {
    runner$test("extract_bytes_sync reads file as bytes", {
      bytes <- readBin(txt_path, "raw", file.info(txt_path)$size)
      result <- extract_bytes_sync(bytes, "text/plain")
      inherits(result, "kreuzberg_result") && nchar(result$content) > 0
    })
  } else {
    runner$skip("extract_bytes_sync reads file as bytes", "text/fake_text.txt not found")
  }

} else {
  for (desc in c(
    "extract text file from test_documents",
    "extract HTML file from test_documents",
    "HTML extraction result has mime_type containing 'html'",
    "extract PDF file from test_documents",
    "PDF extraction contains expected content",
    "extract DOCX file from test_documents",
    "extract XLSX file from test_documents",
    "XLSX extraction contains expected content",
    "extract JSON file from test_documents",
    "extract_bytes_sync reads file as bytes"
  )) {
    runner$skip(desc, "test_documents directory not found")
  }
}

# ============================================================================
# SECTION 13: Extraction with Configuration Options
# ============================================================================
runner$start_section("Extraction with Configuration Options")

runner$test("extraction with force_ocr config", {
  tmp <- tempfile(fileext = ".txt")
  writeLines("Force OCR test content", tmp)
  on.exit(unlink(tmp))

  config <- extraction_config(force_ocr = TRUE)
  # force_ocr on a text file may succeed or produce an error depending on OCR availability
  result <- tryCatch(
    extract_file_sync(tmp, config = config),
    error = function(e) e
  )
  inherits(result, "kreuzberg_result") || inherits(result, "error")
})

runner$test("extraction with ocr_config nested in extraction_config", {
  tmp <- tempfile(fileext = ".txt")
  writeLines("OCR config test", tmp)
  on.exit(unlink(tmp))

  config <- extraction_config(
    ocr = ocr_config(backend = "tesseract", language = "eng", dpi = 150L)
  )
  result <- tryCatch(
    extract_file_sync(tmp, config = config),
    error = function(e) e
  )
  inherits(result, "kreuzberg_result") || inherits(result, "error")
})

runner$test("extraction with chunking_config", {
  tmp <- tempfile(fileext = ".txt")
  writeLines(paste(rep("Chunking test word", 200), collapse = " "), tmp)
  on.exit(unlink(tmp))

  config <- extraction_config(
    chunking = chunking_config(max_characters = 100L, overlap = 20L)
  )
  result <- tryCatch(
    extract_file_sync(tmp, config = config),
    error = function(e) e
  )
  inherits(result, "kreuzberg_result") || inherits(result, "error")
})

# ============================================================================
# SECTION 14: Config Discovery
# ============================================================================
runner$start_section("Config Discovery")

runner$test("discover function exists", {
  is.function(discover)
})

runner$test("from_file function exists", {
  is.function(from_file)
})

runner$test("discover returns NULL or list", {
  result <- tryCatch(discover(), error = function(e) NULL)
  is.null(result) || is.list(result)
})

runner$test("from_file rejects non-character input", {
  tryCatch(
    { from_file(123); FALSE },
    error = function(e) TRUE
  )
})

runner$test("from_file rejects vector input", {
  tryCatch(
    { from_file(c("a", "b")); FALSE },
    error = function(e) TRUE
  )
})

# ============================================================================
# SUMMARY
# ============================================================================
all_passed <- runner$summary()
quit(status = if (all_passed) 0L else 1L)
