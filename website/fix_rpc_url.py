import os, glob

for file_path in glob.glob('website/*.html'):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    if 'fetch("/rpc/"' in content:
        content = content.replace('fetch("/rpc/"', 'fetch("https://kasturisundari.xyz/rpc/"')
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f'Updated {file_path}')
