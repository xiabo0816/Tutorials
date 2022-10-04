分类任务的评价计算方式

除了Classification任务的评价，一般还有Clustering评价计算方式、Regression评价计算方式，这里只说Classification任务

# 准确率

评价一个分类任务是否做对了，一般是通过确定什么是做“对”了，什么是做“错”了

这种“对错”、“是否”问题，可以认为是二分类任务。

对应的，多分类任务就是一次判断“对错”不够，需要多次判断“对错”。

直观来说，就用判断题来做比喻，做十道选择题，准确率就是`做对的`➗`全部的`

$$准确率 = \frac{做对的}{全部的}$$


# 混淆矩阵

这时候我们注意到，`全部的(Total)`也是粗略的表达，讨论一下，可能包含的情况有几种，也就得到了`混淆矩阵（Confusion Matrix）`：

1. 学生画`勾`的做`对`了，`做对的Positive√√`，TP，真正例，真实类别为正例，预测类别为正例
2. 学生画`勾`的做`错`了，`做错的Positive√×`，FP，漏报，真实类别为负例，预测类别为正例，答题时错的画勾了
3. 学生画`叉`的做`错`了，`做错的Negative××`，FN，误报，真实类别为正例，预测类别为负例，答题时对的画叉了
4. 学生画`叉`的做`对`了，`做对的Negative×√`，TN，真负例，真实类别为负例，预测类别为负例

|     |  预测正例   | 预测负例   |
|  :----: |  :----: | :----: |
| 真实正例  | TP  | TN |
| 真实负例  | FP  | FN |

这样一来，一道判断题`做对了`，也就是准确率，其实包含了两层含义，也就是`画勾的做对了√√`和`画叉的做对了×√`

`做对的`就同时包含了`做对的Positive√√`和`做对的Negative×√`，所以：

$$准确率(Accuracy) = \frac{做对的}{全部的} = \frac{做对的Positive√√+做对的Negative×√}{全部的} = \frac{TruePositive+TrueNegative}{Total}$$

$$准确率(Accuracy) = \frac{做对的}{全部的} = \frac{TruePositive+FalseNegative}{Total}$$

```python
from sklearn.metrics import confusion_matrix
y_pred = [0,1,0,1]
y_true = [0,1,1,1]
confusion_matrix(y_true, y_pred, labels=[1, 0])
```

# 查准率与查全率

`查准率`（精度，Precision）是衡量某一检索系统的信号噪声比的一种指标，即检出的相关文献与检出的全部文献的百分比。普遍表示为：查准率=（检索出的相关信息量/检索出的信息总量）x100%。

按照混淆矩阵的定义，`查准率`可表示为：

$$查准率(Precision) = \frac{检索出的相关信息量}{检索出的信息总量} = \frac{做对的Positive√√}{做对的Positive√√+做错的Positive√×} = \frac{TruePositive}{TruePositive+FalsePositive} = \frac{TP}{TP+FP}$$

$$查准率(Precision) = \frac{TP}{TP+FP}$$


即预测是正例的结果中，确实是正例的比例

分母为预测为正样例的个数 ；分子为预测为实际正样例被预测准的个数

```python
from sklearn import metrics
y_pred = [0,1,0,1]
y_true = [0,1,1,1]
print('Precision', metrics.precision_score(y_true,y_pred))
```

`查全率`（召回率，Recall），是衡量某一检索系统从文献集合中检出相关文献成功度的一项指标，即检出的相关文献与全部相关文献的百分比。普遍表示为：查全率=（检索出的相关信息量/系统中的相关信息总量）x100%。

按照混淆矩阵的定义，`查全率`可表示为：

$$查全率(Recall) = \frac{检索出的相关信息量}{系统中的相关信息总量} = \frac{做对的Positive√√}{做对的Positive√√+做错的Negative√×} = \frac{TruePositive}{TruePositive+FalseNegative} = \frac{TP}{TP+FN}$$

$$查全率(Recall) = \frac{TP}{TP+FN}$$

系统中的相关信息总量，可以理解为`真实类别为正例`

关系：查准率与查全率为互逆相关性，当查全率超过一定值时，若想再提高查全率就必然降低查准率。

