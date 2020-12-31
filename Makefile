ftp-start: ftp-stop
	docker run --rm -d -v $(shell pwd):/home/vsftpd/myuser:ro \
		-p 20:20 -p 21:21 -p 21100-21110:21100-21110 \
		-e FTP_USER=myuser -e FTP_PASS=mypass \
		-e PASV_ADDRESS=127.0.0.1 -e PASV_MIN_PORT=21100 -e PASV_MAX_PORT=21110 \
		--name vsftpd fauria/vsftpd

ftp-stop:
	-docker rm -f 'vsftpd'

mysql-start: mysql-stop
	docker run --rm -d \
		-e MYSQL_ROOT_PASSWORD=mysql-password \
		-p 33061:3306 \
		--name 'mysql-5.6' mysql:5.6

	docker run --rm -d \
		-e MYSQL_ROOT_PASSWORD=mysql-password \
		-p 33062:3306 \
		--name 'mysql-5.7' mysql:5.7

	docker run --rm -d \
		-e MYSQL_ROOT_PASSWORD=mysql-password \
		-p 33063:3306 \
		--name 'mysql-8.0' mysql:8.0

mysql-5.6-shell:
	docker run --rm -it --network host \
		--name 'mysql-5.6-shell' mysql:5.6 \
			mysql -h127.0.0.1 -P33061 -uroot -p'mysql-password'

mysql-5.7-shell:
	docker run --rm -it --network host \
		--name 'mysql-5.7-shell' mysql:5.7 \
			mysql -h127.0.0.1 -P33062 -uroot -p'mysql-password'

mysql-8.0-shell:
	docker run --rm -it --network host \
		--name 'mysql-8.0-shell' mysql:8.0 \
			mysql -h127.0.0.1 -P33063 -uroot -p'mysql-password'

mysql-stop:
	-docker rm -f 'mysql-5.6'
	-docker rm -f 'mysql-5.7'
	-docker rm -f 'mysql-8.0'
