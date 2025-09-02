use std::{fs, path::PathBuf};
use specta_typescript::Typescript;

// Reuse server types
use zfs_stats_web::types::{
    Dataset, DatasetProperties, OutputVersion, Property, PropertySource, ZfsListOutput, ZfsStats,
};

fn main() {
    // Determine output path: project_root/src/bindings.ts
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out_path = manifest_dir.join("../src/bindings.ts");

    let ts = Typescript::default();
    // Exporting ZfsStats will include all referenced types
    let contents = specta_typescript::export::<ZfsStats>(&ts).expect("export types");

    fs::write(&out_path, contents).expect("write bindings.ts");
    println!(
        "Exported Specta TypeScript types to {}",
        out_path.display()
    );
}