历史：查准率是衡量某一类文献检索系统的信号噪声比的一种指标。它的数值等于w/m，式中w是用户鉴别检出的m篇文献时，认为实际对口径的文献篇数。这一指标最初是1956年由J.W.佩里、A.肯特等人提出的。F.W.兰开斯特1979年在《情报检索系统──特性、试验与评价》(第二版)一书中将某一系统所拥有的文献总篇数表述为a+b+c+d之和，并列出2×2表格。


# F1 score

每次分析都需要同时给出`R`值和`P`值，实在是太麻烦了，可以算下平均数，代表一下，

`f1-score`是`R`和`P`的调和平均数

可以先来复习一下算数平均数、几何平均数、调和平均数的计算方法：

$$M(算数平均数) = \frac{X_{1} \times W_{1} + X_{2} \times W_{2} + \dots + X_{k} \times W_{k}}{W_{1} + W_{2} + \dots + W_{k}}$$

$$G(几何平均数) = \sqrt[\sum_{i=1}^n,f_{i}]{\prod_{i = 1}^nX^{f_{i}}_{i}} = \sqrt[\sum_{i=1}^n,f_{i}]{X^{f_{1}}_{1},X^{f_{2}}_{2},X^{f_{3}}_{4},\dots,X^{f_{n}}_{n}}$$

$$H(调和平均数) = \frac{1}{\frac{1}{n}\sum_{i=1}^n\frac{1}{x_{i}}} = \frac{n}{\sum_{i=1}^n\frac{1}{x_{i}}}$$

那么我们就可以应用`H()`来推导一下`f1-score`：

$f1 = H(R, P) = \frac{2}{\frac{1}{R} + \frac{1}{P}} = \frac{2}{\frac{P}{PR}+\frac{R}{PR}} = \frac{2}{\frac{P+R}{PR}} = \frac{2PR}{P+R}$

这时，我们推出了常见的f1计算方式：`F1 = 2 * (precision * recall) / (precision + recall)`

又因为：

$$查准率(Precision) = \frac{TP}{TP+FP}$$

$$查全率(Recall) = \frac{TP}{TP+FN}$$

将上式带入，

$f1 = \frac{2PR}{P+R} = \frac{2\frac{TP}{TP+FP} \times \frac{TP}{TP+FN}}{\frac{TP}{TP+FP}+\frac{TP}{TP+FN}} = \frac{2\frac{TP^{2}}{(TP+FP)(TP+FN)}}{\frac{TP(TP+FN)}{(TP+FP)(TP+FN)}+\frac{TP(TP+FP)}{(TP+FN)(TP+FP)}} = \frac{2\frac{TP^{2}}{(TP+FP)(TP+FN)}}{\frac{TP(TP+FN)+TP(TP+FP)}{(TP+FP)(TP+FN)}} = \frac{2TP^{2}}{TP(TP+FN+TP+FP)} = \frac{2TP}{2TP+FN+FP} $

这样就得到了常见的`f1_score=(2*TP/(2*TP+FN+FP))`的计算方式

```python
from sklearn import metrics
y_pred = [0,1,0,1]
y_true = [0,1,1,1]
print('F1-score:',metrics.f1_score(y_true, y_pred))
```

# 使用`sklearn.metrics`

在对任务计算R值、P值、F值时，

* 使用手动统计计算混淆矩阵，需要清楚TP、FP、FN、TN的定义
* 使用sklearn.metrics时，需要为二分类和多分类传不同的参数

下面是一个二分类任务的示例代码


```python
from sklearn.metrics import f1_score, precision_score, accuracy_score, recall_score, confusion_matrix
print(CORRECT, (LINENO-1))
print(CORRECT/(LINENO-1))
print("f1_score(2*TP/(2*TP+FN+FP)) =", f1_score(y_true, y_pred, average='binary', pos_label="检出"))
print("accuracy_score((TP+TN)/(TP+FN+FP+TN)) =", accuracy_score(y_true, y_pred))
print("precision_score((TP)/(TP+FP)) =", precision_score(y_true, y_pred, average='macro'))
print("recall_score((TP)/(TP+FN)) =", recall_score(y_true, y_pred, average='macro'))
cm = confusion_matrix(y_true, y_pred)
print(f"confusion_matrix:\nTP={cm[1][1]}\tFN={cm[1][0]}\nFP={cm[0][1]}\tTN={cm[0][0]}")
```



# 参考资料

https://scikit-learn.org/stable/modules/classes.html#module-sklearn.metrics

https://blog.csdn.net/youif/article/details/105467922
