# ZFS JSON Data Structure Documentation

This document describes the structure of JSON output from the `zfs list -t all -j` command for parsing and displaying ZFS statistics.

## Root Structure

```json
{
  "output_version": {
    "command": "zfs list",
    "vers_major": 0,
    "vers_minor": 1
  },
  "datasets": {
    // Object containing all datasets keyed by their name
  }
}
```

## Output Version
- `command`: Always "zfs list"
- `vers_major`: Major version number (currently 0)
- `vers_minor`: Minor version number (currently 1)

## Datasets Object
The `datasets` object contains all ZFS datasets (filesystems, snapshots, bookmarks) keyed by their full name.

### Dataset Types
1. **FILESYSTEM** - Regular ZFS filesystems
2. **SNAPSHOT** - Point-in-time snapshots (names contain `@`)
3. **BOOKMARK** - ZFS bookmarks (names contain `#`)

### Common Dataset Properties
Each dataset has the following structure:

```json
{
  "name": "dataset/path/name",
  "type": "FILESYSTEM|SNAPSHOT|BOOKMARK",
  "pool": "pool_name",
  "createtxg": "transaction_group_number",
  "properties": {
    "used": {
      "value": "size_string",
      "source": {
        "type": "NONE|LOCAL|INHERITED|DEFAULT",
        "data": "source_data"
      }
    },
    "available": {
      "value": "size_string",
      "source": { ... }
    },
    "referenced": {
      "value": "size_string", 
      "source": { ... }
    },
    "mountpoint": {
      "value": "mount_path|none|-",
      "source": { ... }
    }
  }
}
```

### Snapshot-Specific Properties
Snapshots have additional properties:
- `dataset`: The parent dataset name
- `snapshot_name`: The snapshot name (after `@`)

### Property Details

#### Size Properties
- **used**: Space used by this dataset and all its descendants
- **available**: Available space in the dataset (only for filesystems)
- **referenced**: Space referenced by this dataset alone

Size values are human-readable strings like:
- `"1.09T"` (terabytes)
- `"358G"` (gigabytes) 
- `"192K"` (kilobytes)
- `"0B"` (bytes)
- `"-"` (not applicable, e.g., for snapshots' available space)

#### Mountpoint Property
- Regular path like `"/home"`, `"/root"`, `"/"`
- `"none"` - not mounted
- `"-"` - not applicable (snapshots, bookmarks)

#### Property Sources
- `"NONE"` - Default/calculated value
- `"LOCAL"` - Explicitly set on this dataset
- `"INHERITED"` - Inherited from parent
- `"DEFAULT"` - ZFS default value

## Example Parsing Strategy

1. Parse the root JSON object
2. Check `output_version` for compatibility
3. Iterate through `datasets` object
4. Group by type (FILESYSTEM, SNAPSHOT, BOOKMARK)
5. For filesystems: extract space usage and mountpoint info
6. For snapshots: group by parent dataset and extract space usage
7. Parse size strings to bytes for calculations and comparisons

## Key Insights from Sample Data

- Pool name: `novafs`
- Contains multiple filesystem hierarchies: `novafs/arch0`, `novafs/archold`, etc.
- Heavy use of snapshots with `zrepl_` prefix (backup system)
- Some filesystems have `mountpoint: "none"` (not mounted)
- Bookmark names contain `#` and are used for replication cursors
- Size values need parsing for meaningful display and calculations

## Size Parsing Notes

Size strings follow standard format:
- Numbers followed by unit suffix (B, K, M, G, T, P)
- May contain decimal points (e.g., "1.09T", "48.7M")
- Need conversion to bytes for calculations
- "-" indicates not applicable/unavailable
