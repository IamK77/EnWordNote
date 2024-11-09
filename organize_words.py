import os
import yaml
import shutil

# 删除之前的 word 文件夹
if os.path.exists('word'):
    shutil.rmtree('word')
os.makedirs('word', exist_ok=True)

# 遍历所有的 .yaml 文件
for root, dirs, files in os.walk('.'):  # 遍历多级目录
    for file in files:
        if file.endswith('.yaml'):
            file_path = os.path.join(root, file)
            with open(file_path, 'r', encoding='utf-8') as f:
                try:
                    content = yaml.safe_load(f)
                    if content:
                        for word in content.keys():
                            first_letter = word[0].lower()
                            target_file = os.path.join('word', f'{first_letter}.yaml')
                            
                            # 将单词及其内容追加到对应的目标文件中
                            with open(target_file, 'a', encoding='utf-8') as wf:
                                yaml.dump({word: content[word]}, wf, allow_unicode=True)
                                wf.write('\n')
                except yaml.YAMLError as e:
                    print(f"Error parsing {file_path}: {e}")
