enum FileFormatVersion {
    Legacy,
    Wal,
}

enum TextEncoding {
    Utf8,
    Utf16Le,
    Utf16Be,
}

struct IncrementalVaccuumSettings {
    // The page number of the largest root b-tree page when in auto-vacuum or incremental-vacuum modes, or zero otherwise.
}

// https://www.sqlite.org/fileformat.html
struct DatabaseHeader {
    // The database page size in bytes.
    // Must be a power of two between 512 and 32768 inclusive, or the value 1 representing a page size of 65536.
    page_size: u8,
    // File format write version. 1 for legacy; 2 for WAL.
    file_format_write_version: FileFormatVersion,
    // File format read version. 1 for legacy; 2 for WAL.
    file_format_read_version: FileFormatVersion,
    // Bytes of unused "reserved" space at the end of each page. Usually 0.
    reserved_space: u8,
    // Maximum embedded payload fraction. Must be 64.
    maximum_embedded_payload_fraction: u8,
    // Minimum embedded payload fraction. Must be 32.
    minimum_embedded_payload_fraction: u8,
    // Leaf payload fraction. Must be 32.
    leaf_payload_fraction: u8,
    // File change counter.
    file_change_counter: u8,
    // Size of the database file in pages. The "in-header database size".
    database_size: u8,
    // Page number of the first freelist trunk page.
    first_freelist: u8,
    // Total number of freelist pages.
    num_freelist: u8,
    // The schema cookie.
    schema_cookie: u8,
    // The schema format number. Supported schema formats are 1, 2, 3, and 4.
    schema_format_number: u8,
    // Default page cache size.
    default_page_cache_size: u8,
    // The page number of the largest root b-tree page when in auto-vacuum or incremental-vacuum modes, or zero otherwise.
    largest_root_page: u8,
    // The database text encoding. A value of 1 means UTF-8. A value of 2 means UTF-16le. A value of 3 means UTF-16be.
    text_encoding: TextEncoding,
    // The "user version" as read and set by the user_version pragma.
    user_version: u8,
    // True (non-zero) for incremental-vacuum mode. False (zero) otherwise.
    incremental_vaccuum_mode: bool,
    // The "Application ID" set by PRAGMA application_id.
    application_id: u8,
    version_valid_for: u8,
    sqlite_version_number: u8,
}
