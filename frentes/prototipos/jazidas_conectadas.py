"""Experimento de pesquisa; não é carregado pelo jogo. Python 3.11+, sem pacotes.

Execute: python3 frentes/prototipos/jazidas_conectadas.py
Prova conectividade e limite de consultas, não API, geração nem desempenho Horizon.
"""
from collections import deque
from dataclasses import dataclass
import unittest

FACES = ((1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1))
FAMILIES = {
    f"minecraft:{prefix}{ore}_ore": ore
    for ore in ("copper", "iron", "gold", "coal", "redstone", "lapis", "diamond", "emerald")
    for prefix in ("", "deepslate_")
}


@dataclass
class Result:
    count: int
    state: str  # complete, incomplete (fronteira descarregada), saturated (>= limite)
    reads: int


def connected(read, seed, limit=16):
    """read retorna o ID, ou None se a posição não pode ser consultada.

    Não visitar ar/descarregado evita inventar conexões ou carregar chunks.
    A saturação mede um limite inferior, não o tamanho exato da jazida.
    """
    if limit < 1:
        raise ValueError("limit must be positive")
    block = read(seed)
    reads = 1
    if block is None:
        return Result(0, "incomplete", reads)
    family = FAMILIES.get(block)
    if family is None:
        return Result(0, "complete", reads)
    queue, seen = deque([seed]), {seed}
    count, unknown = 1, False
    while queue and count < limit:
        pos = queue.popleft()
        for offset in FACES:
            neighbor = tuple(a + b for a, b in zip(pos, offset))
            if neighbor in seen:
                continue
            seen.add(neighbor)
            other = read(neighbor)
            reads += 1
            if other is None:
                unknown = True
            elif FAMILIES.get(other) == family:
                count += 1
                queue.append(neighbor)
                if count == limit:
                    break
    state = "saturated" if count == limit else "incomplete" if unknown else "complete"
    return Result(count, state, reads)


class ConnectedVeinTests(unittest.TestCase):
    def scan(self, blocks, seed=(0, 0, 0), limit=16, unloaded=None):
        def read(pos):
            if unloaded and unloaded(pos):
                return None
            return blocks.get(pos, "minecraft:air")
        return connected(read, seed, limit)

    def test_a_vertical_and_bent_vein_is_one_deposit(self):
        positions = [(0, 0, 0), (0, -1, 0), (0, -2, 0), (1, -2, 0), (2, -2, 0), (2, -2, 1)]
        result = self.scan(dict.fromkeys(positions, "minecraft:diamond_ore"))
        self.assertEqual((result.count, result.state), (6, "complete"))

    def test_diagonals_and_air_gaps_do_not_connect(self):
        blocks = dict.fromkeys([(0, 0, 0), (1, 1, 0), (2, 0, 0)], "minecraft:diamond_ore")
        self.assertEqual(self.scan(blocks).count, 1)

    def test_normal_and_deepslate_count_as_the_same_resource(self):
        blocks = {(0, 0, 0): "minecraft:diamond_ore", (1, 0, 0): "minecraft:deepslate_diamond_ore"}
        self.assertEqual(self.scan(blocks).count, 2)
        blocks[(2, 0, 0)] = "minecraft:iron_ore"
        self.assertEqual(self.scan(blocks).count, 2)

    def test_large_veins_stop_at_the_useful_limit(self):
        blocks = dict.fromkeys(((x, 0, 0) for x in range(1000)), "minecraft:copper_ore")
        result = self.scan(blocks)
        self.assertEqual((result.count, result.state), (16, "saturated"))
        self.assertLessEqual(result.reads, 1 + 6 * 16)

    def test_unknown_chunk_boundary_is_not_an_exact_small_deposit(self):
        blocks = dict.fromkeys(((x, 0, 0) for x in range(15, 18)), "minecraft:diamond_ore")
        partial = self.scan(blocks, (15, 0, 0), unloaded=lambda p: p[0] >= 16)
        self.assertEqual((partial.count, partial.state), (1, "incomplete"))
        loaded = self.scan(blocks, (15, 0, 0))
        self.assertEqual((loaded.count, loaded.state), (3, "complete"))

    def test_removing_a_bridge_really_splits_the_deposit(self):
        blocks = dict.fromkeys(((x, 0, 0) for x in range(5)), "minecraft:iron_ore")
        self.assertEqual(self.scan(blocks).count, 5)
        del blocks[(2, 0, 0)]
        self.assertEqual(self.scan(blocks).count, 2)

    def test_stone_does_not_trigger_a_world_sized_scan(self):
        result = connected(lambda _: "minecraft:stone", (0, 0, 0))
        self.assertEqual((result.count, result.reads), (0, 1))


if __name__ == "__main__":
    unittest.main(verbosity=2)
