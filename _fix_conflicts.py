import os, re

files_with_conflicts = []
for root, dirs, files in os.walk('.'):
    if '.git' in root:
        continue
    for f in files:
        if f.endswith(('.rs', '.toml', '.yml', '.yaml', '.json', '.md')):
            path = os.path.join(root, f)
            try:
                content = open(path, 'r', encoding='utf-8', errors='ignore').read()
                if '<<<<<<< HEAD' in content:
                    files_with_conflicts.append(path)
            except:
                pass

print(f'Found {len(files_with_conflicts)} files with conflicts')

for path in files_with_conflicts:
    content = open(path, 'r', encoding='utf-8', errors='ignore').read()
    # Resolve by keeping HEAD side
    resolved = re.sub(
        r'<<<<<<< HEAD\n(.*?)=======\n.*?>>>>>>> [^\n]+\n',
        r'\1',
        content,
        flags=re.DOTALL
    )
    if '<<<<<<< HEAD' not in resolved:
        open(path, 'w', encoding='utf-8').write(resolved)
        print(f'  FIXED: {path}')
    else:
        print(f'  FAILED: {path}')
