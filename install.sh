#!/bin/bash

read -p 'Do you want your service to be accessible from internet?[Y/n]' net

if [ "$net" = "Y" ] || [ "$net" = "y" ]
then 
	echo "BIND_PORT=0.0.0.0" >> .env.docker
elif [ "$net" = "N" ] || [ "$net" = "n" ]
then
	echo "BIND_PORT=127.0.0.1" >> .env.docker
else
	exit
fi

until [[ "$port" -lt 65535 && "$port" -gt 0 ]] 
do
	read -p 'Which port you want to deploy this service:' port
done
echo "BIND_PORT=$port" >> .env.docker

read -p 'Your Google OAuth ID:' id
echo "GOOGLE_OAUTH_ID=$id" >> .env.docker

read -p 'Your Google OAuth Key:' key
echo "GOOGLE_OAUTH_KEY=$key" >>.env.docker

sudo docker compose up
