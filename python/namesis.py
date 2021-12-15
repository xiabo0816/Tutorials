import os

# 1. 全局变量
# 2. 引入依赖：import
# 3. 函数定义：def
# 4. 数据类型：0，""，[]，{}
# 5. 语法糖
# 6. IDE：pycharm、vscode

# du -a 市级政协 | awk '{print $2}' | grep .xlsx > input.list
# 文件编码
FILE_ENCODE = 'UTF-8'

def readlist(path):
    if (path == ''):
        return []
    lines = [line.strip() for line in open(path, 'r', encoding=FILE_ENCODE).readlines() if not line.startswith("#")]
    return lines

FILELIST = readlist("input.list")

if __name__ == '__main__':
    result = {}
    for path in FILELIST:
        dirname, basename = os.path.split(path) 
        if basename not in result:
            result[basename] = []
        result[basename].append(dirname)
    
    dd = [(rr[:-5], len(result[rr])) for rr in result.keys()]
    print(sorted(dd, key=lambda item:item[1], reverse=True))
