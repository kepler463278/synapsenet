use anyhow::Result;
use rusqlite::Connection;
use tracing::info;

/// Database schema version
const CURRENT_VERSION: i32 = 4;

/// Run all pending migrations
pub fn run_migrations(conn: &Connection) -> Result<()> {
    let version = get_schema_version(conn)?;
    info!("Current schema version: {}", version);

    if version < CURRENT_VERSION {
        info!("Running migrations from v{} to v{}", version, CURRENT_VERSION);

        if version < 1 {
            migrate_to_v1(conn)?;
        }

        if version < 2 {
            migrate_to_v2(conn)?;
        }

        if version < 3 {
            migrate_to_v3(conn)?;
        }

        if version < 4 {
            migrate_to_v4(conn)?;
        }

        set_schema_version(conn, CURRENT_VERSION)?;
        info!("✓ Migrations complete");
    } else {
        info!("Schema is up to date");
    }

    Ok(())
}

/// Get current schema version
fn get_schema_version(conn: &Connection) -> Result<i32> {
    // Create version table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (version INTEGER PRIMARY KEY)",
        [],
    )?;

    // Get version or default to 0
    let version: Result<i32, _> = conn.query_row(
        "SELECT version FROM schema_version LIMIT 1",
        [],
        |row| row.get(0),
    );

    Ok(version.unwrap_or(0))
}

/// Set schema version
fn set_schema_version(conn: &Connection, version: i32) -> Result<()> {
    conn.execute("DELETE FROM schema_version", [])?;
    conn.execute(
        "INSERT INTO schema_version (version) VALUES (?1)",
        [version],
    )?;
    Ok(())
}

/// Migration to v1: Initial schema (already exists in store.rs)
fn migrate_to_v1(_conn: &Connection) -> Result<()> {
    info!("Migration v0 -> v1: Initial schema (already applied)");
    Ok(())
}

/// Migration to v2: Add embedding model metadata to grains
fn migrate_to_v2(_conn: &Connection) -> Result<()> {
    info!("Migration v1 -> v2: Adding embedding model metadata");

    // Note: Since we store meta as BLOB (bincode), we don't need to alter the table
    // The new fields are optional and will be None for existing grains
    // This is handled by serde's #[serde(default)] attribute

    info!("✓ Migration v1 -> v2 complete (backward compatible)");
    Ok(())
}

/// Migration to v3: Add grain_access tracking table
fn migrate_to_v3(conn: &Connection) -> Result<()> {
    info!("Migration v2 -> v3: Creating grain_access tracking table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS grain_access (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            grain_id BLOB NOT NULL,
            peer_id TEXT NOT NULL,
            access_type TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        )",
        [],
    )?;

    // Create indexes for efficient queries
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_grain_access_grain_id ON grain_access(grain_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_grain_access_timestamp ON grain_access(timestamp)",
        [],
    )?;

    info!("✓ Migration v2 -> v3 complete");
    Ok(())
}

/// Migration to v4: Add embedding_models and peer_clusters tables
fn migrate_to_v4(conn: &Connection) -> Result<()> {
    info!("Migration v3 -> v4: Creating embedding_models and peer_clusters tables");

    // Embedding models metadata table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS embedding_models (
            name TEXT PRIMARY KEY,
            dimensions INTEGER NOT NULL,
            file_size_mb REAL NOT NULL,
            loaded_at INTEGER NOT NULL
        )",
        [],
    )?;

    // Peer clusters table for topic-based clustering
    conn.execute(
        "CREATE TABLE IF NOT EXISTS peer_clusters (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            topic TEXT NOT NULL,
            peer_id TEXT NOT NULL,
            similarity REAL NOT NULL,
            last_seen INTEGER NOT NULL,
            UNIQUE(topic, peer_id)
        )",
        [],
    )?;

    // Create indexes
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_peer_clusters_topic ON peer_clusters(topic)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_peer_clusters_last_seen ON peer_clusters(last_seen)",
        [],
    )?;

    info!("✓ Migration v3 -> v4 complete");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrations() {
        let conn = Connection::open_in_memory().unwrap();
        
        // Initial version should be 0
        assert_eq!(get_schema_version(&conn).unwrap(), 0);

        // Run migrations
        run_migrations(&conn).unwrap();

        // Should be at current version
        assert_eq!(get_schema_version(&conn).unwrap(), CURRENT_VERSION);

        // Running again should be no-op
        run_migrations(&conn).unwrap();
        assert_eq!(get_schema_version(&conn).unwrap(), CURRENT_VERSION);
    }
}
