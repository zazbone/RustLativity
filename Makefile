CC = cargo
BROWSER = firefox


read_doc:
	$(CC) doc
	$(BROWSER) file:target/doc/rustlativity/index.html

count:
	git ls-files *.rs | xargs wc -l

git_log:
	git log --graph --full-history --all --color \
        --pretty=format:"%x1b[31m%h%x09%x1b[32m%d%x1b[0m%x20%s"
