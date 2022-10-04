
y_true = [1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4]
y_pred = [1, 1, 1, 0, 0, 2, 2, 3, 3, 3, 4, 3, 4, 3]


print(f1_score(y_true, y_pred, labels=[1,2,3,4], average='micro'))


from sklearn.metrics import confusion_matrix
y_pred = [0,1,0,1]
y_true = [0,1,1,1]
confusion_matrix(y_true, y_pred, labels=[1, 0])


from sklearn import metrics
y_pred = [0,1,0,1]
y_true = [0,1,1,1]
print('Precision', metrics.precision_score(y_true,y_pred))


from sklearn import metrics
y_pred = [0,1,0,1]
y_true = [0,1,1,1]
print('F1-score:',metrics.f1_score(y_true, y_pred))


from sklearn.metrics import f1_score, precision_score, accuracy_score, recall_score, confusion_matrix
print(CORRECT, (LINENO-1))
print(CORRECT/(LINENO-1))
print("f1_score(2*TP/(2*TP+FN+FP)) =", f1_score(y_true, y_pred, average='binary', pos_label="检出"))
print("accuracy_score((TP+TN)/(TP+FN+FP+TN)) =", accuracy_score(y_true, y_pred))
print("precision_score((TP)/(TP+FP)) =", precision_score(y_true, y_pred, average='macro'))
print("recall_score((TP)/(TP+FN)) =", recall_score(y_true, y_pred, average='macro'))
cm = confusion_matrix(y_true, y_pred)
print(f"confusion_matrix:\nTP={cm[1][1]}\tFN={cm[1][0]}\nFP={cm[0][1]}\tTN={cm[0][0]}")