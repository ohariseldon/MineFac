//! Clima: estações do ano e temperatura do ar, calculadas do relógio do mundo e da altitude.
//!
//! Nada aqui guarda estado nem varre o mundo: são contas sobre `day_time` (que continua contando
//! depois de 24.000, então cada dia novo avança a estação), a altura do bloco e a chuva. Quem usa
//! o clima (geração de energia, lavoura, rios e o pacote de shaders) relê nos próprios intervalos
//! de sensor, a cada 2 segundos, não por tick.
//!
//! Escolhas de realismo, em escala de jogo:
//! - o ano tem quatro estações de `SEASON_DAYS` dias, começando na primavera;
//! - a temperatura segue a estação, o ciclo do dia (mínima ao nascer do sol, máxima no meio da
//!   tarde), cai 1 °C a cada 10 blocos acima do mar (gradiente da atmosfera, exagerado como as
//!   montanhas do jogo) e 3 °C na chuva;
//! - debaixo da terra o ar fica estável perto de 12 °C e esquenta com a profundidade;
//! - o Nether é sempre quente e o End, frio e sem estações.

/// Dias de jogo de cada estação: um ano de 24 dias, cerca de 8 horas de jogo.
pub(crate) const SEASON_DAYS: i64 = 6;
const DAY: i64 = 24000;

pub(crate) const SPRING: usize = 0;
pub(crate) const SUMMER: usize = 1;
pub(crate) const AUTUMN: usize = 2;
pub(crate) const WINTER: usize = 3;

/// Fase do ano, 0..4: a parte inteira é a estação, a fração quanto dela passou.
pub(crate) fn season_phase(day_time: i64) -> f32 {
    let year = SEASON_DAYS * 4 * DAY;
    let into = day_time.rem_euclid(year);
    (into as f64 / (SEASON_DAYS * DAY) as f64) as f32
}

pub(crate) fn season(day_time: i64) -> usize {
    (season_phase(day_time) as usize).min(3)
}

/// Dia da estação, a partir de 1.
pub(crate) fn season_day(day_time: i64) -> i64 {
    day_time.rem_euclid(SEASON_DAYS * DAY) / DAY + 1
}

/// Quanto de cada estação aparece agora (primavera, verão, outono, inverno), somando 1, com a
/// troca suave nas viradas: a mesma conta do pacote de shaders (`veio_season_weights`).
pub(crate) fn season_weights(day_time: i64) -> [f32; 4] {
    let phase = season_phase(day_time);
    let mut w = [0.0f32; 4];
    for (i, slot) in w.iter_mut().enumerate() {
        let d = (phase - (i as f32 + 0.5)).abs();
        let wrapped = d.min(4.0 - d);
        let t = ((wrapped - 0.35) / 0.30).clamp(0.0, 1.0);
        *slot = 1.0 - t * t * (3.0 - 2.0 * t);
    }
    let sum: f32 = w.iter().sum();
    w.map(|x| x / sum.max(0.001))
}

/// Temperatura média de cada estação ao nível do mar, em °C, interpolada entre os meios das
/// estações para não haver salto na virada.
fn seasonal_mean(phase: f32) -> f32 {
    const MEAN: [f32; 4] = [15.0, 25.0, 13.0, 1.0];
    let shifted = (phase - 0.5).rem_euclid(4.0);
    let index = shifted.floor() as usize % 4;
    let t = shifted - shifted.floor();
    let t = t * t * (3.0 - 2.0 * t);
    MEAN[index] + (MEAN[(index + 1) % 4] - MEAN[index]) * t
}

/// Temperatura do ar em °C num bloco de altura `y`, com chuva `rain` (0..1).
pub(crate) fn temperature(day_time: i64, y: i32, rain: f32, dimension: &str) -> f32 {
    match dimension {
        "minecraft:the_nether" => return 48.0,
        "minecraft:the_end" => return -8.0,
        _ => {}
    }
    if y < 50 {
        // Subsolo: estável, um grau a cada 25 blocos de profundidade abaixo de y 50.
        return 12.0 + (50 - y) as f32 / 25.0;
    }
    let phase = season_phase(day_time);
    // Ciclo do dia: 0 é o nascer do sol, a máxima vem às 9.000 (meio da tarde).
    let hour = day_time.rem_euclid(DAY) as f32 / DAY as f32;
    let daily = (std::f32::consts::TAU * (hour - 0.375 + 0.25)).sin();
    let swing = 5.5;
    let altitude = (y - 64).max(0) as f32 / 10.0;
    seasonal_mean(phase) + swing * daily - altitude - 3.0 * rain.clamp(0.0, 1.0)
}

