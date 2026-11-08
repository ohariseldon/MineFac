//! Instala o pacote visual e prepara somente as opções gráficas da instância.
//! Não carrega o jogo, não altera mundos e não impõe novamente ajustes feitos no jogo.
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::{self, IsTerminal, Write};
use std::path::Path;
use std::process::Command;

/// Pasta em `shaderpacks/`: é também o valor de `pickedShaderPack` que o Horizon procura.
const PACK_FOLDER: &str = "veio-industrial";
// 2: corrige pickedShaderPack (era "horizon:veio_industrial", que o Horizon não acha).
const PRESET_REVISION: &str = "2";
const MARKER: &str = ".veio-visual-profile";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Profile {
    Realism,
    Light,
}

impl Profile {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "realismo" | "realismo-8gb" => Ok(Self::Realism),
            "leve" => Ok(Self::Light),
            _ => Err(format!("visual '{value}' desconhecido. Use --visual realismo ou --visual leve.")),
        }
    }

    fn id(self) -> &'static str {
        match self {
            Self::Realism => "realismo-8gb",
            Self::Light => "leve",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Realism => "Realismo leve — pensado para 8 GB",
            Self::Light => "Leve — sem pacote de shaders",
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct Selection {
    pub profile: Profile,
    // Uma escolha explícita permite restaurar o perfil mesmo depois de ajustes no jogo.
    reset: bool,
}

pub(super) fn choose(args: &[String], config: &mut BTreeMap<String, String>) -> Result<Selection, String> {
    let explicit = args.iter().position(|arg| arg == "--visual")
        .map(|i| args.get(i + 1).ok_or("informe realismo ou leve depois de --visual"))
        .transpose()?.map(|value| Profile::parse(value)).transpose()?;
    let menu = args.iter().any(|arg| arg == "--graphics");
    let saved = config.get("visual_profile").map(|value| Profile::parse(value)).transpose()?;
    let profile = if let Some(profile) = explicit {
        profile
    } else if !menu && saved.is_some() {
        saved.unwrap()
    } else {
        println!("Escolha o visual antes de abrir o jogo:");
        println!("  1. Realismo leve (padrão): iluminação industrial, metais e água; pensado para 8 GB.");
        println!("  2. Leve: sem shaders, para computadores com menos capacidade gráfica.");
        println!("A escolha fica salva. Use Escolher-Visual.cmd para mudar depois.");
        // Execuções sem uma pessoa no terminal nunca precisam carregar o pacote para chegar ao menu.
        if !io::stdin().is_terminal() {
            println!("Sem terminal interativo: usando Leve. Para escolher, use --visual realismo ou --visual leve.");
            Profile::Light
        } else {
            loop {
                print!("Visual [1/2; Enter = 1]: ");
                io::stdout().flush().map_err(|error| error.to_string())?;
                let mut line = String::new();
                if io::stdin().read_line(&mut line).map_err(|error| error.to_string())? == 0 {
                    break Profile::Light;
                }
                match line.trim() {
                    "" | "1" => break Profile::Realism,
                    "2" => break Profile::Light,
                    _ => println!("Digite 1 para Realismo leve ou 2 para Leve."),
                }
            }
        }
    };
    config.insert("visual_profile".into(), profile.id().into());
    println!("Visual: {}.\n", profile.label());
    Ok(Selection { profile, reset: menu || explicit.is_some() })
}

/// Leve também neutraliza um shader forçado por variável de desenvolvimento.
/// Realismo usa a seleção salva do jogo, para continuar permitindo desligar o pacote lá.
pub(super) fn launch_environment(command: &mut Command, profile: Profile) {
    match profile {
        Profile::Light => { command.env("HORIZON_SHADER_PACK", ""); }
        Profile::Realism => { command.env_remove("HORIZON_SHADER_PACK"); }
    }
}

pub(super) fn install(root: &Path, instance: &Path, selection: Selection) -> Result<(), String> {
    // Não há shaders no pacote de conteúdo do mod: desligar aqui remove a seleção sem
    // um gancho nativo voltar a ativá-la. A instalação não executa nenhum WGSL.
    if selection.profile == Profile::Realism {
        let source = root.join("visuals").join(PACK_FOLDER);
        if !source.join("horizon-shaders.toml").is_file() {
            return Err("o pacote visual veio-industrial está faltando. Extraia o ZIP completo, ou abra Jogar-Leve.cmd.".into());
        }
        super::copy_dir(&source, &instance.join("shaderpacks").join(PACK_FOLDER))?;
    }
    let marker = format!("{}:{PRESET_REVISION}", selection.profile.id());
    if !selection.reset && fs::read_to_string(instance.join(MARKER)).unwrap_or_default().trim() == marker {
        println!("      Ajustes visuais da instância preservados.");
        return Ok(());
    }
    // Opções pessoais e futuras continuam intactas. O backup original nunca é sobrescrito.
    let options = instance.join("options.txt");
    let graphics = instance.join("horizon-graphics.txt");
    let mut game = read_options(&options)?;
    let mut extra = read_options(&graphics)?;
    let realism = selection.profile == Profile::Realism;
    let game_values = [
        ("renderDistance", if realism { "8" } else { "6" }),
        ("simulationDistance", "5"),
        ("maxFps", "60"),
        ("graphicsPreset", "fast"),
        ("renderClouds", if realism { "\"fast\"" } else { "\"false\"" }),
        ("cloudRange", "8"),
        ("weatherRadius", "3"),
        ("particles", if realism { "1" } else { "2" }),
        ("ao", "true"),
        ("mipmapLevels", "2"),
        ("entityDistanceScaling", "0.75"),
    ];
    for (key, value) in game_values {
        game = set_option(&game, key, value);
    }
    // Sem LOD, HDR, TAA ou geração de quadros. O único pós-processamento do Realismo
    // inclui sua própria suavização; Leve deixa a imagem no caminho normal do Horizon.
    let extra_values = [
        ("preset", "custom"),
        ("lodDistance", "0"),
        ("textureBudgetMiB", "256"),
        ("workers", "2"),
        ("resolutionScale", if realism { "85" } else { "75" }),
        ("taa", "false"),
        ("fxaa", "false"),
        ("upscaling", "native"),
        ("frameGeneration", "off"),
        ("hdr", "false"),
        ("caveCulling", "true"),
        ("framePacing", "efficient"),
        ("pickedShaderPack", if realism { PACK_FOLDER } else { "" }),
    ];
    for (key, value) in extra_values {
        extra = set_option(&extra, key, value);
    }
    backup(&options)?;
    backup(&graphics)?;
    write_options(&options, &game)?;
    write_options(&graphics, &extra)?;
    write_options(&instance.join(MARKER), &marker)?;
    println!("      Perfil aplicado antes da partida: {}.", selection.profile.label());
    Ok(())
}

fn read_options(path: &Path) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(text),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        Err(error) => Err(format!("não foi possível ler {}: {error}", path.display())),
    }
}

