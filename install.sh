#!/bin/bash

read -p 'Do you want your service to be accessible from internet?[Y/n]' net

if [ "$net" = "Y" ] || [ "$net" = "y" ]
then
	echo "BIND_IP=0.0.0.0" >> .env
elif [ "$net" = "N" ] || [ "$net" = "n" ]
then
	echo "BIND_IP=127.0.0.1" >> .env
else
	exit
fi

until [[ "$port" -lt 65535 && "$port" -gt 0 ]]
do
	read -p 'Which port you want to deploy this service:' port
done
echo "BIND_PORT=$port" >> .env

read -p 'Your Google OAuth Client ID:' id
echo "GOOGLE_OAUTH_CLIENT_ID=$id" >> .env

read -p "Your Mysql/MariaDB server's ip:" dbip
read -p "Your Mysql/MariaDB server's port:" dbport
read -p "Your Mysql/MariaDB server's username:" dbusername
read -p "Your Mysql/MariaDB server's password:" dbpw
read -p "Your database's name:" dbname
echo "DATABASE_URL=mysql://$dbusername:$dbpw@$dbip:$dbport/$dbname" >> .env

sudo docker compose up
