##
mysql-start: mysql-stop
	docker run --rm -d \
		-v $(shell pwd)/asset/my.cnf:/etc/my.cnf \
		-e MYSQL_ROOT_PASSWORD='mysql-password' \
		-p 33061:3306 \
		--hostname mysql \
		--name 'mysql' mysql:5.6

mysql-bash:
	docker exec -it 'mysql' bash

mysql-shell:
	docker exec -it 'mysql' sh -c \
		'MYSQL_PWD=mysql-password mysql'

mysql-dump:
	docker exec -it 'mysql' sh -c \
		'MYSQL_PWD=mysql-password mysqldump --databases TEST --routines --single-transaction --master-data'

mysql-stop:
	-docker rm -f 'mysql'

##
ftp-start: ftp-stop
	docker run --rm -d -v $(shell pwd):/home/vsftpd/myuser:ro \
		-p 20:20 -p 21:21 -p 21100-21110:21100-21110 \
		-e FTP_USER=myuser -e FTP_PASS=mypass \
		-e PASV_ADDRESS=127.0.0.1 -e PASV_MIN_PORT=21100 -e PASV_MAX_PORT=21110 \
		--name vsftpd fauria/vsftpd

ftp-stop:
	-docker rm -f 'vsftpd'

