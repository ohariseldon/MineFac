// Gerado por tools/build_town.py. Não edite à mão.
// Posições relativas ao centro da praça, com y = 0 no primeiro bloco de ar acima do chão.

/// Meio lado da cidade: o perímetro fica em |x| e |z| iguais a isto.
pub(crate) const HALF: i32 = 44;
/// Quantos aldeões a cidade tem ao nascer.
pub(crate) const STARTING_VILLAGERS: usize = 20;
/// Camas (cabeceiras) das casas da cidade.
#[cfg(test)]
pub(crate) const BEDS: usize = 28;
/// Canto noroeste do lote do fundador, no nível do piso (o chão).
pub(crate) const FOUNDER_LOT: [i32; 3] = [-7, -1, -23];
/// Onde o casal fundador se recupera quando cai (o lote da prefeitura).
pub(crate) const REFUGE: [i32; 3] = [-16, 0, -16];
/// Centro do lote da oficina (distrito industrial).
pub(crate) const WORKSHOP: [i32; 3] = [37, 0, 11];
/// Por onde chegam os imigrantes (dentro do portão sul).
pub(crate) const GATE: [i32; 3] = [0, 0, 40];
/// Lugares dos moradores dentro das casas, do centro para fora.
pub(crate) const SPOTS: [[i32; 3]; 29] = [
    [-12, 0, 7], [12, 4, 7], [-14, 0, 7], [14, 4, 7], [-6, 0, 18], [6, 0, 18],
    [-8, 0, 18], [8, 0, 18], [-21, 0, 7], [21, 4, 7], [-23, 0, 7], [23, 4, 7],
    [-16, 0, 18], [16, 0, 18], [-18, 0, 18], [18, 0, 18], [-36, 0, -6], [-36, 0, 6],
    [-36, 0, -8], [-36, 0, 8], [15, 4, -33], [17, 4, -33], [-36, 0, -16], [-36, 0, 16],
    [-36, 0, -18], [-36, 0, 18], [36, 0, 33], [35, 4, -35], [35, 4, -37],
];
