db.start:
	docker-compose up

db.stop: 
	docker-compose down

db.drop:
	rm -rf ./postgres-data

db.reset: db.stop db.drop db.start

rust.nfts.run: 
	cd rust && cargo run -p nfts

rust.uniswap.run:
	cd rust && cargo run -p uniswap

