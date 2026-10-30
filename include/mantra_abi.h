/**
 * ==============================================================================
 * Mantra Programming Language — Stable Native Application Binary Interface (ABI)
 * include/mantra_abi.h
 *
 * Edition 2026 / Version 1.0 Stable C-Compatible ABI Specification
 *
 * Defines the standard C ABI memory layout, object headers, vtable structures,
 * type descriptors, and FFI interop functions for compiled Mantra binaries.
 *
 * Copyright (c) 2026 Solotron Technologies. All rights reserved.
 * ==============================================================================
 */

#ifndef MANTRA_ABI_H
#define MANTRA_ABI_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* --- ABI Versioning Invariants --- */
#define MANTRA_ABI_VERSION_MAJOR 1
#define MANTRA_ABI_VERSION_MINOR 0
#define MANTRA_ABI_VERSION_PATCH 0
#define MANTRA_ABI_MAGIC         0x4D414E5452410001ULL /* "MANTRA\0\1" */

/* --- Standard Calling Conventions & Export Macros --- */
#if defined(_WIN32) || defined(__CYGWIN__)
    #ifdef MANTRA_BUILD_DLL
        #define MANTRA_API __declspec(dllexport)
    #else
        #define MANTRA_API __declspec(dllimport)
    #endif
    #define MANTRA_CALL __cdecl
#else
    #if __GNUC__ >= 4
        #define MANTRA_API __attribute__((visibility("default")))
    #else
        #define MANTRA_API
    #endif
    #define MANTRA_CALL
#endif

/* --- Primitive Native Type Definitions --- */
typedef int64_t     MantraInt;
typedef double      MantraFloat;
typedef uint8_t     MantraBool;
typedef void*       MantraAny;

/* --- Standard String View (Length-Prefixed, UTF-8 Encoded) --- */
typedef struct MantraString {
    const char* data;      /* Pointer to immutable UTF-8 bytes */
    size_t      length;    /* Byte count (excluding trailing null) */
    uint32_t    hash;      /* Cached 32-bit FNV-1a hash */
    uint32_t    flags;     /* Flags (e.g., SSO, static, heap) */
} MantraString;

/* --- Standard Object Header (Prepended to all Mantra Heap Allocations) --- */
typedef struct MantraVTable MantraVTable;
typedef struct MantraTypeInfo MantraTypeInfo;

typedef struct MantraHeader {
    uint64_t            magic;       /* Verification magic constant */
    const MantraVTable* vtable;      /* Pointer to type's virtual table */
    _Atomic uint32_t    refcount;    /* Thread-safe atomic reference count */
    uint32_t            flags;       /* GC/ARC flags (0x1 = pinned, 0x2 = finalizer) */
} MantraHeader;

/* --- Base Mantra Object Structure --- */
typedef struct MantraObject {
    MantraHeader header;
    /* User fields follow immediately in memory aligned to 8 bytes */
} MantraObject;

/* --- Virtual Method Table Structure --- */
typedef void (*MantraMethodFn)(void);
typedef void (MANTRA_CALL *MantraDestructorFn)(MantraObject* self);

struct MantraVTable {
    const MantraTypeInfo* type_info;        /* Pointer to reflection metadata */
    size_t                instance_size;    /* Size of object in bytes */
    MantraDestructorFn    destructor;       /* Invoked on refcount == 0 */
    size_t                method_count;     /* Number of virtual methods */
    MantraMethodFn        methods[1];       /* Flexible array member of virtual functions */
};

/* --- Type Information & Reflection Metadata --- */
struct MantraTypeInfo {
    const char*           type_name;        /* Fully qualified name: e.g. "mantra.io.File" */
    const MantraTypeInfo* super_type;       /* Pointer to superclass metadata (or NULL) */
    size_t                interface_count;  /* Implemented interface count */
    const MantraTypeInfo** interfaces;      /* Array of interface type pointers */
    uint32_t              type_id;          /* Unique compiler-assigned type identifier */
};

/* --- Dynamic Array / List Structure --- */
typedef struct MantraList {
    MantraHeader  header;
    void**        data;        /* Array of element pointers */
    size_t        length;      /* Current element count */
    size_t        capacity;    /* Allocated capacity */
} MantraList;

/* --- Standard Result Envelope for FFI Boundary Invocations --- */
typedef enum MantraStatusCode {
    MANTRA_SUCCESS          = 0,
    MANTRA_ERR_GENERIC      = 1,
    MANTRA_ERR_NULL_PTR     = 2,
    MANTRA_ERR_TYPE_MISMATCH= 3,
    MANTRA_ERR_OUT_OF_BOUNDS= 4,
    MANTRA_ERR_PANIC        = 101,
} MantraStatusCode;

typedef struct MantraResult {
    MantraStatusCode code;
    const char*      error_message;
    MantraObject*    value;
} MantraResult;

/* --- Memory Management & Lifecycle Functions --- */
MANTRA_API MantraObject* MANTRA_CALL mantra_alloc(const MantraVTable* vtable, size_t size);
MANTRA_API void          MANTRA_CALL mantra_retain(MantraObject* obj);
MANTRA_API void          MANTRA_CALL mantra_release(MantraObject* obj);
MANTRA_API MantraString  MANTRA_CALL mantra_string_from_utf8(const char* cstr, size_t len);

/* --- Dynamic FFI Loader Protocol --- */
typedef MantraResult (MANTRA_CALL *MantraFfiEntryPoint)(const MantraList* args);

#ifdef __cplusplus
}
#endif

#endif /* MANTRA_ABI_H */

