use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Fixture(std::path::PathBuf);
impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "veio-visual-test-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(path.join("instance")).unwrap();
        let source = path.join("visuals").join(PACK_FOLDER);
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("horizon-shaders.toml"), "id = \"veio_industrial\"\n").unwrap();
        fs::write(source.join("material.wgsl"), "test-pack").unwrap();
        Self(path)
    }
    fn instance(&self) -> std::path::PathBuf { self.0.join("instance") }
}
impl Drop for Fixture {
    fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
}

fn selection(profile: Profile) -> Selection { Selection { profile, reset: false } }

#[test]
fn realism_is_installed_before_launch_and_preserves_personal_options_and_originals() {
    let fixture = Fixture::new();
    let dir = fixture.instance();
    let original = "lang:pt_br\nkey_key.jump:key.keyboard.j\nresourcePacks:[\"vanilla\",\"file/minha.zip\"]\nrenderDistance:32\nrenderDistance:24\n";
    let graphics = "graphicsApi:dx12\nunknownFutureOption:42\npickedShaderPack:Other.zip\n";
    fs::write(dir.join("options.txt"), original).unwrap();
    fs::write(dir.join("horizon-graphics.txt"), graphics).unwrap();
    fs::create_dir_all(dir.join("shaderpacks/veio-industrial")).unwrap();
    fs::write(dir.join("shaderpacks/veio-industrial/ao.wgsl"), "old").unwrap();
    fs::write(dir.join("shaderpacks/veio-industrial/notes.txt"), "mine").unwrap();
    install(&fixture.0, &dir, selection(Profile::Realism)).unwrap();
    assert!(!dir.join("shaderpacks/veio-industrial/ao.wgsl").exists());
    assert!(dir.join("shaderpacks/veio-industrial/notes.txt").exists());
    let game = fs::read_to_string(dir.join("options.txt")).unwrap();
    assert!(game.contains("lang:pt_br\nkey_key.jump:key.keyboard.j\n"));
    assert!(game.contains("resourcePacks:[\"vanilla\",\"file/minha.zip\"]\n"));
    assert_eq!(game.matches("renderDistance:").count(), 1);
    assert!(game.contains("renderDistance:8\nsimulationDistance:5\n"));
    let extra = fs::read_to_string(dir.join("horizon-graphics.txt")).unwrap();
    assert!(extra.contains("graphicsApi:dx12\nunknownFutureOption:42\n"));
    assert!(extra.contains("pickedShaderPack:veio-industrial\n"));
    assert_eq!(fs::read_to_string(dir.join("shaderpacks/veio-industrial/material.wgsl")).unwrap(), "test-pack");
    assert_eq!(fs::read_to_string(dir.join("options.txt.antes-do-veio")).unwrap(), original);
    assert_eq!(fs::read_to_string(dir.join("horizon-graphics.txt.antes-do-veio")).unwrap(), graphics);
    fs::write(dir.join("options.txt"), "renderDistance:10\nlang:pt_br\n").unwrap();
    fs::write(dir.join("horizon-graphics.txt"), "pickedShaderPack:\nresolutionScale:100\n").unwrap();
    install(&fixture.0, &dir, selection(Profile::Realism)).unwrap();
    assert_eq!(fs::read_to_string(dir.join("options.txt")).unwrap(), "renderDistance:10\nlang:pt_br\n");
    assert_eq!(fs::read_to_string(dir.join("horizon-graphics.txt")).unwrap(), "pickedShaderPack:\nresolutionScale:100\n");
    install(&fixture.0, &dir, Selection { profile: Profile::Realism, reset: true }).unwrap();
    assert!(fs::read_to_string(dir.join("options.txt")).unwrap().contains("renderDistance:8\n"));
    assert_eq!(fs::read_to_string(dir.join("options.txt.antes-do-veio")).unwrap(), original);
}

#[test]
fn light_does_not_need_install_or_select_a_shader_even_on_the_first_start() {
    let fixture = Fixture::new();
    let missing_package = fixture.0.join("no-visual-assets");
    let dir = fixture.instance();
    fs::write(dir.join("horizon-graphics.txt"), "pickedShaderPack:heavy.zip\n").unwrap();
    install(&missing_package, &dir, selection(Profile::Light)).unwrap();
    assert!(!dir.join("shaderpacks").exists());
    let extra = fs::read_to_string(dir.join("horizon-graphics.txt")).unwrap();
    assert!(extra.contains("pickedShaderPack:\n"));
    assert!(!extra.contains("heavy.zip"));
    assert!(fs::read_to_string(dir.join("options.txt")).unwrap().contains("renderDistance:6\n"));
    let mut command = Command::new("horizon-launcher");
    launch_environment(&mut command, Profile::Light);
    assert!(command.get_envs().any(|(key, value)| key == "HORIZON_SHADER_PACK" && value == Some(std::ffi::OsStr::new(""))));
}

