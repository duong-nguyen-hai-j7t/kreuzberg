```c title="C"
#include "kreuzberg.h"
#include <stdio.h>

int main(void) {
    struct CExtractionResult *result = kreuzberg_extract_file_sync("document.pdf");
    if (!result || !result->success) {
        struct CErrorDetails err = kreuzberg_get_error_details();
        fprintf(stderr, "Error: %s\n", err.message);
        return 1;
    }

    printf("%s\n", result->content);
    printf("MIME type: %s\n", result->mime_type);
    kreuzberg_free_result(result);
    return 0;
}
```
