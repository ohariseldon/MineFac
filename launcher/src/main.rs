//! launcher.exe do Veio.
//!
//! Fica na pasta do pacote, ao lado de `content/` e `native/`. Ao abrir:
//! 1. encontra o Horizon Launcher e a pasta `horizon-mods` da instância (guarda em `launcher.ini`);
//! 2. compila o código nativo com `horizon-launcher.exe native-build` quando a versão do pacote
//!    muda (ou com `--rebuild`);
//! 3. copia `content/` para `horizon-mods/veio`;
//! 4. abre o jogo com `horizon-launcher play --instance` (ou o Horizon Launcher, se falhar).
//!
//! Se a instância não existir, ela é criada com `horizon-launcher instance create`. As instâncias
//! ficam em `%APPDATA%\\horizon\\instances\\<nome>` (ou `HORIZON_HOME`), conforme a documentação
//! do launcher em github.com/horizon-rust/horizon/blob/main/launcher/README.md.
//!
//! Só usa a biblioteca padrão. Não baixa nada e não toca em mundos ou contas.
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const CONFIG: &str = "launcher.ini";
/// Arquivo gravado na pasta do mod depois de uma compilação bem-sucedida.
const STAMP: &str = ".veio-native-build";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = run(&args);
    if let Err(message) = &result {
        eprintln!("\nERRO: {message}");
    }
    if !args.iter().any(|arg| arg == "--no-pause") {
        pause();
    }
    if result.is_ok() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}

fn run(args: &[String]) -> Result<(), String> {
    let root = package_root()?;
    let version = read_version(&root.join("content/horizon-mod.toml"))?;
    println!("Veio {version}");
    println!("Pacote: {}\n", root.display());

    let config_path = root.join(CONFIG);
    let mut config = read_config(&config_path);
    let horizon = horizon_launcher(&mut config)?;
    let instance = config.entry("instance".into()).or_insert_with(|| "Veio".into()).clone();
    let mods = mods_dir(&mut config, &horizon, &instance)?;
    write_config(&config_path, &config)?;

    let target = mods.join("veio");
    let built = fs::read_to_string(target.join(STAMP)).unwrap_or_default();
    let rebuild = args.iter().any(|arg| arg == "--rebuild") || built.trim() != version;
    let skip = args.iter().any(|arg| arg == "--skip-build");
    if skip {
        println!("[1/3] Compilação pulada (--skip-build).");
    } else if rebuild {
        println!("[1/3] Compilando o código nativo do Veio {version} na instância {instance}.");
        println!("      Na primeira vez o Horizon baixa o Rust: leva de 20 a 30 minutos e alguns GB.");
        let status = Command::new(&horizon)
            .arg("native-build").arg(root.join("native"))
            .args(["--instance", &instance, "--trust-native-code", "--profile", "dev"])
            .status()
            .map_err(|error| format!("não foi possível executar {}: {error}", horizon.display()))?;
        if !status.success() {
            return Err(format!("a compilação nativa falhou ({status}). Veja as mensagens acima. Nada foi instalado."));
        }
    } else {
        println!("[1/3] Código nativo já compilado para {version}. Use --rebuild para compilar de novo.");
    }

    println!("[2/3] Instalando o conteúdo em {}", target.display());
    install(&root.join("content"), &target)?;
    // A marca só vale para uma compilação desta versão (feita agora ou antes).
    if !skip || !rebuild {
        fs::write(target.join(STAMP), &version).map_err(|error| format!("não foi possível gravar {STAMP}: {error}"))?;
    }

    println!("[3/3] Abrindo o jogo na instância {instance}.");
    println!("      Na primeira partida, ative o Veio em Opções > Mods (mods de conteúdo começam desligados)");
    println!("      e reinicie o jogo. Se o jogo não abrir, use o Horizon Launcher: Prepare, conta e Play.");
    let started = Command::new(&horizon).args(["play", "--instance", &instance]).args(config_args(&config)).status()
        .map(|status| status.success()).unwrap_or(false);
    if !started {
        println!("      O jogo não abriu pela linha de comando; abrindo o Horizon Launcher.");
        Command::new(&horizon).spawn()
            .map_err(|error| format!("não foi possível abrir {}: {error}", horizon.display()))?;
    }
    Ok(())
}

/// Pasta do pacote: a do executável, ou uma acima (para `launcher/target/...` no repositório).
fn package_root() -> Result<PathBuf, String> {
    let exe = std::env::current_exe().map_err(|error| error.to_string())?;
    let mut dir = exe.parent().map(Path::to_path_buf);
    while let Some(candidate) = dir {
        if candidate.join("content/horizon-mod.toml").is_file() && candidate.join("native").is_dir() {
            return Ok(candidate);
        }
        dir = candidate.parent().map(Path::to_path_buf);
    }
    Err("não encontrei as pastas content/ e native/ junto do launcher.exe. Mantenha o pacote completo na mesma pasta.".into())
}

