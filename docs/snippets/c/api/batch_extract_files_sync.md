```c title="C"
#include "kreuzberg.h"
#include <stdio.h>

int main(void) {
    const char *files[] = {"doc1.pdf", "doc2.docx", "doc3.txt"};
    uintptr_t count = 3;

    struct CBatchResult *batch = kreuzberg_batch_extract_files_sync(files, count, NULL);
    if (!batch) {
        fprintf(stderr, "Batch error: %s\n", kreuzberg_get_error_details().message);
        return 1;
    }

    for (uintptr_t i = 0; i < batch->count; i++) {
        struct CExtractionResult *r = batch->results[i];
        if (r && r->success) {
            printf("--- %s ---\n%s\n", files[i], r->content);
        }
    }

    kreuzberg_free_batch_result(batch);
    return 0;
}
```
