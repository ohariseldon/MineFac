from pathlib import Path
import shutil
root = Path(__file__).resolve().parents[1]
destination = root / '.build/world-map-bundles/veio'
destination.mkdir(parents=True, exist_ok=True)
manifest = (root/'work-in-progress/finite-world/horizon-mixin.toml.draft').read_text(encoding='utf-8')
manifest = manifest.replace('[api]\npath = "api"\n', '')
(destination/'horizon-mixin.toml').write_text(manifest, encoding='utf-8')
for source in (root/'work-in-progress/finite-world').glob('*.rs'):
    shutil.copy2(source, destination/source.name)
(destination/'hooks').mkdir(exist_ok=True)
for source in (root/'work-in-progress/finite-world/hooks').glob('*.rs'):
    shutil.copy2(source,destination/'hooks'/source.name)
print(destination)
