start-ftp:
	docker run --rm -d -v $(shell pwd):/home/vsftpd/myuser:ro \
	-p 20:20 -p 21:21 -p 21100-21110:21100-21110 \
	-e FTP_USER=myuser -e FTP_PASS=mypass \
	-e PASV_ADDRESS=127.0.0.1 -e PASV_MIN_PORT=21100 -e PASV_MAX_PORT=21110 \
	--name vsftpd fauria/vsftpd
