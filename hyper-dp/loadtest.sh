
docker-compose up -d
echo "Sleep 10 sec"
sleep 10
cd ../loadtest
npm run test-hyper-dp
cd ../hyper-dp
docker-compose down --volumes