# Next Steps for ATProto Rust SDK

## Current Status

**Completed Files (286 tests passing + 39 doc tests):**

**NEW: Lexicon Code Generator** ✨✨✨
- ✅ Parses ATProto Lexicon JSON schemas
- ✅ Generates type-safe Rust query endpoints
- ✅ Generates type-safe Rust procedure endpoints
- ✅ **Generates type-safe Rust record types** (NEW!)
- ✅ **Generates nested object types automatically** (NEW!)
- ✅ Full documentation from lexicon descriptions
- ✅ Proper type mapping (DID, AtUri, etc.)
- ✅ Error type generation
- ✅ **Serde annotations (rename, skip_serializing_if)** (NEW!)
- ⚠️ Token types not yet implemented
- ⚠️ Namespace organization pending

**Core SDK Modules:**
- ✅ types.ts → types.rs (36 tests)
- ✅ const.ts → consts.rs (4 tests)
- ✅ util.ts → util.rs (27 tests)
- ✅ session-manager.ts → session_manager.rs (6 tests)
- ✅ **XRPC client** → xrpc/mod.rs (24 tests)
  - ✅ Exponential backoff retry logic
  - ✅ Configurable retry attempts, delays, and backoff multiplier
  - ✅ Smart error detection (retries network/server errors, not client errors)
- ✅ AtUri parser → syntax.rs (24 tests)
- ✅ **Rich Text Module (92 tests):**
  - ✅ unicode.ts → unicode.rs (25 tests)
  - ✅ util.ts → util.rs (25 tests)
  - ✅ detection.ts → detection.rs (19 tests)
  - ✅ rich-text.ts → rich_text.rs (15 tests)
  - ✅ sanitization.ts → sanitization.rs (8 tests)
- ✅ **TID (Timestamp Identifier)** → tid.rs (16 tests)
- ✅ **DID Resolution** → did_doc.rs (12 tests)
- ✅ **Agent (Core)** → agent.rs (17 tests)
  - Note: Core Agent without client namespaces (ComNS, AppNS, etc.)
- ✅ **Handle Resolution** → handle.rs (30 tests)
  - Handle validation and normalization
  - HTTPS well-known resolution
  - DNS TXT record resolution (stub for future DNS library integration)

**Partially Complete:**
- ⚠️ agent.ts - Core complete, but full client namespaces still need generation
- ❌ atp-agent.ts - Requires full agent.ts + client code
- ❌ bsky-agent.ts - Requires full agent.ts
- ❌ predicate.ts - Requires client validation functions

---

## Analysis: What's Blocking agent.ts?

### Major Dependency: Client Code (~300 files)

The client code is **auto-generated** from lexicon schemas and includes:
- Namespace classes (ComNS, AppNS, ChatNS, ToolsNS)
- Type definitions for all API endpoints
- Validation functions
- Request/response types

**Options for handling client code:**

### Option A: Build a Lexicon→Rust Code Generator
- **Effort:** High (several days)
- **Benefit:** Automated, maintainable, matches TypeScript approach
- **Files to study:** `scripts/` directory in TypeScript repo

### Option B: Manual Translation of Core Client Types
- **Effort:** Very High (weeks)
- **Benefit:** Full control, but tedious
- **Downside:** Hard to maintain when lexicons change

### Option C: Mixed Approach
- Manually translate essential types needed by agent.ts
- Build generator for remaining bulk
- **Effort:** Medium-High

---

## Recommended Next Steps (In Order)

### Path 1: Complete the Lexicon Code Generator (IN PROGRESS!)

**Current Status:** ✅ Basic generator working! ⚠️ Needs completion

**What's Done:**
- ✅ Lexicon JSON parser
- ✅ Query endpoint generation
- ✅ Procedure endpoint generation
- ✅ Type mapping (primitives, DIDs, At URIs)
- ✅ Error type generation
- ✅ Documentation generation

