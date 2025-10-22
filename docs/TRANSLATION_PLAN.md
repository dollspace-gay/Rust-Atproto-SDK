# ATProto SDK Translation Plan

## Goal
Translate the TypeScript ATProto SDK to Rust, file by file, following the dependency chain to ensure each component has its dependencies available.

## Status: Foundation Complete ✓

### Completed Files
- [x] **types.ts** → `src/types.rs` (36 tests passing)
- [x] **const.ts** → `src/consts.rs` (4 tests passing)
- [x] **util.ts** → `src/util.rs` (27 tests passing)

**Total: 67 unit tests passing ✓**

---

## Current Task: Agent Dependencies

### agent.ts Dependency Analysis

The `agent.ts` file (1,595 lines) is the core of the SDK but has significant dependencies:

```
agent.ts
├── External Dependencies (from other packages)
│   ├── @atproto/xrpc
│   │   ├── XrpcClient (base HTTP client class)
│   │   ├── FetchHandler (HTTP fetch abstraction)
│   │   └── buildFetchHandler (fetch handler builder)
│   ├── @atproto/syntax
│   │   ├── AtUri (AT Protocol URI parser)
│   │   └── ensureValidDid (DID validation)
│   ├── @atproto/common-web
│   │   ├── TID (Timestamp Identifier)
│   │   └── retry (retry logic)
│   └── await-lock (async locking)
│
├── Internal Dependencies (in this package)
│   ├── session-manager.ts (6 lines - simple interface)
│   ├── client/index.ts (auto-generated, ~300+ files)
│   │   ├── AppBskyActorDefs
│   │   ├── AppNS, ComNS, ChatNS, ToolsNS (namespace classes)
│   │   └── Lexicon schemas
│   ├── moderation/ (entire moderation system)
│   │   ├── interpretLabelValueDefinitions
│   │   ├── DEFAULT_LABEL_SETTINGS
│   │   └── ModerationPrefs types
│   └── predicate.ts (type guards/predicates)
│
└── Already Translated ✓
    ├── types.ts
    ├── const.ts
    └── util.ts
```

---

## Translation Strategy: Dependency-First Approach

### Phase 1: External Package Foundations
**Goal:** Create Rust equivalents of external dependencies

1. **HTTP Client Foundation** (`src/xrpc/`)
   - Create basic `XrpcClient` trait/struct
   - Implement fetch handler using `reqwest`
   - Add retry logic with exponential backoff
   - **Estimated complexity:** Medium (200-300 lines)

2. **Syntax Utilities** (`src/syntax/`)
   - Implement `AtUri` parser (AT Protocol URI format)
   - Enhance `Did` validation from types.rs
   - **Estimated complexity:** Low-Medium (100-200 lines)

3. **Common Utilities** (`src/common/`)
   - Implement TID (Timestamp Identifier) generation
   - Retry utilities
   - **Estimated complexity:** Low (50-100 lines)

### Phase 2: Simple Internal Dependencies
**Goal:** Translate straightforward internal files

4. **session-manager.ts** → `src/session_manager.rs`
   - Simple trait definition (6 lines in TS)
   - Defines interface for session management
   - **Estimated complexity:** Very Low (20-30 lines)

5. **predicate.ts** → `src/predicate.rs`
   - Type guards and validation predicates
   - **Estimated complexity:** Low (check file size first)

### Phase 3: Client Code Foundation
**Goal:** Handle auto-generated client code

6. **Lexicon System** (`src/lexicon/`)
   - Understand lexicon schema format
   - Decide on code generation strategy:
     - Option A: Manual translation of key types
     - Option B: Create Rust code generator from lexicons
     - Option C: Hybrid approach
   - Start with core record types

7. **Client Namespaces** (`src/client/`)
   - Translate namespace structure (ComNS, AppNS, ChatNS, ToolsNS)
   - Implement key RPC methods incrementally
   - **Note:** This is the largest task (~300 files in TypeScript)

