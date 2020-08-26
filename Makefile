CC = cargo
BROWSER = firefox


read_doc:
	$(CC) doc
	$(BROWSER) file:target/doc/rustlativity/index.html