**What's Needed:**
1. **Record type generation** - For data models (posts, profiles, etc.)
2. **Namespace organization** - Generate mod.rs files and namespace structs
3. **Integration with Agent** - Wire up ComNS, AppNS, ChatNS, ToolsNS
4. **Get real lexicon files** - Download from bluesky-social/atproto repo
5. **Generate all endpoints** - Run generator on full lexicon set

**Value:**
- Completes the SDK with all 300+ API endpoints
- Makes Agent fully functional
- Enables building complete ATProto applications

**Effort:**
- 1-2 more days to complete

---

### Path 2: Simplified Client API (For Testing - Alternative)

Create **minimal** client types for testing agent.ts:

1. **Stub namespace structures** (ComNS, AppNS)
2. **Essential preference types**
3. **Key endpoint methods** (getProfile, createPost)

**Value:**
- Allows testing agent.ts functionality
- Can be replaced with full generated code later

**Downside:**
- Partial implementation (violates CLAUDE.md unless clearly scoped)

---

### Path 4: Lexicon Code Generator (Long-term)

Build a proper code generator:

1. **Lexicon parser** - Parse .json lexicon schemas
2. **Rust code emitter** - Generate structs, enums, validation
3. **Namespace generator** - Create ComNS, AppNS structures
4. **Integration** - Wire up to XRPC client

**Value:**
- Proper, maintainable solution
- Matches TypeScript approach
- Handles all 300+ client files

**Effort:**
- Multiple days of work
- Requires understanding lexicon format thoroughly

---

## My Strong Recommendation

**Option A: Handle Resolution (Path 1)**

**Reasons:**
1. ✅ **Complete implementation possible** - No dependencies we don't have
2. ✅ **High value** - Critical for making Agent actually useful
3. ✅ **Complements what we just built** - DID resolution already done
4. ✅ **Clear requirements** - Well-defined DNS/HTTPS protocols
5. ✅ **Testable** - Can test with real handles or mocks
6. ✅ **Follows CLAUDE.md** - Can be 100% complete

**What this unlocks:**
- Convert handles (alice.bsky.social) to DIDs
- Lookup users by handle
- Fully functional Agent for user operations

---

**Option B: Retry Logic (Path 2) - Quick Win**

**Reasons:**
1. ✅ **Very short** - ~50 lines
2. ✅ **Production value** - Makes XRPC client reliable
3. ✅ **No dependencies** - Pure Rust
4. ✅ **Common pattern** - Standard HTTP client practice

**What this unlocks:**
- Production-ready XRPC client
- Resilience to transient network failures

---

**Option C: Lexicon Code Generator (Path 4) - Long-term Investment**

If you want to unblock the full client API with all namespaces, this is the path. However, it's a multi-day effort.

---

## Alternative: PDS Development

If the goal is also to build a PDS, we could shift focus to PDS-specific components:

1. **Repository storage** - Managing user repositories
2. **Record validation** - Lexicon-based validation
3. **Event streams** - Firehose implementation
4. **Identity resolution** - DID/handle resolution (we have DID, need handle)

**Note:** PDS also needs some client types, but fewer than the full SDK.

---

## Suggested Immediate Next Steps

**My recommendation: Complete the Lexicon Code Generator**

The code generator is now **functional** and generating working code! The foundation is in place. The next steps are:

1. **Add record type support** (most common lexicon type)
2. **Download real lexicon files** from the ATProto repo
3. **Generate the full client API** (all 300+ endpoints)
4. **Organize into namespaces** (ComNS, AppNS, etc.)
5. **Integrate with Agent** for full SDK functionality

**Why this is the right path:**
- Foundation is proven and working
- Each step builds incrementally
- Immediate value with each endpoint generated
- Matches the proven TypeScript approach

**Quick Win Alternative:** Generate a few key endpoints manually (getProfile, createPost, getTimeline) to make the SDK immediately useful while continuing generator work.

The code generator breakthrough puts us on the path to a complete, production-ready SDK! 🚀
