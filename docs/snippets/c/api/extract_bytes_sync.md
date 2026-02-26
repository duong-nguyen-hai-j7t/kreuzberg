```c title="C"
#include "kreuzberg.h"
#include <stdio.h>
#include <string.h>

int main(void) {
    const char *html = "<html><body><h1>Hello</h1><p>World</p></body></html>";
    size_t len = strlen(html);

    struct CExtractionResult *result = kreuzberg_extract_bytes_sync(
        (const uint8_t *)html, len, "text/html");
    if (!result || !result->success) {
        fprintf(stderr, "Error: %s\n", kreuzberg_get_error_details().message);
        return 1;
    }

    printf("%s\n", result->content);
    kreuzberg_free_result(result);
    return 0;
}
```