fn backup(path: &Path) -> Result<(), String> {
    if !path.is_file() { return Ok(()); }
    let destination = path.with_file_name(format!("{}.antes-do-veio", path.file_name().unwrap().to_string_lossy()));
    match OpenOptions::new().write(true).create_new(true).open(&destination) {
        Ok(mut file) => {
            let bytes = fs::read(path).map_err(|error| error.to_string())?;
            file.write_all(&bytes).map_err(|error| error.to_string())
        }
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(()),
        Err(error) => Err(format!("não foi possível guardar {}: {error}", destination.display())),
    }
}

fn write_options(path: &Path, text: &str) -> Result<(), String> {
    let temporary = path.with_file_name(format!("{}.veio-new", path.file_name().unwrap().to_string_lossy()));
    fs::write(&temporary, text).and_then(|()| fs::rename(&temporary, path))
        .map_err(|error| format!("não foi possível gravar {}: {error}. Feche o jogo e tente de novo.", path.display()))
}

fn set_option(text: &str, key: &str, value: &str) -> String {
    let mut result = String::new();
    let mut found = false;
    for line in text.lines() {
        if line.split_once(':').is_some_and(|(name, _)| name.trim() == key) {
            if !found { result.push_str(&format!("{key}:{value}\n")); }
            found = true;
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    if !found { result.push_str(&format!("{key}:{value}\n")); }
    result
}

#[cfg(test)]
mod tests;
