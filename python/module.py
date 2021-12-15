# 1. 自己的包直接引入
# 2. 第三方包安装
#    pip3 -V
#    pip3 install jieba

import namesis

print(namesis.FILELIST)

# 分词
import jieba
for file in namesis.FILELIST:
    print(list(jieba.cut(file)))
    # file = re.sub(r"[\Wa-z]","",file)
    # print(list(jieba.cut(file)))
