# Build Git from Scratch in Rust

> Following [The Git Parable](https://tom.preston-werner.com/2009/05/19/the-git-parable)

---

## Phase 01 — Snapshots

- [x] `cargo new rgit --bin`, set up `clap` for CLI args
- [x] `rgit init` → create `working/` and `.rgit/` dirs
- [x] `rgit snapshot "msg"` → copy `working/` to `.rgit/snapshots/snapshot-N`
- [x] Write a `message` file inside the snapshot with timestamp + message text
- [x] `rgit log` → print each snapshot's message file in reverse order

---

## Phase 02 — Branches

- [ ] Add a `parent: snapshot-N` line to each new message file
- [ ] Maintain a `branches` file: `<branch-name> <snapshot-N>`
- [ ] `rgit branch <name>` → create new branch pointing at current snapshot
- [ ] `rgit checkout <name>` → copy that snapshot into `working/`
- [ ] Update the active branch pointer after every new snapshot

---

## Phase 03 — Tags

- [ ] Maintain a `tags` file: `<tag-name> <snapshot-N>`
- [ ] `rgit tag <name>` → write current snapshot to tags file
- [ ] Tag checkout is read-only — does not move the pointer

---

## Phase 04 — SHA1 Naming

- [ ] Add `sha1` or `sha2` crate
- [ ] Hash `author + date + parent_sha + message` to get the snapshot name
- [ ] Replace sequential `snapshot-N` naming with the SHA1 hex everywhere
- [ ] Update branches and tags files to reference SHAs

---

## Phase 05 — Staging Area

- [ ] `rgit add <file>` → copy file from `working/` into `.rgit/staging/`
- [ ] `rgit add .` → copy all modified files into staging
- [ ] `rgit snapshot` now reads from `staging/`, not `working/`
- [ ] `rgit status` → list files that differ between working, staging, and last snapshot

---

## Phase 06 — Object Store: Blobs

- [ ] `hash_blob(path)` → SHA1 of raw file content
- [ ] `store_blob(path)` → write content to `objects/<sha1>`, skip if already exists
- [ ] `read_blob(sha1)` → read bytes back from `objects/`

---

## Phase 07 — Object Store: Trees

- [ ] Define `TreeEntry { mode, kind: Blob|Tree, sha1, name }`
- [ ] `build_tree(dir)` → recursively store blobs, collect sorted entries
- [ ] Serialize entries, SHA1 the bytes, write to `objects/<tree-sha>`
- [ ] `read_tree(sha1)` → parse tree object, return `Vec<TreeEntry>`

---

## Phase 08 — Object Store: Commits

- [ ] Define `Commit { tree, parents, author, timestamp, message }`
- [ ] `serialize_commit()` → produce text format, SHA1 it, write to `objects/`
- [ ] `rgit snapshot` now calls `build_tree` + `serialize_commit` end to end
- [ ] `read_commit(sha1)` → parse from `objects/`, return `Commit`
- [ ] Update branches file to point at new commit SHA
- [ ] **Milestone:** `snapshot-N` system fully replaced by the object store

---

## Phase 09 — Checkout from Objects

- [ ] `checkout_tree(sha1, dest_dir)` → recursively restore files from a tree object
- [ ] `rgit checkout <sha1|branch|tag>` → resolve the ref, call `checkout_tree`
- [ ] Update `staging/` and HEAD ref after checkout

---

## Phase 10 — Diffs

- [ ] Line-based LCS diff algorithm
- [ ] Render unified diff format: `@@ -a,b +c,d @@` with context lines
- [ ] `rgit diff` → working vs staging
- [ ] `rgit diff --staged` → staging vs last commit tree
- [ ] `rgit diff <sha1> <sha2>` → diff any two commit trees

---

## Phase 11 — Blob Compression

- [ ] Add `flate2` crate
- [ ] `store_blob`: SHA1 raw bytes first, then deflate before writing
- [ ] `read_blob`: decompress with `ZlibDecoder` before returning
- [ ] Apply same compression to tree and commit objects

---

## Phase 12 — Merges

- [ ] `find_common_ancestor(sha1, sha2)` → walk parent chains to find LCA
- [ ] Three-way file merge: base + ours + theirs → merged output or conflict markers
- [ ] `rgit merge <branch>` → auto-merge clean changes, flag conflicts
- [ ] Merge commit has `Vec<parent_sha>` with two entries

---

## Phase 13 — Log & Reflog

- [ ] `rgit log` → walk parent chain from HEAD, print commit info
- [ ] `--oneline` flag for compact output
- [ ] Append every HEAD change to `.rgit/reflog`
- [ ] `rgit reflog` → print reflog in reverse
- [ ] **Milestone:** fully working content-addressed VCS in Rust
