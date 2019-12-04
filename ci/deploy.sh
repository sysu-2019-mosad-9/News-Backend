openssl aes-256-cbc -K $encrypted_4006a652c8d1_key -iv $encrypted_4006a652c8d1_iv -in ci/deploy_rsa.enc -out /tmp/deploy_rsa -d
eval "$(ssh-agent -s)"
chmod 600 /tmp/deploy_rsa
ssh-add /tmp/deploy_rsa
ssh $SERVER_USERNAME@$SERVER_IP 'cd /root/newsback && \
  git pull origin master && \
  docker-compose down && \
  docker-compose pull && \
  docker-compose up -d'