/// Ganho do painel solar pela estação (insolação: dias mais longos e sol mais alto no verão)
/// e pela temperatura do painel (−0,4% por °C acima de 25 °C, como o silício).
pub(crate) fn solar_factor(day_time: i64, temperature: f32) -> f32 {
    let season = [1.0, 1.18, 0.85, 0.62][season(day_time)];
    let heat = 1.0 - 0.004 * (temperature - 25.0);
    season * heat.clamp(0.85, 1.1)
}

/// Ventos mais fortes na metade fria do ano.
pub(crate) fn wind_factor(day_time: i64) -> f32 {
    [1.0, 0.85, 1.1, 1.2][season(day_time)]
}

/// Vazão dos rios pela estação: cheia do degelo na primavera, seca no fim do verão.
pub(crate) fn river_factor(day_time: i64) -> f32 {
    [1.35, 0.75, 1.0, 0.85][season(day_time)]
}

/// Abaixo disto a água da roda congela e a roda para.
pub(crate) const FREEZE: f32 = -3.0;

/// Valor que os painéis recebem: estação × 1.000 + temperatura + 100, sempre positivo.
pub(crate) fn pack(day_time: i64, temperature: f32) -> i32 {
    season(day_time) as i32 * 1000 + (temperature.round() as i32 + 100).clamp(0, 999)
}

/// O inverso de `pack`: (estação, °C).
pub(crate) fn unpack(value: i32) -> (usize, i32) {
    ((value / 1000).clamp(0, 3) as usize, value % 1000 - 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_year_runs_through_four_seasons_and_wraps() {
        assert_eq!(season(0), SPRING);
        assert_eq!(season(SEASON_DAYS * DAY), SUMMER);
        assert_eq!(season(2 * SEASON_DAYS * DAY + 5), AUTUMN);
        assert_eq!(season(3 * SEASON_DAYS * DAY), WINTER);
        assert_eq!(season(4 * SEASON_DAYS * DAY), SPRING);
        assert_eq!(season(-1), WINTER, "/time set negativo continua no ano anterior");
        assert_eq!(season_day(DAY * 2 + 10), 3);
    }

    #[test]
    fn temperature_follows_season_hour_altitude_rain_and_depth() {
        let summer_afternoon = SEASON_DAYS * DAY + 3 * DAY + 9000;
        let winter_dawn = 3 * SEASON_DAYS * DAY + DAY * 3;
        let warm = temperature(summer_afternoon, 64, 0.0, "minecraft:overworld");
        let cold = temperature(winter_dawn, 64, 0.0, "minecraft:overworld");
        assert!(warm > 25.0 && warm < 35.0, "{warm}");
        assert!(cold < 0.0, "{cold}");
        let peak = temperature(summer_afternoon, 164, 0.0, "minecraft:overworld");
        assert!((warm - peak - 10.0).abs() < 0.01, "10 graus por 100 blocos");
        let wet = temperature(summer_afternoon, 64, 1.0, "minecraft:overworld");
        assert!((warm - wet - 3.0).abs() < 0.01);
        assert_eq!(temperature(winter_dawn, 0, 0.0, "minecraft:overworld"), 14.0, "subsolo estável");
        assert_eq!(temperature(0, 64, 0.0, "minecraft:the_nether"), 48.0);
    }

    #[test]
    fn seasons_change_without_a_jump() {
        let border = SEASON_DAYS * DAY;
        let before = temperature(border - 1, 64, 0.0, "minecraft:overworld");
        let after = temperature(border, 64, 0.0, "minecraft:overworld");
        assert!((before - after).abs() < 0.1);
    }

    #[test]
    fn energy_and_water_respond_to_the_seasons() {
        let summer = SEASON_DAYS * DAY;
        let winter = 3 * SEASON_DAYS * DAY;
        assert!(solar_factor(summer, 25.0) > solar_factor(winter, 25.0));
        assert!(solar_factor(summer, 40.0) < solar_factor(summer, 25.0), "painel quente rende menos");
        assert!(wind_factor(winter) > wind_factor(summer));
        assert!(river_factor(0) > river_factor(summer), "cheia na primavera");
    }

    #[test]
    fn season_weights_sum_to_one_and_peak_mid_season() {
        for day in (0..SEASON_DAYS * 4 * DAY).step_by(5000) {
            let w = season_weights(day);
            assert!((w.iter().sum::<f32>() - 1.0).abs() < 0.001);
        }
        let mid_autumn = season_weights(2 * SEASON_DAYS * DAY + SEASON_DAYS * DAY / 2);
        assert!(mid_autumn[AUTUMN] > 0.99);
    }

    #[test]
    fn panel_value_round_trips() {
        for (day, temp) in [(0, 15.4), (SEASON_DAYS * DAY * 3, -12.0), (SEASON_DAYS * DAY, 48.0)] {
            let (season_index, celsius) = unpack(pack(day, temp));
            assert_eq!(season_index, season(day));
            assert_eq!(celsius, temp.round() as i32);
        }
    }
}
