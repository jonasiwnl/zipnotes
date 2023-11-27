dev:
	trunk serve

build:
	trunk build --release

dbuild:
	docker build -t zipnotes-img .

drun:
	docker run -p 8080:8080 --rm --name zipnotes zipnotes-img
