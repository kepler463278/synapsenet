use anyhow::Result;
use rusqlite::Connection;
use tracing::{info, warn};

/// Migrate v0.3 database to v0.4 format
/// 
/// This handles:
/// - Adding default values for new GrainMeta fields (embedding_model, embedding_dimensions)
/// - Creating new tables (grain_access, embedding_models, peer_clusters)
/// - Updating schema version
pub fn migrate_v03_to_v04(db_path: &str) -> Result<()> {
    info!("Starting v0.3 -> v0.4 migration for database: {}", db_path);

    let conn = Connection::open(db_path)?;

    // Check if this is a v0.3 database
    let version = get_schema_version(&conn)?;
    if version >= 3 {
        info!("Database is already at v{}, no migration needed", version);
        return Ok(());
    }

    if version == 0 {
        warn!("Database appears to be empty or very old (v0). Running full migrations.");
        crate::migrations::run_migrations(&conn)?;
        return Ok(());
    }

    info!("Detected v{} database, migrating to v4", version);

    // Run standard migrations (will create new tables)
    crate::migrations::run_migrations(&conn)?;

    // Update existing grains with default embedding metadata
    update_grain_metadata(&conn)?;

    info!("✓ Migration v0.3 -> v0.4 complete!");
    Ok(())
}

/// Get schema version from database
fn get_schema_version(conn: &Connection) -> Result<i32> {
    // Check if version table exists
    let table_exists: bool = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='schema_version'",
        [],
        |row| {
            let count: i32 = row.get(0)?;
            Ok(count > 0)
        },
    )?;

    if !table_exists {
        return Ok(0);
    }

    // Get version
    let version: Result<i32, _> = conn.query_row(
        "SELECT version FROM schema_version LIMIT 1",
        [],
        |row| row.get(0),
    );

    Ok(version.unwrap_or(0))
}

/// Update existing grain metadata with default values for new fields
fn update_grain_metadata(conn: &Connection) -> Result<()> {
    info!("Updating grain metadata with default embedding info");

    // Count grains that need updating
    let grain_count: usize = conn.query_row("SELECT COUNT(*) FROM grains", [], |row| row.get(0))?;

    if grain_count == 0 {
        info!("No grains to update");
        return Ok(());
    }

    info!("Found {} grains to update", grain_count);

    // Note: Since GrainMeta is stored as bincode BLOB, and we use Option<T> with #[serde(default)]
    // for new fields, existing grains will automatically deserialize with None values.
    // No actual data migration is needed - the Rust code handles backward compatibility.

    // However, we can register a default embedding model for tracking purposes
    let default_model = "all-MiniLM-L6-v2"; // Most common model in v0.3
    let default_dims = 384;
    let loaded_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;

    conn.execute(
        "INSERT OR IGNORE INTO embedding_models (name, dimensions, file_size_mb, loaded_at) 
         VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![default_model, default_dims, 22.0, loaded_at],
    )?;

    info!(
        "✓ Registered default embedding model: {} ({}D)",
        default_model, default_dims
    );
    info!("✓ Grain metadata is backward compatible (no data changes needed)");

    Ok(())
}

/// Check if database needs migration
pub fn needs_migration(db_path: &str) -> Result<bool> {
    let conn = Connection::open(db_path)?;
    let version = get_schema_version(&conn)?;
    Ok(version < 4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_migration_detection() {
        let test_db = "test_migration.db";
        
        // Clean up if exists
        let _ = fs::remove_file(test_db);

        // Create v0.3-style database
        let conn = Connection::open(test_db).unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE grains (
                id BLOB PRIMARY KEY,
                vec BLOB NOT NULL,
                meta BLOB NOT NULL,
                sig BLOB NOT NULL,
                created_at INTEGER NOT NULL
            );
            CREATE TABLE schema_version (version INTEGER PRIMARY KEY);
            INSERT INTO schema_version (version) VALUES (2);
            "#,
        )
        .unwrap();
        drop(conn);

        // Check if migration is needed
        assert!(needs_migration(test_db).unwrap());

        // Run migration
        migrate_v03_to_v04(test_db).unwrap();

        // Check migration completed
        assert!(!needs_migration(test_db).unwrap());

        // Verify new tables exist
        let conn = Connection::open(test_db).unwrap();
        let table_count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('grain_access', 'embedding_models', 'peer_clusters')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(table_count, 3);

        // Clean up
        drop(conn);
        let _ = fs::remove_file(test_db);
    }
}