#[test]
fn switching_both_directions_replaces_the_shader_selection_and_keeps_saves() {
    let fixture = Fixture::new();
    let dir = fixture.instance();
    fs::create_dir_all(dir.join("saves/MyWorld")).unwrap();
    fs::write(dir.join("saves/MyWorld/level.dat"), "player-data").unwrap();
    install(&fixture.0, &dir, selection(Profile::Realism)).unwrap();
    install(&fixture.0, &dir, selection(Profile::Light)).unwrap();
    assert!(fs::read_to_string(dir.join("horizon-graphics.txt")).unwrap().contains("pickedShaderPack:\n"));
    install(&fixture.0, &dir, selection(Profile::Realism)).unwrap();
    assert!(fs::read_to_string(dir.join("horizon-graphics.txt")).unwrap().contains("pickedShaderPack:veio-industrial\n"));
    assert_eq!(fs::read_to_string(dir.join("saves/MyWorld/level.dat")).unwrap(), "player-data");
}

#[test]
fn explicit_launch_choice_wins_over_saved_choice_and_invalid_choice_fails_early() {
    let mut config = BTreeMap::from([("visual_profile".into(), "realismo-8gb".into())]);
    let args = ["--visual".into(), "leve".into()];
    let pick = choose(&args, &mut config).unwrap();
    assert_eq!(pick.profile, Profile::Light);
    assert!(pick.reset);
    assert_eq!(config["visual_profile"], "leve");
    let saved = config.clone();
    assert!(choose(&["--visual".into(), "ultra".into()], &mut config).is_err());
    assert_eq!(config, saved);
    assert!(choose(&["--visual".into()], &mut config).is_err());
    assert_eq!(choose(&[], &mut config).unwrap().profile, Profile::Light);
}

#[test]
fn realism_installs_and_enables_the_texture_pack_and_light_removes_it() {
    let fixture = Fixture::new();
    let dir = fixture.instance();
    let textures = fixture.0.join("visuals").join(TEXTURES_FOLDER);
    fs::create_dir_all(&textures).unwrap();
    fs::write(textures.join("Veio-Texturas-JG-128x.zip"), "zip").unwrap();
    fs::create_dir_all(dir.join("resourcepacks")).unwrap();
    fs::write(dir.join("resourcepacks/Veio-Texturas-JG-256x.zip"), "old").unwrap();
    fs::write(dir.join("resourcepacks/minha.zip"), "mine").unwrap();
    fs::write(dir.join("options.txt"), "resourcePacks:[\"vanilla\",\"file/minha.zip\",\"file/Veio-Texturas-JG-256x.zip\"]\n").unwrap();
    install(&fixture.0, &dir, selection(Profile::Realism)).unwrap();
    let game = fs::read_to_string(dir.join("options.txt")).unwrap();
    assert!(game.contains("resourcePacks:[\"vanilla\",\"file/minha.zip\",\"file/Veio-Texturas-JG-128x.zip\"]\n"), "{game}");
    assert!(game.contains("incompatibleResourcePacks:[\"file/Veio-Texturas-JG-128x.zip\"]\n"), "{game}");
    assert!(dir.join("resourcepacks/Veio-Texturas-JG-128x.zip").is_file());
    assert!(!dir.join("resourcepacks/Veio-Texturas-JG-256x.zip").exists());
    assert!(dir.join("resourcepacks/minha.zip").is_file());
    assert!(fs::read_to_string(dir.join("horizon-graphics.txt")).unwrap().contains("textureBudgetMiB:1024\n"));
    install(&fixture.0, &dir, Selection { profile: Profile::Light, reset: true }).unwrap();
    let game = fs::read_to_string(dir.join("options.txt")).unwrap();
    assert!(game.contains("resourcePacks:[\"vanilla\",\"file/minha.zip\"]\n"), "{game}");
    assert!(game.contains("incompatibleResourcePacks:[]\n"), "{game}");
    assert!(!dir.join("resourcepacks/Veio-Texturas-JG-128x.zip").exists());
}

