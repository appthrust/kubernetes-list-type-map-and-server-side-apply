# Kubernetes: `listType=map` And Server-side Apply Demo

## How to run

CRD `document.suin.jp` を作成します:

```bash
cargo run --bin apply_crd
```

CR `example` を作成します:

```bash
cargo run --bin create
```

すると、次のようなCRが作成されます:

```yaml

apiVersion: suin.jp/v1
kind: Document
metadata:
  # ...
  name: example
  namespace: default
  # ...
spec:
  tags:
    - key: key1
      value: value1
    - key: key2
      value: value2
    - key: key3
      value: value3
    - key: key4
      value: value4
  title: Example document
```

このCRは2つのマネージャーによって管理されています。マネージャーはそれぞれ異なるタグを持っています:

- マネージャー1: `manager-1`
  - タグ: `key1`, `key2`
- マネージャー2: `manager-2`
  - タグ: `key3`, `key4`

CR `example` を更新します:

```bash
cargo run --bin update
```

この処理では、次のことをします。

- マネージャー1はタグ `key2` を `new-value2!` に更新します。
  - `key1`はペイロードに含まれていません。
- マネージャー2はタグ `key4` を `new-value4!` に更新します。
  - `key3`はペイロードに含まれていません。

この処理の詳細は[update.rs](./src/update.rs)を参照してください。

すると、次のような差分が発生します:

```diff
manager-1でApplyした場合のdiff:
 title: Example document
 tags:
-- key: key1
-  value: value1
 - key: key2
-  value: value2
+  value: new-value2!
 - key: key3
   value: value3
 - key: key4
   value: value4

manager-2でApplyした場合のdiff:
 title: Example document
 tags:
 - key: key1
   value: value1
 - key: key2
   value: value2
-- key: key3
-  value: value3
 - key: key4
-  value: value4
+  value: new-value4!
```