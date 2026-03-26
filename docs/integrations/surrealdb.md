# SurrealDB

Bridges [Kreuzberg](https://github.com/kreuzberg-dev/kreuzberg) extraction into [SurrealDB](https://surrealdb.com/) — schema generation, content deduplication, chunk storage, and index configuration.

[![PyPI](https://img.shields.io/pypi/v/kreuzberg-surrealdb)](https://pypi.org/project/kreuzberg-surrealdb/)
[![Python](https://img.shields.io/pypi/pyversions/kreuzberg-surrealdb)](https://pypi.org/project/kreuzberg-surrealdb/)
[![License](https://img.shields.io/pypi/l/kreuzberg-surrealdb)](https://github.com/kreuzberg-dev/kreuzberg-surrealdb/blob/main/LICENSE)

## Installation

```bash
pip install kreuzberg-surrealdb
```

Requires Python 3.10+ and a running SurrealDB instance:

```bash
docker run --rm -p 8000:8000 surrealdb/surrealdb:latest start --allow-all --user root --pass root
```

## Quick Start

```python
from kreuzberg_surrealdb import DocumentPipeline

pipeline = DocumentPipeline(db=db, embed=True, embedding_model="balanced")
await pipeline.setup_schema()
await pipeline.ingest_directory("./papers", glob="**/*.pdf")
```

## Choosing a Class

| | `DocumentConnector` | `DocumentPipeline` | `DocumentPipeline(embed=False)` |
|---|---|---|---|
| Stores | Full documents | Documents + chunks | Documents + chunks |
| Embeddings | No | Yes (configurable) | No |
| Indexes | BM25 on documents | BM25 + HNSW on chunks | BM25 on chunks |
| Best for | Keyword search on whole docs | Semantic/hybrid search on chunks | Keyword search on chunks |

See the [full README](https://github.com/kreuzberg-dev/kreuzberg-surrealdb) for the complete API reference, embedding models, chunking configuration, and database schema details.

---

- [SurrealDB documentation](https://surrealdb.com/docs)