fn read_version(manifest: &Path) -> Result<String, String> {
    let text = fs::read_to_string(manifest).map_err(|error| format!("{}: {error}", manifest.display()))?;
    text.lines()
        .find_map(|line| line.trim().strip_prefix("version").map(|rest| rest.trim_start_matches([' ', '=']).trim_matches('"').to_owned()))
        .ok_or_else(|| format!("{} sem versão", manifest.display()))
}

fn read_config(path: &Path) -> BTreeMap<String, String> {
    fs::read_to_string(path).unwrap_or_default().lines()
        .filter(|line| !line.trim_start().starts_with(['#', ';']))
        .filter_map(|line| line.split_once('='))
        .map(|(key, value)| (key.trim().to_owned(), value.trim().to_owned()))
        .filter(|(_, value)| !value.is_empty())
        .collect()
}

fn write_config(path: &Path, config: &BTreeMap<String, String>) -> Result<(), String> {
    let mut text = String::from(
        "# Configuração do launcher do Veio. Apague uma linha para ela ser perguntada de novo.\n\
         # horizon_launcher: caminho do horizon-launcher.exe\n\
         # instance: nome da instância no Horizon Launcher\n\
         # mods_dir: pasta horizon-mods da instância\n\
         # launch_args: argumentos extras ao abrir o Horizon Launcher (opcional)\n");
    for key in ["horizon_launcher", "instance", "mods_dir", "launch_args"] {
        text.push_str(&format!("{key}={}\n", config.get(key).map_or("", String::as_str)));
    }
    fs::write(path, text).map_err(|error| format!("não foi possível gravar {}: {error}", path.display()))
}

fn config_args(config: &BTreeMap<String, String>) -> Vec<String> {
    config.get("launch_args").map(|args| args.split_whitespace().map(str::to_owned).collect()).unwrap_or_default()
}

fn horizon_launcher(config: &mut BTreeMap<String, String>) -> Result<PathBuf, String> {
    if let Some(path) = config.get("horizon_launcher").map(PathBuf::from).filter(|path| path.is_file()) {
        return Ok(path);
    }
    let guess = std::env::var_os("LOCALAPPDATA")
        .map(|base| PathBuf::from(base).join("Programs").join("Horizon Launcher").join("horizon-launcher.exe"))
        .filter(|path| path.is_file());
    let path = match guess {
        Some(path) => path,
        None => {
            let answer = ask("Não encontrei o Horizon Launcher. Cole o caminho completo do horizon-launcher.exe")?;
            let path = PathBuf::from(answer.trim_matches('"'));
            if !path.is_file() {
                return Err(format!("{} não existe", path.display()));
            }
            path
        }
    };
    config.insert("horizon_launcher".into(), path.display().to_string());
    Ok(path)
}

/// Pasta do jogo do Horizon: `HORIZON_HOME` ou `%APPDATA%\\horizon`.
fn game_dir() -> Option<PathBuf> {
    std::env::var_os("HORIZON_HOME").filter(|value| !value.is_empty()).map(PathBuf::from)
        .or_else(|| std::env::var_os("APPDATA").map(|base| PathBuf::from(base).join("horizon")))
}

