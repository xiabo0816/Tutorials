build:
	rm -f test.o
	gcc -c test.cpp -o test.o
	gcc -shared -o libgotest.dll test.o
# if mac
#	gcc -shared -o libgotest.so test.o
	rm -f test.o