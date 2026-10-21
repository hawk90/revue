//! `#[derive(Store)]` and the store registry

use std::sync::Arc;

use revue::reactive::store::{Store, StoreRegistry};

#[derive(revue::Store, Default)]
struct SettingsStore {
    theme: u8,
}

#[derive(revue::Store, Default)]
struct SettingsStoreV2;

mod other {
    #[derive(revue::Store, Default)]
    pub struct SettingsStore;
}

/// The id used to come from the first 8 bytes of the name, so names that
/// share them got one id and the registry kept only the last (#1191).
#[test]
fn derived_ids_differ_when_names_share_a_prefix() {
    assert_ne!(SettingsStore::default().id(), SettingsStoreV2.id());
}

#[test]
fn derived_ids_differ_for_the_same_name_in_another_module() {
    assert_ne!(SettingsStore::default().id(), other::SettingsStore.id());
    assert_eq!(other::SettingsStore.name(), "SettingsStore");
}

#[test]
fn a_derived_id_is_the_same_for_every_instance_of_a_type() {
    let dark = SettingsStore { theme: 1 };
    assert_eq!(dark.theme, 1);
    assert_eq!(dark.id(), SettingsStore::default().id());
}

#[test]
fn the_registry_keeps_stores_whose_names_share_a_prefix() {
    let registry = StoreRegistry::new();
    registry.register(Arc::new(SettingsStore::default()));
    registry.register(Arc::new(SettingsStoreV2));

    assert_eq!(registry.all().len(), 2);
    assert!(registry.find_by_name("SettingsStore").is_some());
    assert!(registry.find_by_name("SettingsStoreV2").is_some());
}
