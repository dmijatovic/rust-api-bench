
docker-compose up -d
echo "Sleep 10 sec"
sleep 10
cd ../loadtest
npm run test-actix-dp
cd ../actix-dp
docker-compose down --volumes