### Phase 4: Preferences System
**Goal:** User preferences management (needed by agent.ts)

8. **Preferences Logic**
   - Feed view preferences
   - Thread view preferences
   - Saved feeds management
   - Already have types from types.rs, need business logic

### Phase 5: Moderation System
**Goal:** Content moderation (complete implementation required)

9. **Moderation Types** → `src/moderation/types.rs`
10. **Label Interpretation** → `src/moderation/labels.rs`
11. **Moderation Decisions** → `src/moderation/decision.rs`

**Note:** Moderation system must be fully implemented. May be tackled after core agent functionality if it's a large dependency.

### Phase 6: Agent Implementation
**Goal:** Finally translate agent.ts

12. **agent.ts** → `src/agent.rs`
    - Main Agent struct
    - Session integration
    - Proxy configuration
    - Labeler configuration
    - Preferences API
    - Clone/copy utilities

---

## Recommended Immediate Next Steps

### Step 1: session-manager.ts (Easiest)
- **File:** `src/session_manager.rs`
- **Complexity:** Very Low
- **Dependencies:** None (just trait definition)
- **Estimated time:** 15-30 minutes

### Step 2: Basic XRPC Client
- **File:** `src/xrpc.rs` or `src/xrpc/mod.rs`
- **Complexity:** Medium
- **Dependencies:** reqwest (already in Cargo.toml)
- **Key features:**
  - HTTP request/response handling
  - Header management
  - Error types
  - Basic retry logic
- **Estimated time:** 1-2 hours

### Step 3: AtUri Parser
- **File:** `src/syntax.rs`
- **Complexity:** Low-Medium
- **Dependencies:** regex (already in Cargo.toml)
- **Key features:**
  - Parse `at://did:plc:abc/collection/rkey` format
  - Extract DID, collection, rkey
  - Validation
- **Estimated time:** 30-60 minutes

### Step 4: Simplified Agent (MVP)
- **File:** `src/agent.rs`
- **Complexity:** High (but we can stub sections)
- **Strategy:**
  - Core agent struct
  - Session management integration
  - Basic HTTP client wrapping
  - Stub preferences/moderation for now
  - Add functionality incrementally

---

## Alternative: Incremental Agent Approach

Instead of translating all dependencies first, we could:

1. Create a **minimal agent.rs** with:
   - Basic struct definition
   - Session manager trait
   - Simplified HTTP client (without full XRPC)
   - Stub methods marked with `todo!()`

2. **Benefits:**
   - See the full structure earlier
   - Understand what's needed
   - Test compilation early

3. **Drawbacks:**
   - Won't be functional until dependencies filled in
   - More refactoring later

---

## Decision Point

**Which path should we take?**

### Path A: Dependency-First (Recommended)
```
session-manager.rs → xrpc.rs → syntax.rs → agent.rs
```
- Each file is functional when written
- Builds incrementally
- More satisfying progress

### Path B: Structure-First
```
agent.rs (with stubs) → fill in dependencies → complete agent.rs
```
- See end goal earlier
- More refactoring
- Less functional initially

---

## Questions to Answer

1. **Client code generation:**
   - Should we write a lexicon → Rust code generator?
   - Or manually translate key types/endpoints?
   - Hybrid: manual core types, generate repetitive code?

2. **Moderation:**
   - Stub it for now and implement later?
   - Or translate moderation system next?

3. **Testing strategy:**
   - Unit tests for each module (current approach ✓)
   - Integration tests for agent once complete?
   - Mock services for testing?

---

## Next File Recommendation

**Start with: session-manager.ts → session_manager.rs**

Why?
- Only 6 lines
- No dependencies
- Quick win
- Required by agent.ts
- Will inform XRPC design

After that:
- Basic XRPC client (simplified version)
- AtUri parser
- Then tackle agent.ts with proper foundation
