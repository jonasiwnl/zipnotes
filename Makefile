dev:
	trunk serve

build:
	trunk build --release

image:
	docker build -t zipnotes-img .

.PHONY: container

container:
	docker run -p 8080:8080 --rm --name zipnotes zipnotes-img
