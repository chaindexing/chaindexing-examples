db.start:
	docker-compose up

db.setup: 
	cd ark-db && diesel setup && cd .. 

db.setup.after_sleep:
	sleep 40 && make db.setup &

db.stop: 
	docker-compose down

db.drop:
	rm -rf ./postgres-data

db.reset: db.stop db.drop db.start

nfts.run: 
	cd rust && cargo run -p nfts

uniswap.run:
	cd rust && cargo run -p uniswap

