# 直接通过已分词语料训练词向量
from gensim.models import KeyedVectors
import gensim

# 构建数据集
sentences = gensim.models.word2vec.LineSentence('odc.fasttext.utf8')
# sentences = gensim.models.word2vec.LineSentence('/home/xiabo/yucang/writing-server/gwcla/data/mainclass.train')
# 创建词向量对象
model = gensim.models.Word2Vec(sentences, min_count=3, workers=1)

# 生成语料词表
model.build_vocab(sentences)

# 训练
model.train(sentences, total_examples=model.corpus_count, epochs=model.epochs) 

# 查看训练结果
model.wv['改革']

# 计算两个词的相似度
print(model.wv.similarity('改革', '体制改革'))
print(model.wv.similarity('改革', '的'))

# 列出全部的相似词
print(model.wv.similar_by_word('改革'))

# 保存训练好的模型
model.save('od.gensim.wv.model')
model.wv.save_word2vec_format('od.word2vec.wv.model', binary=False)
#加载模型
# from gensim.models import KeyedVectors
# model= KeyedVectors.load('od.gensim.wv.model')
# model.wv.get_vector(word)
# model.wv.similarity('改革', '开放')
# model.wv.similarity('改革', '一')
