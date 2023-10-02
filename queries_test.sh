echo '{"name": "Notro Bert","email": "notrobert@gmail.com","user_id" : "notrober2023"}' | http localhost:8080/users

echo '{"user_id": "law","ticker": "AMZN","quantity" : 30,"limit_order" : true,"limit_price" : 200.00}  ' | http localhost:8080/order/buy

echo '{"user_id": "law","ticker": "AMZN","quantity" : 30,"limit_order" : true,"limit_price" : 100.00}  ' | http localhost:8080/order/sell