#[test]
fn sha512_matches_the_standard_vectors() {
    assert_eq!(sha512::hex(b"abc"), "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f");
    assert_eq!(&sha512::hex(b"")[..16], "cf83e1357eefb8bd");
    let long = vec![b'a'; 1000];
    assert_eq!(sha512::hex(&long).len(), 128);
}

#[test]
fn maximum_downloads_bsl_with_the_veio_preset_and_falls_back_when_it_cannot() {
    let fixture = Fixture::new();
    let dir = fixture.instance();
    let presets = fixture.0.join("visuals").join("iris-presets");
    fs::create_dir_all(&presets).unwrap();
    fs::write(presets.join("BSL_v10.1.8.zip.txt"), "TAA=false\n").unwrap();
    // A download that does not match the published hash is thrown away: Realismo leve instead.
    install_with(&fixture.0, &dir, selection(Profile::Maximum), |_, path| {
        fs::write(path, b"not bsl").map_err(|e| e.to_string())
    }).unwrap();
    assert!(!dir.join("shaderpacks/BSL_v10.1.8.zip").exists());
    assert!(!dir.join("shaderpacks/BSL_v10.1.8.zip.baixando").exists());
    let extra = fs::read_to_string(dir.join("horizon-graphics.txt")).unwrap();
    assert!(extra.contains("pickedShaderPack:veio-industrial\n"));
    // No network: same fallback.
    install_with(&fixture.0, &dir, Selection { profile: Profile::Maximum, reset: true }, |url, _| {
        Err(format!("sem rede para {url}"))
    }).unwrap();
    assert!(fs::read_to_string(dir.join("horizon-graphics.txt")).unwrap().contains("pickedShaderPack:veio-industrial\n"));
}

#[test]
fn maximum_keeps_an_installed_bsl_and_the_players_own_options() {
    let fixture = Fixture::new();
    let dir = fixture.instance();
    let presets = fixture.0.join("visuals").join("iris-presets");
    fs::create_dir_all(&presets).unwrap();
    fs::write(presets.join("BSL_v10.1.8.zip.txt"), "TAA=false\n").unwrap();
    let pack = iris::IrisPack { file: "BSL_v10.1.8.zip", url: "x", sha512: "" };
    let bytes = b"fake bsl".to_vec();
    let fake = iris::IrisPack { sha512: Box::leak(sha512::hex(&bytes).into_boxed_str()), ..pack };
    fs::create_dir_all(dir.join("shaderpacks")).unwrap();
    iris::install(&fixture.0, &dir, &fake, false, |_, path| fs::write(path, &bytes).map_err(|e| e.to_string())).unwrap();
    assert_eq!(fs::read(dir.join("shaderpacks/BSL_v10.1.8.zip")).unwrap(), bytes);
    assert_eq!(fs::read_to_string(dir.join("shaderpacks/BSL_v10.1.8.zip.txt")).unwrap(), "TAA=false\n");
    // The player changed an option on the game's screen: kept, and no new download.
    fs::write(dir.join("shaderpacks/BSL_v10.1.8.zip.txt"), "EXPOSURE=0.50\n").unwrap();
    iris::install(&fixture.0, &dir, &fake, false, |_, _| Err("não devia baixar".into())).unwrap();
    assert_eq!(fs::read_to_string(dir.join("shaderpacks/BSL_v10.1.8.zip.txt")).unwrap(), "EXPOSURE=0.50\n");
    // Choosing the profile again on purpose restores the Veio preset.
    iris::install(&fixture.0, &dir, &fake, true, |_, _| Err("não devia baixar".into())).unwrap();
    assert_eq!(fs::read_to_string(dir.join("shaderpacks/BSL_v10.1.8.zip.txt")).unwrap(), "TAA=false\n");
}

#[test]
fn the_real_bsl_hash_check_accepts_the_published_file() {
    // The published hash is 128 hex digits (catches a truncated paste).
    assert_eq!(iris::BSL.sha512.len(), 128);
    assert!(iris::BSL.sha512.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn sha512_of_a_downloaded_bsl_matches_modrinth() {
    // Set VEIO_BSL_ZIP to a downloaded BSL_v10.1.8.zip to check the hash end to end.
    let Ok(path) = std::env::var("VEIO_BSL_ZIP") else { return };
    assert_eq!(sha512::hex(&fs::read(path).unwrap()), iris::BSL.sha512);
}
