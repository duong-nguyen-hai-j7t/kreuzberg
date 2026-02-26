```c title="C"
#include "kreuzberg.h"
#include <stdio.h>

int main(void) {
    struct CExtractionResult *result = kreuzberg_extract_file_sync("missing.pdf");
    if (!result || !result->success) {
        struct CErrorDetails err = kreuzberg_get_error_details();
        fprintf(stderr, "Error [%s]: %s\n",
                kreuzberg_error_code_name(err.error_code),
                err.message);

        if (err.error_code == kreuzberg_error_code_io()) {
            fprintf(stderr, "File not found or unreadable\n");
        } else if (err.error_code == kreuzberg_error_code_unsupported_format()) {
            fprintf(stderr, "Unsupported file format\n");
        }

        if (result) kreuzberg_free_result(result);
        return 1;
    }

    printf("%s\n", result->content);
    kreuzberg_free_result(result);
    return 0;
}
```
