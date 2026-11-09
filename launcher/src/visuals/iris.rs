//! Realismo máximo: um shader Iris/OptiFine famoso (o BSL), baixado do Modrinth no PC de quem
//! joga e nunca dentro do zip do Veio, com o ajuste "Veio 8 GB" de `visuals/iris-presets/`.
//! O Horizon roda packs Iris: `pickedShaderPack` é o nome do zip e as opções ficam em
//! `shaderpacks/<zip>.txt`, as mesmas que a tela Opções do Shader mostra e grava.
use std::fs;
use std::path::Path;
use std::process::Command;

use super::sha512;

pub(super) struct IrisPack {
    pub file: &'static str,
    pub url: &'static str,
    pub sha512: &'static str,
}

/// BSL 10.1.8 (Capt Tatsu), versão para 26.3 no Modrinth.
pub(super) const BSL: IrisPack = IrisPack {
    file: "BSL_v10.1.8.zip",
    url: "https://cdn.modrinth.com/data/Q1vvjJYV/versions/Y68yiql9/BSL_v10.1.8.zip",
    sha512: "94a3c25d6820868db624e11347873e6b572e7cd571967d9ac6f4690ca5d37ca302a7645d85eec800e0654541e3825fba14d37b5a25d3996aaf038178552be26f",
};

/// Baixa com o curl do sistema (vem no Windows 10 e 11). Sem dependências no launcher.
pub(super) fn curl(url: &str, destination: &Path) -> Result<(), String> {
    let program = if cfg!(windows) { "curl.exe" } else { "curl" };
    let status = Command::new(program)
        .args(["-fsSL", "--retry", "3", "-o"])
        .arg(destination)
        .arg(url)
        .status()
        .map_err(|error| format!("não foi possível rodar {program}: {error}"))?;
    if status.success() { Ok(()) } else { Err(format!("o download de {url} falhou ({status})")) }
}

/// Garante o pack em `shaderpacks/` (baixa se faltar ou não conferir) e põe o ajuste do Veio
/// ao lado, sem apagar um ajuste que a pessoa fez na tela do jogo, a não ser que `reset`.
pub(super) fn install(
    root: &Path,
    instance: &Path,
    pack: &IrisPack,
    reset: bool,
    fetch: impl Fn(&str, &Path) -> Result<(), String>,
) -> Result<(), String> {
    let folder = instance.join("shaderpacks");
    fs::create_dir_all(&folder).map_err(|error| format!("{}: {error}", folder.display()))?;
    let target = folder.join(pack.file);
    let ok = |path: &Path| fs::read(path).is_ok_and(|bytes| sha512::hex(&bytes) == pack.sha512);
    if !ok(&target) {
        println!("      Baixando {} do Modrinth (uma vez só)...", pack.file);
        let partial = folder.join(format!("{}.baixando", pack.file));
        let result = fetch(pack.url, &partial);
        if result.is_err() || !ok(&partial) {
            let _ = fs::remove_file(&partial);
            return Err(result.err().unwrap_or_else(|| format!("{} não confere com o hash do Modrinth", pack.file)));
        }
        fs::rename(&partial, &target).map_err(|error| format!("{}: {error}", target.display()))?;
    }
    let preset = root.join("visuals").join("iris-presets").join(format!("{}.txt", pack.file));
    let options = folder.join(format!("{}.txt", pack.file));
    if preset.is_file() && (reset || !options.exists()) {
        fs::copy(&preset, &options).map_err(|error| format!("{}: {error}", options.display()))?;
    }
    Ok(())
}