/// Pasta `horizon-mods` da instância. Cria a instância pelo Horizon Launcher se ela não existir.
fn mods_dir(config: &mut BTreeMap<String, String>, horizon: &Path, instance: &str) -> Result<PathBuf, String> {
    if let Some(path) = config.get("mods_dir").map(PathBuf::from).filter(|path| path.is_dir()) {
        return Ok(path);
    }
    if let Some(game) = game_dir() {
        let folder = game.join("instances").join(instance);
        if !folder.is_dir() {
            println!("A instância {instance} não existe. Criando pelo Horizon Launcher...");
            let created = Command::new(horizon).args(["instance", "create", instance]).status()
                .map(|status| status.success()).unwrap_or(false);
            if !created {
                println!("Não foi possível criar a instância automaticamente.");
            }
        }
        if folder.is_dir() {
            let path = folder.join("horizon-mods");
            fs::create_dir_all(&path).map_err(|error| format!("{}: {error}", path.display()))?;
            println!("Instância {instance}: {}", folder.display());
            config.insert("mods_dir".into(), path.display().to_string());
            return Ok(path);
        }
    }
    println!("Não encontrei a pasta da instância {instance}.");
    println!("No Horizon Launcher: clique em + ao lado de Instances, crie uma chamada {instance},");
    println!("depois use Open folder na instância. A pasta horizon-mods fica dentro dela (crie se faltar).");
    let answer = ask("Cole o caminho completo da pasta horizon-mods da instância")?;
    let path = PathBuf::from(answer.trim_matches('"'));
    if path.file_name().is_none_or(|name| !name.eq_ignore_ascii_case("horizon-mods")) {
        return Err(format!("{} não é uma pasta horizon-mods", path.display()));
    }
    fs::create_dir_all(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    config.insert("mods_dir".into(), path.display().to_string());
    Ok(path)
}

/// Substitui `target` pelo conteúdo do pacote. Só apaga uma pasta que já seja o mod Veio.
fn install(content: &Path, target: &Path) -> Result<(), String> {
    if target.exists() {
        let manifest = fs::read_to_string(target.join("horizon-mod.toml")).unwrap_or_default();
        let is_veio = manifest.lines().any(|line| line.replace(' ', "") == "id=\"veio\"");
        if !is_veio {
            return Err(format!("{} existe, mas não é o mod Veio. Remova ou renomeie essa pasta e tente de novo.", target.display()));
        }
        fs::remove_dir_all(target).map_err(|error| format!(
            "não foi possível substituir {}: {error}. Feche o jogo e tente de novo.", target.display()))?;
    }
    copy_dir(content, target)
}

fn copy_dir(from: &Path, to: &Path) -> Result<(), String> {
    fs::create_dir_all(to).map_err(|error| format!("{}: {error}", to.display()))?;
    for entry in fs::read_dir(from).map_err(|error| format!("{}: {error}", from.display()))? {
        let entry = entry.map_err(|error| error.to_string())?;
        let destination = to.join(entry.file_name());
        if entry.file_type().map_err(|error| error.to_string())?.is_dir() {
            copy_dir(&entry.path(), &destination)?;
        } else {
            fs::copy(entry.path(), &destination).map_err(|error| format!("{}: {error}", destination.display()))?;
        }
    }
    Ok(())
}

fn ask(question: &str) -> Result<String, String> {
    print!("{question}:\n> ");
    io::stdout().flush().ok();
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).map_err(|error| error.to_string())?;
    let line = line.trim().to_owned();
    if line.is_empty() { Err("nenhum caminho informado".into()) } else { Ok(line) }
}

fn pause() {
    print!("\nPressione Enter para fechar...");
    io::stdout().flush().ok();
    let _ = io::stdin().lock().read_line(&mut String::new());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_version_and_config() {
        let dir = std::env::temp_dir().join(format!("veio-launcher-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let manifest = dir.join("horizon-mod.toml");
        fs::write(&manifest, "id = \"veio\"\nversion = \"0.10.0\"\n").unwrap();
        assert_eq!(read_version(&manifest).unwrap(), "0.10.0");
        let ini = dir.join(CONFIG);
        let mut config = BTreeMap::from([("instance".to_owned(), "Veio".to_owned())]);
        config.insert("launch_args".into(), "--a b".into());
        write_config(&ini, &config).unwrap();
        let read = read_config(&ini);
        assert_eq!(read.get("instance").map(String::as_str), Some("Veio"));
        assert_eq!(config_args(&read), ["--a", "b"]);
        assert!(!read.contains_key("mods_dir"));
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn install_refuses_to_replace_a_foreign_folder() {
        let dir = std::env::temp_dir().join(format!("veio-launcher-install-{}", std::process::id()));
        let content = dir.join("content");
        let target = dir.join("horizon-mods/veio");
        fs::create_dir_all(content.join("assets")).unwrap();
        fs::write(content.join("horizon-mod.toml"), "id = \"veio\"\n").unwrap();
        fs::write(content.join("assets/a.json"), "{}").unwrap();
        fs::create_dir_all(&target).unwrap();
        fs::write(target.join("horizon-mod.toml"), "id = \"outro\"\n").unwrap();
        assert!(install(&content, &target).is_err());
        assert!(target.join("horizon-mod.toml").exists(), "pasta alheia preservada");
        fs::write(target.join("horizon-mod.toml"), "id = \"veio\"\n").unwrap();
        fs::write(target.join("velho.json"), "{}").unwrap();
        install(&content, &target).unwrap();
        assert!(target.join("assets/a.json").exists());
        assert!(!target.join("velho.json").exists(), "arquivos removidos do pacote saem da instância");
        fs::remove_dir_all(dir).unwrap();
    }
}
